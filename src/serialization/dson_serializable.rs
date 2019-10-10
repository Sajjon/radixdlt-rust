// External crates

// Internal crates
use crate::atom_model::{RadixHashable, RadixHash };

pub trait DSONSerializable: RadixHashable {
    fn to_dson(&self) -> Vec<u8>;
}

// Rust forum: https://users.rust-lang.org/t/extending-traits/1802/3
// swift: `extension DSONSerializable where Self: RadixHashable`
impl<DS: DSONSerializable> RadixHashable for DS {
    fn radix_hash(&self) -> RadixHash {
        RadixHash::sha256sha256(self.to_dson())
    }
}
