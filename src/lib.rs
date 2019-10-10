pub use crate::address::*;
pub use crate::serialization::*;
pub use crate::atom_model::*;
pub use crate::utils::*;

mod utils;
mod atom_model;
mod address;
mod serialization;

#[cfg(test)]
mod tests {
    use crate::atom_model::*;
    use crate::Address;

    #[test]
    fn it_work_to_dson_serialize_empty_atom() {
        let alice = Address::on_base58("JH1P8f3znbyrDj8F4RWpix7hRkgxqHjdW2fNnKpR3v6ufXnknor");
        let unique_particle = UniqueParticle::new(alice, "HEJ".to_string());
        let spun_particle = SpunParticle::new(unique_particle, Spin::UP);
        let spun_particles = vec![spun_particle];
        let particle_group = ParticleGroup{ particles: spun_particles };
        let atom = Atom { particle_groups: vec![particle_group], ..Default::default() };
        let hash_id = atom.to_euid();
        assert_eq!(hash_id.to_hex_string(), "abc");
    }
}
