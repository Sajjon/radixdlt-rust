use crate::Identifiable;

pub enum ValuePrefix {
    Address,
    Bytes,
    RRI,
    String,
    UInt256,
    EUID,
}

impl ValuePrefix {
    fn prefix(&self) -> String {
        match self {
            ValuePrefix::Address => ":adr:".into_string(),
            ValuePrefix::Bytes => ":byt:".into_string(),
            ValuePrefix::RRI => ":rri:".into_string(),
            ValuePrefix::String => ":str:".into_string(),
            ValuePrefix::UInt256 => ":u20:".into_string(),
            ValuePrefix::EUID => ":uid:".into_string(),
        }
    }
}

impl Identifiable for ValuePrefix {

    type Identifier = String;

    fn identifier(&self) -> Identifier {
        self.prefix();
    }
}