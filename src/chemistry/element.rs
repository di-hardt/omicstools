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

#[cfg(test)]
mod test {
    // std imports
    use std::fs::read;

    // local imports
    use super::*;

    const ELEMENTS_FILE: &'static str = "data/elements.csv";

    #[test]
    fn test_completeness() {
        let plain_elements = String::from_utf8(read(ELEMENTS_FILE).unwrap()).unwrap();

        let elements_lines: Vec<String> = plain_elements
            .split("\n")
            .map(|line| line.to_owned())
            .collect();

        // -1 because of the headers
        assert!(ELEMENTS.len() == elements_lines.len() - 1);

        for line in &elements_lines[1..] {
            let attributes: Vec<&str> = line.split(",").collect();
            let symbol = attributes[1];
            let element = get_element_by_symbol(symbol).unwrap();
            assert!(element.get_symbol() == symbol)
        }
    }

    #[test]
    fn test_getting_unknown_element() {
        assert!(get_element_by_symbol("Ä").is_err());
    }
}
