extern crate rand;
use rand::Rng;

pub struct Nonce {
    pub value: i64,
    _make_constructor_private: ()
}

impl Nonce {
    pub fn new() -> Nonce {
        let mut rng = rand::thread_rng();
        return Nonce{value: rng.gen(), _make_constructor_private: () }
    }
}

impl Default for Nonce {
    fn default() -> Self { Nonce::new() }
}