use sha2::{Sha256, Digest};
use sha2::digest::{FixedOutput};
use crate::{EUID, clone_into_array};

pub struct RadixHash {
    pub data: [u8; 32],
    _make_constructor_private: ()
}

pub struct SHA256Once {
    pub digest: [u8; 32],
    _make_constructor_private: ()
}

impl SHA256Once {
    pub fn from_unhashed(unhashed_data: &Vec<u8>) -> Self {
        let mut hasher = Sha256::new();
        hasher.input(unhashed_data);
        let digest = hasher.fixed_result();
        const BYTE_COUNT: usize = 32;
        let mut hash_result: [u8; BYTE_COUNT] = [Default::default(); BYTE_COUNT];
        digest.iter().zip(hash_result.iter_mut()).map(|(&digest, hash_result)| *hash_result = digest).count();
        return SHA256Once { digest: hash_result, _make_constructor_private: ()};
    }
}

pub struct SHA256Twice {
    pub digest: [u8; 32],
    _make_constructor_private: ()
}

impl SHA256Twice {
    pub fn from_unhashed(unhashed_data: &Vec<u8>) -> Self {
        let once = SHA256Once::from_unhashed(unhashed_data);
        let twice = SHA256Once::from_unhashed(&once.digest.to_vec());
        return SHA256Twice { digest: twice.digest, _make_constructor_private: () }
    }
}

impl RadixHash {
    pub fn sha256sha256(unhashed_data: &Vec<u8>) -> RadixHash {
        let twice = SHA256Twice::from_unhashed(unhashed_data);
        return RadixHash { data: twice.digest, _make_constructor_private: () };
    }
}



impl RadixHash {
    pub fn to_euid(&self) -> EUID {
        let prefix = clone_into_array(&self.data[0 .. 16]);
        return EUID::from_data(prefix);
    }
}
impl Iterator for RadixHash {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        return self.into_iter().next();
    }
}