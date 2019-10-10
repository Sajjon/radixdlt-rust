use crate::atom_model::{Particle, Spin};

pub struct SpunParticle {
    pub particle: Box<dyn Particle>,
    pub spin: Spin
}

impl SpunParticle {
    pub fn new<P: 'static>(particle: P, spin: Spin) -> Self where P: Particle {
        let boxed_particle = Box::new(particle);
        return SpunParticle { particle: boxed_particle, spin: spin };
    }
}