use std::{
    future::Future,
    pin::Pin,
};

use crate::contacts::Contacts;

pub enum Job<M, R> {
    Spacer(u64),
    Function(Box<dyn Fn(Contacts<M>) -> Pin<Box<dyn Future<Output = R>>>>),
}

impl<M, R> Job<M, R> {
    pub fn from_spacer(amount: u64) -> Self { Self::Spacer(amount) }

    pub fn from_function<Fut>(f: fn(Contacts<M>) -> Fut) -> Self
    where
        M: 'static,
        Fut: 'static + Future<Output = R>,
    {
        Self::Function(Box::new(move |contacts| Box::pin(f(contacts))))
    }
}

impl<M, R> Clone for Job<M, R> {
    fn clone(&self) -> Self {
        use Job::*;

        match self {
            Spacer(amount) => Self::Spacer(*amount),
            _ => panic!(),
        }
    }
}

unsafe impl<M, R> Send for Job<M, R> {}

unsafe impl<M, R> Sync for Job<M, R> {}
