use std::fmt::{
    Display,
    Formatter,
    Result,
};

use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Clone)]
pub enum ContactsError {
    SenderDoesNotExist(String),
    SendError,
}

impl Display for ContactsError {
    fn fmt(&self, _: &mut Formatter) -> Result {
        use ContactsError::*;

        match self {
            SenderDoesNotExist(_) => todo!(),
            SendError => todo!(),
        }
    }
}

impl<T> From<SendError<T>> for ContactsError {
    fn from(_: SendError<T>) -> Self { Self::SendError }
}
