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

use tokio::sync::mpsc::error::SendError;

use crate::{
    component::Identifier,
    utils::MutexError,
};

#[derive(Debug, Clone)]
pub enum ComponentError {
    AlreadyInitializedComponent,
    SendError,
    IdError,
    ContactDoesNotExist(Identifier),
}

impl Display for ComponentError {
    fn fmt(&self, _: &mut Formatter) -> StdFmtResult {
        use ComponentError::*;

        match self {
            AlreadyInitializedComponent => todo!(),
            SendError => todo!(),
            IdError => todo!(),
            ContactDoesNotExist(_) => todo!(),
        }
    }
}

impl<T> From<SendError<T>> for ComponentError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}

impl<'a> From<MutexError<'a>> for ComponentError {
    fn from(_: MutexError) -> Self { Self::IdError }
}
