// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::{
    fmt::{
        Debug,
        Display,
        Formatter,
        Result,
    },
    future::Future,
    pin::Pin,
};

use crate::port::Port;

pub enum Job<PSH>
where
    PSH: 'static + Send,
{
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

impl<PSH> Clone for Job<PSH>
where
    PSH: 'static + Send,
{
    fn clone(&self) -> Self {
        use Job::*;

        match self {
            Spacer(amount) => Self::Spacer(*amount),
            _ => panic!(),
        }
    }
}

impl<PSH> Display for Job<PSH>
where
    PSH: 'static + Send,
{
    fn fmt(&self, _: &mut Formatter) -> Result { todo!() }
}

impl<PSH> Debug for Job<PSH>
where
    PSH: 'static + Send,
{
    fn fmt(&self, _: &mut Formatter) -> Result { todo!() }
}

unsafe impl<PSH> Send for Job<PSH> where PSH: 'static + Send {}

unsafe impl<PSH> Sync for Job<PSH> where PSH: 'static + Send {}
