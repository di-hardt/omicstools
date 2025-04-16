use std::slice::Iter;

use super::is_element::IsElement;

pub trait IsList<'a, E>
where
    E: IsElement,
{
    fn iter(&'a self) -> Iter<'a, E>;
}
