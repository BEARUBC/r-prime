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

pub struct RoutineBuilder<PSH>(Vec<Arc<Job<PSH>>>);

impl<PSH> RoutineBuilder<PSH> {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn push(&mut self, job: Job<PSH>) { self.0.push(Arc::new(job)) }
}

impl<PSH> Builder<Routine<PSH>, ()> for RoutineBuilder<PSH> {
    fn build(self) -> Result<Routine<PSH>, ()> { Ok(Routine::new(self.0)) }
}

impl<PSH> Deref for RoutineBuilder<PSH> {
    type Target = Vec<Arc<Job<PSH>>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<PSH> DerefMut for RoutineBuilder<PSH> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<PSH> AsRef<Vec<Arc<Job<PSH>>>> for RoutineBuilder<PSH> {
    fn as_ref(&self) -> &Vec<Arc<Job<PSH>>> { &self.0 }
}

impl<PSH> AsMut<Vec<Arc<Job<PSH>>>> for RoutineBuilder<PSH> {
    fn as_mut(&mut self) -> &mut Vec<Arc<Job<PSH>>> { &mut self.0 }
}
