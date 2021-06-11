use std::{
    future::Future,
    pin::Pin,
};

use crate::port::Port;

pub enum Job<PSH> {
    Spacer(u64),
    Function(Box<dyn Fn(Port<PSH>) -> Pin<Box<dyn Future<Output = ()>>>>),
}

impl<PSH> Job<PSH>
where
    PSH: 'static + Send,
{
    pub fn from_spacer(amount: u64) -> Self { Self::Spacer(amount) }

    pub fn from_function<JRFut>(f: fn(Port<PSH>) -> JRFut) -> Self
    where
        JRFut: 'static + Future<Output = ()>,
    {
        Self::Function(Box::new(move |contacts| Box::pin(f(contacts))))
    }
}

impl<PSH> Clone for Job<PSH> {
    fn clone(&self) -> Self {
        use Job::*;

        match self {
            Spacer(amount) => Self::Spacer(*amount),
            _ => panic!(),
        }
    }
}

unsafe impl<PSH> Send for Job<PSH> {}

unsafe impl<PSH> Sync for Job<PSH> {}
