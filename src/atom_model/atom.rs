// External crates

// Internal crates
use crate::atom_model::{ParticleGroup};
use crate::{RadixHash, RadixHashable, MetaData};

pub struct Signature;

/// An atom is the fundamental atomic unit of storage on the ledger (similar to a block
/// in a blockchain) and defines the actions that can be issued onto the ledger.
pub struct Atom {
    pub particle_groups: Vec<ParticleGroup>,
    pub meta_data: MetaData,
    pub signatures: Vec<Signature>,
    version: i32,
}

pub struct RadixSerializerVersion {
    pub version: i32,
}

impl Default for RadixSerializerVersion {
    fn default() -> Self {
        RadixSerializerVersion { version: 100 }
    }
}

impl Atom {
    fn new(
        particle_groups: Vec<ParticleGroup>,
        meta_data: ChronoMetaData,
        signatures: Vec<Signature>
    ) -> Self {

    }
}

//impl DSONSerializable for Atom {
//    fn to_dson(self) -> Vec<u8> {
//        unimplemented!()
//    }
//}
impl RadixHashable for Atom {
    fn radix_hash(&self) -> RadixHash {

    }
}

impl Default for Atom {

    fn default() -> Self {
        Atom {
            ..Default::default()
        }
    }
}
