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

#[cfg(test)]
mod test {
    // std imports
    use std::fs::read;

    // local imports
    use super::*;

    const SUBATOMIC_PARTICLES_FILE: &'static str = "data/subatomic_particles.csv";

    #[test]
    fn test_completeness() {
        let plain_subatomic_particles =
            String::from_utf8(read(SUBATOMIC_PARTICLES_FILE).unwrap()).unwrap();

        let subatomic_particles_lines: Vec<String> = plain_subatomic_particles
            .split("\n")
            .map(|line| line.to_owned())
            .collect();

        // -1 because of the headers
        assert!(SUBATOMIC_PARTICLES.len() == subatomic_particles_lines.len() - 1);

        for line in &subatomic_particles_lines[1..] {
            let attributes: Vec<&str> = line.split(",").collect();
            let name = attributes[0];
            let subatomic_particle = get_subatomic_particle_by_name(name).unwrap();
            assert!(subatomic_particle.get_name() == name)
        }
    }

    #[test]
    fn test_getting_unknown_subatomic_particle() {
        assert!(get_subatomic_particle_by_name("some_imaginary_name").is_err());
    }
}
