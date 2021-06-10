pub mod builder;
pub mod error;
pub mod request;

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
    sync::mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
    task::{
        spawn_local,
        LocalSet,
    },
    time::sleep,
};

use crate::{
    component::{
        error::ComponentError,
        request::Request,
    },
    contacts::{
        builder::ContactsBuilder,
        Contacts,
    },
    job::Job,
    routine::{
        builder::RoutineBuilder,
        Routine,
    },
};

pub type Identifier = usize;
pub type ComponentResult<T> = Result<T, ComponentError>;

pub struct Component<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    id: Identifier,
    name: String,
    sender: UnboundedSender<Request<M>>,
    recver: Option<UnboundedReceiver<Request<M>>>,
    contacts: Option<Contacts<M>>,
    routine: Option<Routine<M, R>>,
    handler: Option<Box<dyn Fn(Contacts<M>, M) -> Pin<Box<dyn Future<Output = A>>> + Send>>,
}

impl<M, R, A> Component<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    pub(crate) fn new(
        id: Identifier,
        name: String,
        sender: UnboundedSender<Request<M>>,
        recver: UnboundedReceiver<Request<M>>,
        contacts_builder: ContactsBuilder<M>,
        routine_builder: RoutineBuilder<M, R>,
        handler: Box<dyn Fn(Contacts<M>, M) -> Pin<Box<dyn Future<Output = A>>> + Send>,
    ) -> Self {
        Self {
            id,
            name,
            sender,
            recver: Some(recver),
            contacts: Some(contacts_builder.into()),
            routine: Some(routine_builder.into()),
            handler: Some(handler),
        }
    }

    pub fn send(&self, message: M) -> ComponentResult<()> {
        self.sender
            .send(Request::HandleMessage(message))
            .map_err(ComponentError::from)
    }

    pub fn run_next_job(&self) -> ComponentResult<()> {
        self.sender
            .send(Request::RunJob)
            .map_err(ComponentError::from)
    }

    pub fn id(&self) -> Identifier { self.id }

    pub fn name(&self) -> &String { &self.name }

    pub fn start(&mut self) -> ComponentResult<JoinHandle<()>> {
        if self.recver.is_some()
            && self.contacts.is_some()
            && self.routine.is_some()
            && self.handler.is_some()
        {
            Ok((
                self.recver.take().unwrap(),
                self.contacts.take().unwrap(),
                self.routine.take().unwrap(),
                self.handler.take().unwrap(),
            ))
        } else {
            Err(ComponentError::AlreadyInitializedComponent)
        }
        .map(|(mut recv, contacts, mut routine, handler)| {
            thread::spawn(move || {
                let local = LocalSet::new();

                local.spawn_local(async move {
                    while let Some(new_task) = recv.recv().await {
                        use Request::*;

                        match new_task {
                            HandleMessage(msg) => {
                                spawn_local(handler(contacts.clone(), msg));
                            },
                            RunJob => match routine.next() {
                                Some(job) => {
                                    use Job::*;

                                    match job.as_ref() {
                                        Spacer(spacer) => {
                                            sleep(Duration::from_millis(*spacer)).await
                                        },
                                        Function(lambda) => {
                                            spawn_local(lambda(contacts.clone()));
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
