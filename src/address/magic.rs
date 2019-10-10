
#[derive(Eq, PartialEq, Hash)]
pub struct Magic {
    pub value: i32,
    _make_constructor_private: ()
}

impl Magic {

    fn value_for_encoding_and_decoding(&self) -> i64 {
        return i64::from(self.value);
    }

    fn as_byte_array(&self) -> [u8; 8] {
        return self.value_for_encoding_and_decoding().to_le_bytes();
    }

    pub fn byte(&self) -> u8 {
        return self.as_byte_array()[0];
    }
}

/// Debug only
impl Magic {
    #[cfg(debug_assertions)]
    pub fn new(value: i32) -> Self {
        return Magic {
            value: value,
            _make_constructor_private: ()
        }
    }
}