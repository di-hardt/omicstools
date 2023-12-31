/// Creats a Map- or Set-like collection, e.g.
/// ```txt
/// // HashMap
/// collection! {
///     "SOMEKEY".to_string() => "A_VALUE",
///     "ANOTHERKEY".to_string() => "ANOTHER_VALUE",
/// }
///
/// // HashSet
/// collection! {
///     "SOMETHING_UNIQUE".to_string(),
///     "ANOTHER_UNIQUE".to_string(),
/// }
/// ```
/// Copied from https://stackoverflow.com/a/27582993
// Used
#[allow(unused_macros)]
macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::iter::IntoIterator::into_iter([$(($k, $v),)*]))
    };
    // set-like
    ($($v:expr),* $(,)?) => {
        std::iter::Iterator::collect(std::iter::IntoIterator::into_iter([$($v,)*]))
    };
}
