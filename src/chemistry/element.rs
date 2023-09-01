pub struct Element {
    name: &'static str,
    symbol: &'static str,
    mono_mass: f64,
    average_mass: f64,
}

impl Element {
    /// Returns the name of the element.
    ///
    pub fn get_name(&self) -> &'static str {
        return &self.name;
    }

    /// Returns the symbol of the element.
    ///
    pub fn get_symbol(&self) -> &'static str {
        return &self.symbol;
    }

    /// Returns the monoisotopic mass of the element.
    ///
    pub fn get_mono_mass(&self) -> &f64 {
        return &self.mono_mass;
    }

    /// Returns the average mass of the element.
    ///
    pub fn get_average_mass(&self) -> &f64 {
        return &self.average_mass;
    }
}

// Include elements from data/elements.csv
include!(concat!(env!("OUT_DIR"), "/element.rs"));
