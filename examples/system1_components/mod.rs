use tokio::sync::mpsc::error::SendError;

use crate::system1_components::fa::FaMsgs;

pub mod emg;
pub mod fa;

#[derive(Debug)]
pub enum System1Error {
    SendError,
}

impl From<SendError<FaMsgs>> for System1Error {
    fn from(_: SendError<FaMsgs>) -> Self {
        Self::SendError
    }
}
