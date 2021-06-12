// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::ops::{
    Deref,
    DerefMut,
};

use crate::{
    builder::Builder,
    component::builder::ComponentBuilder,
    system::{
        error::SystemError,
        System,
        SystemResult,
    },
};

pub struct SystemBuilder<PSH, PSR>(Vec<ComponentBuilder<PSH, PSR>>)
where
    PSH: 'static + Send,
    PSR: 'static;

impl<PSH, PSR> SystemBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn push(&mut self, component_builder: ComponentBuilder<PSH, PSR>) {
        self.0.push(component_builder)
    }
}

impl<PSH, PSR> Builder<System<PSH, PSR>, SystemError> for SystemBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn build(self) -> SystemResult<System<PSH, PSR>> { Ok(System::new(self.0)) }
}

impl<PSH, PSR> Deref for SystemBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    type Target = Vec<ComponentBuilder<PSH, PSR>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<PSH, PSR> DerefMut for SystemBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<PSH, PSR> AsRef<Vec<ComponentBuilder<PSH, PSR>>> for SystemBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn as_ref(&self) -> &Vec<ComponentBuilder<PSH, PSR>> { &self.0 }
}

impl<PSH, PSR> AsMut<Vec<ComponentBuilder<PSH, PSR>>> for SystemBuilder<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn as_mut(&mut self) -> &mut Vec<ComponentBuilder<PSH, PSR>> { &mut self.0 }
}
