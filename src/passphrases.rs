//! --- Day 4: High-Entropy Passphrases ---
//!
//! A new system policy has been put in place that requires all accounts to
//! use a passphrase instead of simply a password. A passphrase consists of a
//! series of words (lowercase letters) separated by spaces.
//!
//! To ensure security, a valid passphrase must contain no duplicate words.
//!
//! For example:
//!
//! aa bb cc dd ee is valid.
//! aa bb cc dd aa is not valid - the word aa appears more than once.
//! aa bb cc dd aaa is valid - aa and aaa count as different words.
//!
//! The system's full passphrase list is available as your puzzle input.
//! How many passphrases are valid?

use std::collections::HashSet;
use std::iter::FromIterator;

pub fn validate(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            // compare the source length vs the de-duped length
            // they should match if there are no duplicates
            let set: HashSet<&str> = HashSet::from_iter(tokens.clone());
            tokens.len() == set.len()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aa_bb_cc_dd_ee_is_valid() {
        assert_eq!(validate("aa bb cc dd ee"), 1)
    }
    #[test]
    fn test_aa_bb_cc_dd_aa_is_not_valid() {
        assert_eq!(validate("aa bb cc dd aa"), 0)
    }
    #[test]
    fn test_aa_bb_cc_dd_aaa_is_valid() {
        assert_eq!(validate("aa bb cc dd aaa"), 1)
    }
}
