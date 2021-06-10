use std::fmt::{
    Display,
    Formatter,
    Result,
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
    fn fmt(&self, _: &mut Formatter) -> Result {
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
