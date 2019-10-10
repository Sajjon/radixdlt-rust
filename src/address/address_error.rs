use std::{error};
use std::fmt::*;

#[derive(Debug)]
pub struct AddressError {
    v: String,
}

impl AddressError {
    pub(crate) fn new() -> Self {
        Self {
            v: "oh no!".to_string()
        }
    }
}

impl error::Error for AddressError {
    fn description(&self) -> &str { &self.v }
}

impl Display for AddressError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "AddressError: {}", &self.v)
    }
}