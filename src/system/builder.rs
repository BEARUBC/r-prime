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

pub struct SystemBuilder<M, R, A>(Vec<ComponentBuilder<M, R, A>>)
where
    M: 'static + Send,
    R: 'static,
    A: 'static;

impl<'a, M, R, A> SystemBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    pub fn with_capacity(capacity: usize) -> Self { Self(Vec::with_capacity(capacity)) }

    pub fn push(&mut self, component_builder: ComponentBuilder<M, R, A>) {
        self.0.push(component_builder)
    }
}

impl<'a, M, R, A> Builder<System<M, R, A>, SystemError> for SystemBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    fn build(self) -> SystemResult<System<M, R, A>> { Ok(System::new(self.0)) }
}

impl<'a, M, R, A> Deref for SystemBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    type Target = Vec<ComponentBuilder<M, R, A>>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<'a, M, R, A> DerefMut for SystemBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<'a, M, R, A> AsRef<Vec<ComponentBuilder<M, R, A>>> for SystemBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    fn as_ref(&self) -> &Vec<ComponentBuilder<M, R, A>> { &self.0 }
}

impl<'a, M, R, A> AsMut<Vec<ComponentBuilder<M, R, A>>> for SystemBuilder<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    fn as_mut(&mut self) -> &mut Vec<ComponentBuilder<M, R, A>> { &mut self.0 }
}
