// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

pub mod builder;
pub mod error;
pub mod state_store;

use std::{
    any::Any,
    future::Future,
    pin::Pin,
    sync::Arc,
    thread::{
        self,
        JoinHandle,
    },
    time::Duration,
};

use tokio::{
    runtime::Builder as TokioBuilder,
    sync::mpsc::UnboundedReceiver,
    task::{
        spawn_local,
        LocalSet,
    },
    time::sleep,
};

use crate::{
    builder::Builder,
    component::{
        error::ComponentError,
        state_store::StateStore,
    },
    job::Job,
    port::{
        builder::PortBuilder,
        request::Request,
        Port,
    },
    routine::{
        builder::RoutineBuilder,
        Routine,
    },
};

pub type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

pub struct Component<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    // persistent data
    id: Identifier,
    name: String,
    port: Port<PSH>,
    state_store: Box<dyn StateStore>,

    // consumed data
    consumable_data: Option<(
        Routine<PSH>,
        UnboundedReceiver<Request<PSH>>,
        Arc<dyn Fn(Port<PSH>, PSH) -> Pin<Box<dyn Future<Output = PSR>>> + Send + Sync>,
    )>,
}

impl<PSH, PSR> Component<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    pub(crate) fn new(
        id: Identifier,
        name: String,
        port_builder: PortBuilder<PSH>,
        state_store: Box<dyn StateStore>,
        routine_builder: RoutineBuilder<PSH>,
        recver: UnboundedReceiver<Request<PSH>>,
        handler: Arc<dyn Fn(Port<PSH>, PSH) -> Pin<Box<dyn Future<Output = PSR>>> + Send + Sync>,
    ) -> Self {
        Self {
            id,
            name,
            port: port_builder.into(),
            state_store,
            consumable_data: Some((routine_builder.build().unwrap(), recver, handler)),
        }
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn name(&self) -> &String { &self.name }

    pub fn port(&self) -> &Port<PSH> { &self.port }

    pub fn start(&mut self) -> ComponentResult<JoinHandle<()>> {
        if self.consumable_data.is_some() {
            Ok((self.port.clone(), self.consumable_data.take().unwrap()))
        } else {
            Err(ComponentError::AlreadyInitializedComponent)
        }
        .map(|(port, (mut routine, mut recver, handler))| {
            thread::spawn(move || {
                let local = LocalSet::new();

                local.spawn_local(async move {
                    while let Some(new_task) = recver.recv().await {
                        use Request::*;

                        match new_task {
                            HandleMessage(message) => {
                                // spawn_local(handler(port.clone(), message));
                                let handler_copy = handler.clone();
                                let port_clone = port.clone();

                                spawn_local(async move {
                                    #[allow(unused)]
                                    let response = handler_copy(port_clone, message).await;
                                });
                            },
                            RunJob => match routine.next() {
                                Some(job) => {
                                    use Job::*;

                                    match job.as_ref() {
                                        Spacer(spacer) => {
                                            sleep(Duration::from_millis(*spacer)).await
                                        },
                                        Function(lambda) => {
                                            spawn_local(lambda(port.clone()));
                                        },
                                    };
                                },
                                _ => (),
                            },
                        };
                    }
                });

                TokioBuilder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("unable to construct runtime")
                    .block_on(local);
            })
        })
    }

    // CSS stands for ConcreteStateStore
    pub fn state_store<CSS>(&mut self) -> &mut CSS
    where
        CSS: StateStore,
    {
        let concrete_state_store = &mut self.state_store as &mut dyn Any;

        match concrete_state_store.downcast_mut::<CSS>() {
            Some(css) => css,
            _ => panic!(),
        }
    }
}
