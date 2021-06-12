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

#[derive(Debug, Clone)]
pub enum PortError {
    SenderDoesNotExist(String),
    SendError,
}

impl Display for PortError {
    fn fmt(&self, f: &mut Formatter) -> StdFmtResult {
        use PortError::*;

        match self {
            SenderDoesNotExist(name) => write!(
                f,
                "Port belonging to component with name {} was not found",
                name
            ),
            SendError => write!(f, "Unable to send to this port"),
        }
    }
}

impl<T> From<SendError<T>> for PortError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
