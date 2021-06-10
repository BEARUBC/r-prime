pub mod builder;
pub mod error;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    sync::Arc,
};

use tokio::sync::mpsc::UnboundedSender;

use crate::{
    builder::Builder,
    component::request::Request,
    contacts::{
        builder::ContactsBuilder,
        error::ContactsError,
    },
};

pub type ContactsResult<T> = Result<T, ContactsError>;

pub struct Contacts<M>(Arc<BTreeMap<String, UnboundedSender<Request<M>>>>);

impl<M> Contacts<M> {
    pub fn new(btreemap: BTreeMap<String, UnboundedSender<Request<M>>>) -> Self {
        Self(Arc::new(btreemap))
    }

    pub fn send<'a, N>(&self, name: N, msg: M) -> ContactsResult<()>
    where
        N: Into<Cow<'a, str>>,
    {
        let owned_name: String = name.into().into_owned();

        self.0
            .as_ref()
            .get(&owned_name)
            .ok_or(ContactsError::SenderDoesNotExist(owned_name))
            .and_then(|sender| {
                sender
                    .send(Request::HandleMessage(msg))
                    .map_err(ContactsError::from)
            })
    }
}

impl<M> Clone for Contacts<M> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<M> From<ContactsBuilder<M>> for Contacts<M> {
    fn from(contacts_builder: ContactsBuilder<M>) -> Self {
        contacts_builder.build().expect("unable to build contacts")
    }
}
