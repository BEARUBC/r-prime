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

pub struct RoutineBuilder<M, R>(Vec<Arc<Job<M, R>>>);

impl<M, R> RoutineBuilder<M, R> {
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn push(&mut self, job: Job<M, R>) { self.0.push(Arc::new(job)) }
}

impl<M, R> Deref for RoutineBuilder<M, R> {
    type Target = Vec<Arc<Job<M, R>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<M, R> DerefMut for RoutineBuilder<M, R> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<M, R> AsRef<Vec<Arc<Job<M, R>>>> for RoutineBuilder<M, R> {
    fn as_ref(&self) -> &Vec<Arc<Job<M, R>>> { &self.0 }
}

impl<M, R> AsMut<Vec<Arc<Job<M, R>>>> for RoutineBuilder<M, R> {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<M, R>>> { &mut self.0 }
}

impl<M, R> Builder<Routine<M, R>, ()> for RoutineBuilder<M, R> {
    fn build(self) -> Result<Routine<M, R>, ()> { Ok(Routine::new(self.0)) }
}
