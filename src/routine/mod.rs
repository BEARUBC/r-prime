pub mod builder;

use std::sync::Arc;

use crate::{
    builder::Builder,
    job::Job,
    routine::builder::RoutineBuilder,
};

pub struct Routine<M, R> {
    jobs: Box<[Arc<Job<M, R>>]>,
    current_index: usize,
    max_capacity: usize,
}

impl<M, R> Routine<M, R> {
    pub(crate) fn new(jobs: Vec<Arc<Job<M, R>>>) -> Self {
        let max_capacity = jobs.len();

        Self {
            jobs: jobs.into_boxed_slice(),
            current_index: 0usize,
            max_capacity,
        }
    }
}

impl<M, R> Iterator for Routine<M, R> {
    type Item = Arc<Job<M, R>>;

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

impl<M, R> From<RoutineBuilder<M, R>> for Routine<M, R> {
    fn from(routine_builder: RoutineBuilder<M, R>) -> Self {
        routine_builder.build().expect("unable to build routine")
    }
}
