extern crate libc;

use libc::c_char;

use std::str::from_utf8;
use std::ascii::AsciiExt;
use std::ffi::CStr;

#[no_mangle]
/// External interface for the scoring algorithm
pub extern "C" fn ext_score(choice: *const c_char, query: *const c_char) -> f64 {
    let slice = unsafe { CStr::from_ptr(choice).to_bytes() };
    let choice = from_utf8(slice).unwrap();

    let slice = unsafe { CStr::from_ptr(query).to_bytes() };
    let query = from_utf8(slice).unwrap();

    score(choice, query)
}

pub fn score(choice: &str, query: &str) -> f64 {
    if query.len() == 0 {
        return 1.0;
    }

    if choice.len() == 0 {
        return 0.0;
    }

    // TODO: use UTF-8 versions of `to_lowercase()` when stable.
    let lower_choice = choice.to_ascii_lowercase();
    let lower_query = query.to_ascii_lowercase();
    let lower_choice_len = lower_choice.len() as f64;

    let match_length = compute_match_length(lower_choice.as_ref(), lower_query.chars().collect());

    match match_length {
        Some(match_length) => {
            let score = lower_query.len() as f64 / match_length as f64;
            score / lower_choice_len
        },
        None => { 0.0 },
    }

}

/// Find the length of the shortest substring matching the given characters.
fn compute_match_length(haystack: &str, needles: Vec<char>) -> Option<usize> {
    let first_char = needles.first().expect("Unable to get first char of needle");
    let rest = &needles[1..]; // use tail() whenever it stabilizes

    let first_indexes = find_char_in_string(haystack, first_char);

    first_indexes.iter().map(|&first_index|
        match find_end_of_match(haystack, rest, first_index) {
            Some(index) => {
                Some(index - first_index + 1)
            },
            None => { None }
        }
    ).filter(|&m|
        m.is_some()
    ).map(|m|
        m.unwrap()
    ).min()
}

/// Find all occurrences of the character in the string, returning their indexes.
fn find_char_in_string(haystack: &str, needle: &char) -> Vec<usize> {
    let mut index: usize = 0;
    let mut indexes = Vec::new();

    loop {
        index = match find_from_offset(haystack, *needle, index) {
            Some(i) => {
                indexes.push(i);
                i + 1
            },
            None => { break; },
        };
    }

    indexes
}

/// Find each of the characters in the string, moving strictly left to right.
fn find_end_of_match(haystack: &str, needles: &[char], first_index: usize) -> Option<usize> {
    let mut last_index = first_index;
    for needle in needles.iter() {
        last_index = match find_from_offset(haystack, *needle, last_index + 1) {
            Some(i) => i,
            None => { return None; },
        };
    }

    Some(last_index)
}

/// Implements Ruby's `#index` method
fn find_from_offset(haystack: &str, needle: char, offset: usize) -> Option<usize> {
    let h = &haystack[offset..];

    let index = h.find(needle);

    match index {
        Some(i) => Some(i + offset),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
    fn test_scores_zero_when_choice_is_empty() {
        assert!(score("", "a") == 0.0);
    }

	#[test]
    fn test_scores_one_when_query_is_empty() {
        assert!(score("a", "") == 1.0);
    }

	#[test]
    fn test_scores_zero_when_the_query_longer_than_choice() {
        assert!(score("short", "longer") == 0.0);
    }

	#[test]
    fn test_scores_zero_when_query_does_not_match_at_all() {
        assert!(score("a", "b") == 0.0);
    }

	#[test]
    fn test_scores_zero_when_only_prefix_of_query_matches() {
        assert!(score("ab", "ac") == 0.0);
    }

	#[test]
    fn test_scores_greater_than_zero_when_matches() {
        let given_choices: Vec<&str> = vec!("a", "ab", "ba", "bab");

        for choice in given_choices.iter() {
            assert!(score(*choice, "a") > 0.0);
        }

        assert!(score("babababab", "aaaa") > 0.0);
    }

	#[test]
    fn test_scores_1_normalized_to_length_when_the_query_equals_choice() {
        assert!(score("a", "a") == 1.0);
        assert!(score("ab", "ab") == 0.5);
        assert!(score("a long string", "a long string") == 1.0 / "a long string".len() as f64);
        assert!(score("spec/search_spec.rb", "sear") == 1.0 / "spec/search_spec.rb".len() as f64);
    }

	#[test]
    fn test_matches_punctuation() {
        assert!(score("/! symbols $^", "/!$^") > 0.0);
    }

	#[test]
    fn test_is_case_insensitive() {
        assert!(score("a", "A") == 1.0);
        assert!(score("A", "a") == 1.0);
    }

	#[test]
    fn test_does_not_match_when_same_letter_is_repeated_in_choice() {
        assert!(score("a", "aa") == 0.0);
    }

	#[test]
    fn test_scores_higher_for_better_matches() {
        assert!(score("selecta.gemspec", "asp") > score("algorithm4_spec.rb", "asp"));
        assert!(score("README.md", "em") > score("benchmark.rb", "em"));
        assert!(score("search.rb", "sear") > score("spec/search_spec.rb", "sear"));
    }

	#[test]
    fn test_scores_shorter_matches_higher() {
        assert!(score("fbb", "fbb") > score("foo bar baz", "fbb"));
        assert!(score("foo", "foo") > score("longer foo", "foo"));
        assert!(score("foo", "foo") > score("foo longer", "foo"));
        assert!(score("1/2/3/4", "1/2/3") > score("1/9/2/3/4", "1/2/3"));
    }

	#[test]
    fn test_sometimes_score_longer_strings_higher_if_better_match() {
        assert!(score("long 12 long", "12") > score("1 long 2", "12"));
    }

	#[test]
    fn test_scores_higher_of_two_matches_regardless_of_order() {
        let tight = "12";
        let loose = "1padding2";
        let expect1 = tight.to_string() + loose;
        let expect2 = loose.to_string() + tight;

        assert!(score(expect1.as_ref(), "12") == 1.0 / expect1.len() as f64);
        assert!(score(expect2.as_ref(), "12") == 1.0 / expect2.len() as f64);
    }
}
