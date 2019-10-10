use crate::{ValuePrefix, Identifiable};
use std::fmt::{Formatter, Display};
use std::error;

pub trait ValueExtracting {
    type Value;

    fn get_value(&mut self) -> Option<Self::Value>;
}



pub struct PrefixedValue<V, I> {
    pub value: V,
    _make_constructor_private: (),
}

impl<V, I: Identifiable> PrefixedValue<V, I> {
    fn new(prefix: I, unprocessed: String) -> Result<Self, PrefixedValueError> {
        if unprocessed.contains(prefix.identifier()) {

        } else {
            Err(PrefixedValueError::new())
        }
    }
}

impl<V, I: Identifiable> ValueExtracting for PrefixedValue<V, I> {
    type Value = V;

    fn get_value(&mut self) -> Option<Self::Value> {

    }
}


#[derive(Debug)]
pub struct PrefixedValueError {
    v: String,
}

impl PrefixedValueError {
    pub(crate) fn new() -> Self {
        Self {
            v: "oh no!".to_string()
        }
    }
}

impl error::Error for PrefixedValueError {
    fn description(&self) -> &str { &self.v }
}

impl Display for PrefixedValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PrefixedValueError: {}", &self.v)
    }
}