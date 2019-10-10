use crate::{Particle, Address, MetaData, Nonce};

pub struct MessageParticle {
    pub sender: Address,
    pub recipient: Address,
    pub payload: Vec<u8>,
    pub meta_data: MetaData,
    pub nonce: Nonce,
    _make_constructor_private: ()
}

impl MessageParticle {
    pub fn new(sender: Address, recipient: Address, payload: Vec<u8>, meta_data: Option<MetaData>, nonce: Option<Nonce>) -> MessageParticle {
        return MessageParticle{
            sender: sender,
            recipient: recipient,
            payload: payload,
            meta_data: meta_data.unwrap_or(MetaData::default()),
            nonce: nonce.unwrap_or(Nonce::default()),
            _make_constructor_private: ()
        }
    }
}

impl Particle for MessageParticle {}
