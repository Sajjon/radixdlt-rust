/// `EUID` is an abbreviation for `Extended Unique Identifier`, and holds a 128 bit int
pub struct EUID {
//    pub value: u128,
    pub bytes: [u8; 16],
}

impl EUID {
    pub fn from_data(data: [u8; 16]) -> Self {
        return EUID { bytes: data };
    }

    pub fn to_hex_string(&self) -> String {
        unimplemented!()
    }
}

