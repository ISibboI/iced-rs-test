use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPECIAL_CASE_A: Regex = Regex::new(r"(unit|euler).*").unwrap();
}

/// Returns the (more or less) correct instance of 'a' or 'an' to be used with the given word.
/// E.g. 'a train', 'an elite'.
/// Some special case like 'a unit' are obeyed.
pub fn a_or_an(word: &str) -> &'static str {
    let first_char = word.chars().next().unwrap_or('z');
    if ['a', 'e', 'i', 'o', 'u'].contains(&first_char) && !SPECIAL_CASE_A.is_match(word) {
        "an"
    } else {
        "a"
    }
}
