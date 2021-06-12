// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

pub mod builder;
pub mod error;
pub mod request;

use std::{
    borrow::Cow,
    collections::BTreeMap,
    sync::Arc,
    fmt::{
        Display,
        Formatter,
        Result as StdFmtResult,
    }
};

use tokio::sync::mpsc::UnboundedSender;

use crate::{
    builder::Builder,
    port::{
        builder::PortBuilder,
        error::PortError,
        request::Request,
    },
};

pub type PortResult<T> = Result<T, PortError>;

#[derive(Debug)]
pub struct Port<PSH>
where
    PSH: 'static + Send, {
    sender: UnboundedSender<Request<PSH>>,
    others: Arc<BTreeMap<String, UnboundedSender<Request<PSH>>>>,
}

impl<PSH> Port<PSH>
where
    PSH: 'static + Send, {
    pub(crate) fn new(
        sender: UnboundedSender<Request<PSH>>,
        others: BTreeMap<String, UnboundedSender<Request<PSH>>>,
    ) -> Self {
        Self {
            sender,
            others: Arc::new(others),
        }
    }

    pub fn sender(&self) -> UnboundedSender<Request<PSH>> { self.sender.clone() }

    pub fn send<'a, NM>(&self, name: NM, message: PSH) -> PortResult<()>
    where
        NM: Into<Cow<'a, str>>,
    {
        let owned_name: String = name.into().into_owned();

        self.others
            .as_ref()
            .get(&owned_name)
            .ok_or(PortError::SenderDoesNotExist(owned_name))
            .and_then(|sender| {
                sender
                    .send(Request::HandleMessage(message))
                    .map_err(PortError::from)
            })
    }

    pub(crate) fn run_next_job(&self) -> PortResult<()> {
        self.sender.send(Request::RunJob).map_err(PortError::from)
    }
}

impl<PSH> Clone for Port<PSH>
where
    PSH: 'static + Send, {
    fn clone(&self) -> Self {
        Self {
            sender: self.sender.clone(),
            others: self.others.clone(),
        }
    }
}

impl<PSH> From<PortBuilder<PSH>> for Port<PSH>
where
    PSH: 'static + Send, {
    fn from(contacts_builder: PortBuilder<PSH>) -> Self {
        contacts_builder.build().expect("unable to build contacts")
    }
}

impl<PSH> Display for Port<PSH>
where
    PSH: 'static + Send,
{
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult { todo!() }
}
