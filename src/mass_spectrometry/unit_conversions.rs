// internal imports
use crate::chemistry::element::HYDROGEN;

/// Converts mass to charge ration (Thompson) as Dalton
///
/// # Arguments
/// * `mz` - Mass to charge ratio (Thompson)
/// * `charge` - Charge
///
pub fn mass_to_charge_to_dalton(mz: f64, charge: u8) -> f64 {
    let charge = charge as f64;
    mz * charge - HYDROGEN.get_mono_mass() * charge
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mass_to_charge_ratio_to_dalton() {
        assert_eq!(
            mass_to_charge_to_dalton(464.888129195412, 3),
            1391.640912481236
        )
    }
}
