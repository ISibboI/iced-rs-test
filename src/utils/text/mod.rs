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
    if ['a', 'e', 'i', 'o', 'u'].contains(&first_char.to_ascii_lowercase())
        && !SPECIAL_CASE_A.is_match(&word.to_ascii_lowercase())
    {
        "an"
    } else {
        "a"
    }
}

pub fn ordinal_suffix(number: impl Into<i128>) -> &'static str {
    let number = number.into().abs();
    let lsd = number % 10;
    let slsd = (number % 100) / 10;
    if slsd != 1 {
        match lsd {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    } else {
        "th"
    }
}
