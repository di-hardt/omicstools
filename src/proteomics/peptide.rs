// std imports
use std::str::FromStr;
use std::string::ToString;

// 3rd party imports
use anyhow::{bail, Error};

#[derive(Clone, Debug, PartialEq)]
pub enum Terminus {
    N,
    C,
}

impl FromStr for Terminus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "N" => Ok(Terminus::N),
            "C" => Ok(Terminus::C),
            _ => bail!("Invalid terminus. Valid termini are `N` or `C`"),
        }
    }
}

impl ToString for Terminus {
    fn to_string(&self) -> String {
        match self {
            Self::N => "N".to_owned(),
            Self::C => "C".to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terminus_from_str() {
        assert_eq!(Terminus::from_str("N").unwrap(), Terminus::N);
        assert_eq!(Terminus::from_str("C").unwrap(), Terminus::C);
        assert!(Terminus::from_str("X").is_err());
    }

    #[test]
    fn test_terminus_to_string() {
        assert_eq!(Terminus::N.to_string(), "N");
        assert_eq!(Terminus::C.to_string(), "C");
    }
}
