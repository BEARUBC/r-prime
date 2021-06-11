use std::fmt::{
    Display,
    Formatter,
    Result,
};

use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Clone)]
pub enum PortError {
    SenderDoesNotExist(String),
    SendError,
    PortNotConsumable,
}

impl Display for PortError {
    fn fmt(&self, _: &mut Formatter) -> Result {
        use PortError::*;

        match self {
            SenderDoesNotExist(_) => todo!(),
            SendError => todo!(),
            PortNotConsumable => todo!(),
        }
    }
}

impl<T> From<SendError<T>> for PortError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
