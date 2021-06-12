// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::collections::BTreeMap;

use tokio::sync::mpsc::UnboundedSender;

use crate::{
    builder::Builder,
    port::{
        error::PortError,
        request::Request,
        Port,
        PortResult,
    },
    prelude::ComponentBuilder,
};

pub struct PortBuilder<PSH> {
    sender: UnboundedSender<Request<PSH>>,
    others: BTreeMap<String, UnboundedSender<Request<PSH>>>,
}

impl<PSH> PortBuilder<PSH>
where
    PSH: 'static + Send,
{
    pub fn new(sender: UnboundedSender<Request<PSH>>) -> Self {
        Self {
            sender,
            others: BTreeMap::new(),
        }
    }

    pub fn sender(&self) -> UnboundedSender<Request<PSH>> { self.sender.clone() }

    pub fn add_component<PSR>(&mut self, component_builder: &mut ComponentBuilder<PSH, PSR>) {
        let name = component_builder.name().clone();
        let sender = component_builder.port_builder().sender();

        self.others.insert(name, sender);
    }
}

impl<PSH> Builder<Port<PSH>, PortError> for PortBuilder<PSH> {
    fn build(self) -> PortResult<Port<PSH>> { Ok(Port::new(self.sender, self.others)) }
}
