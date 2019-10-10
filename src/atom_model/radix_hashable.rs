use crate::{RadixHash, EUID};

pub trait HashIdentifiable {
    fn to_euid(&self) -> EUID;
}

pub trait RadixHashable: HashIdentifiable {
    fn radix_hash(&self) -> RadixHash;
}

impl<RH: RadixHashable> HashIdentifiable for RH {
    fn to_euid(&self) -> EUID {
        self.radix_hash().to_euid();
    }
}
