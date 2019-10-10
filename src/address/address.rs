// Standard Library Crates

// Third Party Crates
use bitcoin::secp256k1::PublicKey;
use bitcoin::util::base58::from_check;

// Internal Crates
use crate::{AddressError, DSONSerializable, Magic, RadixHash};

#[derive(Eq, PartialEq, Hash)]
pub struct Address {
//    magic_byte: u8,
    pub base58_string: String,
    pub public_key: PublicKey,
    _make_constructor_private: ()
}

type AddressValidation = Result<(), AddressError>;

impl Address {

    fn new(base58_string: String, public_key: PublicKey) -> Self {
        Address { base58_string: base58_string, public_key: public_key, _make_constructor_private: () }
    }

    fn from_magic_byte(magic_byte: u8, public_key: PublicKey) -> Self {
        let base58_string = Address::base58_string_from(magic_byte, public_key);
        Address::new(base58_string, public_key)
    }

    pub fn from_magic(magic: Magic, public_key: PublicKey) -> Self {
        Address::from_magic_byte(magic.byte(), public_key)
    }

    pub fn on_base58(base58_str: &str) -> Self {
        assert_eq!(base58_str.len(), 51);
        let result = from_check(base58_str);
        let mut bytes = result.unwrap();
        let public_key_result = PublicKey::from_slice(&bytes);
        let public_key = public_key_result.unwrap();
        Address::new(String::from(base58_str), public_key)
    }

    fn checksummed(data: &Vec<u8>) -> [u8; 36] {
        let radix_hash = RadixHash::sha256sha256(&data);

        let mut array = [0u8; 36];
        for index in 0..32 {
            array[index] = data[index];
        }
        for index in 0..4 {
            array[32 + index] = radix_hash.data[index]
        }
        return array;
    }

    fn is_checksummed(data: Vec<u8>) -> AddressValidation {
        let checksummed_dropped = data[0 .. (data.len() - 4)].to_vec();
        let calc_checksummed = Self::checksummed(&checksummed_dropped);

        if data != checksummed_dropped {
            return Err(AddressError::new());
        }

        // All is well
        Ok(())
    }

}

impl DSONSerializable for Address {
    fn to_dson(&self) -> Vec<u8> {
        unimplemented!()
    }

//    fn radix_hash(&self) -> RadixHash {
//        let compressed_pub_key = &self.public_key.serialize().to_vec();
//
//        RadixHash::sha256sha256(compressed_pub_key)
//    }

}
