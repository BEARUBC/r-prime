pub mod builder;
pub mod error;

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

pub struct System<M, R, A>(Box<[Component<M, R, A>]>)
where
    M: 'static + Send,
    R: 'static,
    A: 'static;

impl<M, R, A> System<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    pub(crate) fn new(component_builders: Vec<ComponentBuilder<M, R, A>>) -> Self {
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
                        component.run_next_job().expect("unable to run job");
                    }
                }
            });

        panic!()
    }
}

impl<M, R, A> From<SystemBuilder<M, R, A>> for System<M, R, A>
where
    M: 'static + Send,
    R: 'static,
    A: 'static,
{
    fn from(system_builder: SystemBuilder<M, R, A>) -> Self { system_builder.build().unwrap() }
}
