use std::{
    borrow::Cow,
    future::Future,
    pin::Pin,
};

use tokio::sync::mpsc::{
    unbounded_channel,
    UnboundedReceiver,
    UnboundedSender,
};

use crate::{
    builder::Builder,
    component::{
        error::ComponentError,
        request::Request,
        Component,
        ComponentResult,
        Identifier,
    },
    contacts::{
        builder::ContactsBuilder,
        Contacts,
    },
    routine::builder::RoutineBuilder,
    utils::get_new_id,
};

pub struct ComponentBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    id: Identifier,
    name: String,
    sender: UnboundedSender<Request<M>>,
    recver: UnboundedReceiver<Request<M>>,
    routine_builder: RoutineBuilder<M, R>,
    contacts_builder: ContactsBuilder<M>,
    handler: Box<dyn Fn(Contacts<M>, M) -> Pin<Box<dyn Future<Output = A>>> + Send>,
}

impl<'a, M, R, A> ComponentBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    pub fn new<N, Fut>(
        name: N,
        routine_builder: RoutineBuilder<M, R>,
        handler: fn(Contacts<M>, M) -> Fut,
    ) -> ComponentResult<Self>
    where
        Fut: 'static + Future<Output = A>,
        N: Into<Cow<'a, str>>,
    {
        get_new_id()
            .map(|id| (id, unbounded_channel::<Request<M>>()))
            .map(|(id, (send, recv))| Self {
                id,
                name: name.into().into_owned(),
                sender: send,
                recver: recv,
                routine_builder,
                contacts_builder: ContactsBuilder::new(),
                handler: Box::new(move |contacts, message| Box::pin(handler(contacts, message))),
            })
            .map_err(ComponentError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn sender(&self) -> UnboundedSender<Request<M>> { self.sender.clone() }

    pub fn name(&self) -> &String { &self.name }

    pub fn add_component(&mut self, component_builder: &Self) {
        self.contacts_builder
            .add_sender(component_builder.name().clone(), component_builder.sender())
    }
}

impl<'a, M, R, A> Builder<Component<M, R, A>, ComponentError> for ComponentBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    fn build(self) -> ComponentResult<Component<M, R, A>> {
        Ok(Component::new(
            self.id,
            self.name,
            self.sender,
            self.recver,
            self.contacts_builder,
            self.routine_builder,
            self.handler,
        ))
    }
}
