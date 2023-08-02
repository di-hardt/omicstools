pub struct SubatomicParticle {
    name: &'static str,
    mass: f64,
}

impl SubatomicParticle {
    /// Returns the name of the subatomic particle.
    ///
    pub fn get_name(&self) -> &'static str {
        return &self.name;
    }

    /// Returns the monoisotopic mass of the subatomic particle.
    ///
    pub fn get_mass(&self) -> &f64 {
        return &self.mass;
    }
}

// Include subatomic particles from data/subatomic_particles.csv
include!(concat!(env!("OUT_DIR"), "/subatomic_particle.rs"));
