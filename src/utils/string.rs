/// Converts a string to snake_case
/// e.g. `Two Sum` => `two_sum`
/// e.g. `Contains Duplicate` => `contains_duplicate`
pub fn to_snake_case(str: &str) -> String {
    // TODO Test with values that have special chars. Maybe just remove them.
    str.split_whitespace()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}
