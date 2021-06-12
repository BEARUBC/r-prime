// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::{
    ops::{
        Deref,
        DerefMut,
    },
    sync::Arc,
};

use crate::{
    builder::Builder,
    job::Job,
    routine::Routine,
};

pub struct RoutineBuilder<PSH> {
    jobs: Vec<Arc<Job<PSH>>>,
    start_index: usize,
}

impl<PSH> RoutineBuilder<PSH> {
    pub fn new() -> Self {
        Self {
            jobs: Vec::new(),
            start_index: 0usize,
        }
    }

    pub fn push(&mut self, job: Job<PSH>) { self.jobs.push(Arc::new(job)) }

    pub fn set_start_index(&mut self, start_index: usize) { self.start_index = start_index }
}

impl<PSH> Builder<Routine<PSH>, ()> for RoutineBuilder<PSH> {
    fn build(self) -> Result<Routine<PSH>, ()> { Ok(Routine::new(self.jobs, self.start_index)) }
}

impl<PSH> Deref for RoutineBuilder<PSH> {
    type Target = Vec<Arc<Job<PSH>>>;

    fn deref(&self) -> &Self::Target { &self.jobs }
}

impl<PSH> DerefMut for RoutineBuilder<PSH> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.jobs }
}

impl<PSH> AsRef<Vec<Arc<Job<PSH>>>> for RoutineBuilder<PSH> {
    fn as_ref(&self) -> &Vec<Arc<Job<PSH>>> { &self.jobs }
}

impl<PSH> AsMut<Vec<Arc<Job<PSH>>>> for RoutineBuilder<PSH> {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<PSH>>> { &mut self.jobs }
}
