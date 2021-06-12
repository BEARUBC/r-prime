// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::fmt::{
    Display,
    Formatter,
    Result as StdFmtResult,
};

#[derive(Debug)]
pub enum Request<PSH>
where
    PSH: 'static + Send,
{
    HandleMessage(PSH),
    RunJob,
}

impl<PSH> Display for Request<PSH>
where
    PSH: 'static + Send,
{
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult { todo!() }
}
