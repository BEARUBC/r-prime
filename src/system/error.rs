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
