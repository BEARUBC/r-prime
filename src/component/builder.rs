// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::{
    borrow::Cow,
    future::Future,
    pin::Pin,
    sync::Arc,
    fmt::{
        Display,
        Debug,
        Formatter,
        Result as StdFmtResult,
    },
};

use tokio::sync::mpsc::{
    unbounded_channel,
    UnboundedReceiver,
};

use crate::{
    builder::Builder,
    component::{
        error::ComponentError,
        state_store::StateStore,
        Component,
        ComponentResult,
        Identifier,
    },
    port::{
        builder::PortBuilder,
        request::Request,
    },
    prelude::Port,
    routine::builder::RoutineBuilder,
    utils::get_new_id,
};

pub struct ComponentBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    id: Identifier,
    name: String,
    port_builder: PortBuilder<PSH>,
    state_store: Box<dyn StateStore>,
    routine_builder: RoutineBuilder<PSH>,
    recver: UnboundedReceiver<Request<PSH>>,
    handler: Arc<dyn Fn(Port<PSH>, PSH) -> Pin<Box<dyn Future<Output = PSR>>> + Send + Sync>,
}

impl<PSH, PSR> ComponentBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    pub fn new<'a, N, PSFut>(
        name: N,
        state_store: Box<dyn StateStore>,
        handler: fn(Port<PSH>, PSH) -> PSFut,
    ) -> ComponentResult<Self>
    where
        N: Into<Cow<'a, str>>,
        PSFut: 'static + Future<Output = PSR>,
    {
        get_new_id()
            .map(|id| (id, unbounded_channel()))
            .map(|(id, (sender, recver))| Self {
                id,
                name: name.into().into_owned(),
                port_builder: PortBuilder::new(sender),
                state_store,
                routine_builder: RoutineBuilder::new(),
                recver,
                handler: Arc::new(move |port, message| Box::pin(handler(port, message))),
            })
            .map_err(ComponentError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn name(&self) -> &String { &self.name }

    pub fn port_builder(&mut self) -> &mut PortBuilder<PSH> { &mut self.port_builder }

    pub fn routine_builder(&mut self) -> &mut RoutineBuilder<PSH> { &mut self.routine_builder }
}

impl<PSH, PSR> Builder<Component<PSH, PSR>, ComponentError> for ComponentBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn build(self) -> ComponentResult<Component<PSH, PSR>> {
        Ok(Component::new(
            self.id,
            self.name,
            self.port_builder,
            self.state_store,
            self.routine_builder,
            self.recver,
            self.handler,
        ))
    }
}

impl<PSH, PSR> Display for ComponentBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult { todo!() }
}

impl<PSH, PSR> Debug for ComponentBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult { todo!() }
}
