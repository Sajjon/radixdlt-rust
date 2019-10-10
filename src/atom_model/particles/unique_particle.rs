//public let address: Address
//public let name: String
//public let nonce: Nonce

use crate::{Address, Nonce, Particle};

pub struct UniqueParticle {
    pub address: Address,
    pub string: String,
    pub nonce: Nonce,
    _make_constructor_private: ()
}

impl UniqueParticle {
    pub fn new(address: Address, string: String) -> UniqueParticle {
        return UniqueParticle{
            address: address,
            string: string,
            nonce: Nonce::default(),
            _make_constructor_private: ()
        }
    }
}

impl Particle for UniqueParticle {}
