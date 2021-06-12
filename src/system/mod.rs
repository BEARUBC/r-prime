// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

pub mod builder;
pub mod error;

use std::fmt::{
    Display,
    Formatter,
    Result as StdFmtResult,
};

use tokio::runtime::Builder as TokioBuilder;

use crate::{
    builder::Builder,
    component::{
        builder::ComponentBuilder,
        Component,
    },
    system::{
        builder::SystemBuilder,
        error::SystemError,
    },
};

pub type SystemResult<T> = Result<T, SystemError>;

pub struct System<PSH, PSR>(Box<[Component<PSH, PSR>]>)
where
    PSH: 'static + Send,
    PSR: 'static;

impl<PSH, PSR> System<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    pub(crate) fn new(component_builders: Vec<ComponentBuilder<PSH, PSR>>) -> Self {
        Self(
            component_builders
                .into_iter()
                .map(|component_builder| component_builder.build().unwrap())
                .collect(),
        )
    }

    pub fn run(mut self) -> ! {
        self.0.iter_mut().for_each(|component| {
            component.start().unwrap();
        });

        TokioBuilder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                loop {
                    for component in self.0.iter() {
                        component.port().run_next_job().expect("unable to run job");
                    }
                }
            });

        panic!()
    }
}

impl<PSH, PSR> From<SystemBuilder<PSH, PSR>> for System<PSH, PSR>
where
    PSH: 'static + Send,
    PSR: 'static,
{
    fn from(system_builder: SystemBuilder<PSH, PSR>) -> Self { system_builder.build().unwrap() }
}

impl<PSH, PSR> Display for System<PSH, PSR>
where
PSH: 'static + Send,
PSR: 'static, {
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult { todo!() }
}
