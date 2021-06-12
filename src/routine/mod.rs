// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

pub mod builder;

use std::sync::Arc;

use crate::{
    builder::Builder,
    job::Job,
    routine::builder::RoutineBuilder,
};

pub struct Routine<PSH> {
    jobs: Box<[Arc<Job<PSH>>]>,
    current_index: usize,
    max_capacity: usize,
}

impl<PSH> Routine<PSH> {
    pub(crate) fn new(jobs: Vec<Arc<Job<PSH>>>) -> Self {
        let max_capacity = jobs.len();

        Self {
            jobs: jobs.into_boxed_slice(),
            current_index: 0usize,
            max_capacity,
        }
    }
}

impl<PSH> Iterator for Routine<PSH> {
    type Item = Arc<Job<PSH>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_capacity == 0usize {
            None
        } else {
            Some(
                self.jobs
                    .get(self.current_index)
                    .unwrap() // unwrap should never fail
                    .clone(),
            )
        }
        .map(|item| {
            self.current_index = if self.current_index == (self.max_capacity - 1usize) {
                0usize
            } else {
                self.current_index + 1usize
            };

            item
        })
    }
}

impl<PSH> From<RoutineBuilder<PSH>> for Routine<PSH> {
    fn from(routine_builder: RoutineBuilder<PSH>) -> Self {
        routine_builder.build().expect("unable to build routine")
    }
}
