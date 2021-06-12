// Copyright 2021 UBC Bionics, Ltd.
//
// Licensed under the MIT license
// <LICENSE.md or https://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or
// distributed except according to those terms.

use std::fmt::{
    Display,
    Formatter,
    Result,
};

#[derive(Debug, Clone)]
pub enum SystemError {}

impl Display for SystemError {
    fn fmt(&self, _: &mut Formatter) -> Result { todo!() }
}
