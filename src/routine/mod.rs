// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

pub mod builder;

use std::{
    fmt::{
        Display,
        Formatter,
        Result as StdFmtResult,
    },
    sync::Arc,
};

use crate::{
    builder::Builder,
    job::Job,
    routine::builder::RoutineBuilder,
};

#[derive(Debug)]
pub struct Routine<PSH>
where
    PSH: 'static + Send,
{
    jobs: Box<[Arc<Job<PSH>>]>,
    start_index: usize,
    current_index: usize,
}

impl<PSH> Routine<PSH>
where
    PSH: 'static + Send,
{
    pub(crate) fn new(jobs: Vec<Arc<Job<PSH>>>, start_index: usize) -> Self {
        Self {
            jobs: jobs.into_boxed_slice(),
            start_index,
            current_index: 0usize,
        }
    }
}

impl<PSH> Iterator for Routine<PSH>
where
    PSH: 'static + Send,
{
    type Item = Arc<Job<PSH>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.jobs.get(self.current_index) {
            Some(job) => {
                if self.current_index == (self.jobs.len() - 1usize) {
                    self.current_index = self.start_index
                } else {
                    self.current_index += 1usize
                }

                Some(job.clone())
            },
            None => None,
        }
    }
}

impl<PSH> From<RoutineBuilder<PSH>> for Routine<PSH>
where
    PSH: 'static + Send,
{
    fn from(routine_builder: RoutineBuilder<PSH>) -> Self {
        routine_builder.build().expect("unable to build routine")
    }
}

impl<PSH> Display for Routine<PSH>
where
    PSH: 'static + Send,
{
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult { todo!() }
}
