pub mod builder;
pub mod error;

use std::{
    future::Future,
    pin::Pin,
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
    component::error::ComponentError,
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

    // consumed by the while loop
    routine: Option<Routine<PSH>>,
    recver: Option<UnboundedReceiver<Request<PSH>>>,
    handler: Option<Box<dyn Fn(Port<PSH>, PSH) -> Pin<Box<dyn Future<Output = PSR>>> + Send>>,
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
        routine_builder: RoutineBuilder<PSH>,
        recver: UnboundedReceiver<Request<PSH>>,
        handler: Box<dyn Fn(Port<PSH>, PSH) -> Pin<Box<dyn Future<Output = PSR>>> + Send>,
    ) -> Self {
        Self {
            id,
            name,
            port: port_builder.into(),
            routine: Some(routine_builder.build().unwrap()),
            recver: Some(recver),
            handler: Some(handler),
        }
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn name(&self) -> &String { &self.name }

    pub fn port(&self) -> &Port<PSH> { &self.port }

    pub fn start(&mut self) -> ComponentResult<JoinHandle<()>> {
        if self.routine.is_some() {
            // let (recver, handler) =
            // self.port.partially_consume_data().unwrap();
            Ok((
                self.port.clone(),
                self.routine.take().unwrap(),
                self.recver.take().unwrap(),
                self.handler.take().unwrap(),
            ))
        } else {
            Err(ComponentError::AlreadyInitializedComponent)
        }
        .map(|(port, mut routine, mut recver, handler)| {
            thread::spawn(move || {
                let local = LocalSet::new();

                local.spawn_local(async move {
                    while let Some(new_task) = recver.recv().await {
                        use Request::*;

                        match new_task {
                            HandleMessage(msg) => {
                                spawn_local(handler(port.clone(), msg));
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
}
