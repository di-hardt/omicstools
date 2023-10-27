// std imports
use std::convert::From;
use std::fmt::{Debug, Display};

// 3rd party imports
use anyhow::{bail, Result};
use quick_xml::events::{attributes::Attribute, BytesStart};

/// Simple error type for attribute not found so it is distinguishable from other errors.
///
pub struct AttributeNotFound(String);

impl Display for AttributeNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' not found", self.0)
    }
}

impl Debug for AttributeNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' not found", self.0)
    }
}

/// Get attribute vector from event while collecting errors.
///
/// # Arguments
/// * `event` - The event to get the attributes from.
///
pub fn get_attributes<'a>(event: &'a BytesStart<'a>) -> Result<Vec<Attribute<'a>>> {
    Ok(event
        .attributes()
        .collect::<std::result::Result<Vec<Attribute>, _>>()?)
}

/// Searches for an attribute with the given name and returns its value.
///
/// # Arguments
/// * `attr_name` - The name of the attribute to search for.
/// * `attributes` - The vector of attributes to search in.
///
pub fn get_value_of_attribute<T>(attr_name: &[u8], attributes: &Vec<Attribute>) -> Result<T>
where
    T: From<String>,
{
    match attributes
        .iter()
        .find(|attr| attr.key.as_ref() == attr_name)
    {
        Some(attr) => Ok(attr.unescape_value()?.as_ref().to_string().into()),
        None => bail!(AttributeNotFound(
            String::from_utf8_lossy(attr_name).to_string()
        )),
    }
}
