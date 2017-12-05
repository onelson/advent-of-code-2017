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
//!
//! --- Part Two ---
//!
//! For added security, yet another system policy has been put in place. Now, a valid passphrase
//! must contain no two words that are anagrams of each other - that is, a passphrase is invalid
//! if any word's letters can be rearranged to form any other word in the passphrase.
//!
//! For example:
//!
//! abcde fghij is a valid passphrase.
//! abcde xyz ecdab is not valid - the letters from the third word can be rearranged to form the
//! first word.
//! a ab abc abd abf abj is a valid passphrase, because all letters need to be used when forming
//! another word.
//! iiii oiii ooii oooi oooo is valid.
//! oiii ioii iioi iiio is not valid - any of these words can be rearranged to form any other word.
//! Under this new system policy, how many passphrases are valid?

use std::collections::HashSet;
use std::iter::FromIterator;
use std::collections::BTreeMap;

pub fn validate_dupes(data: &str) -> usize {
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

pub type CharCounts = BTreeMap<char, usize>;

pub fn validate_anagrams(data: &str) -> usize {
    data.lines()
        .filter(|line| {
            let tokens: Vec<CharCounts> = line.split_whitespace().map(|w| count_chars(w)).collect();
            // check char counts to see if there is more than one btmap matching...
            !tokens.iter().any(|x| {
                tokens.iter().filter(|y| *y == x).count() > 1
            })
        })
        .count()
}

fn count_chars(word: &str) -> CharCounts {
    let mut counts = BTreeMap::new();
    for c in word.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aa_bb_cc_dd_ee_is_valid() {
        assert_eq!(validate_dupes("aa bb cc dd ee"), 1)
    }
    #[test]
    fn test_aa_bb_cc_dd_aa_is_not_valid() {
        assert_eq!(validate_dupes("aa bb cc dd aa"), 0)
    }
    #[test]
    fn test_aa_bb_cc_dd_aaa_is_valid() {
        assert_eq!(validate_dupes("aa bb cc dd aaa"), 1)
    }

    #[test]
    fn test_count_chars() {
        let expected = BTreeMap::from_iter(vec![('o', 1), ('i', 3)]);
        assert_eq!(count_chars("oiii"), expected);
        // order of the keys shouldn't matter
        let expected = BTreeMap::from_iter(vec![('i', 3), ('o', 1)]);
        assert_eq!(count_chars("oiii"), expected);
    }


    #[test]
    fn test_anagram_1() {
        assert_eq!(validate_anagrams("abcde fghij"), 1);
    }

    #[test]
    fn test_anagram_2() {
        assert_eq!(validate_anagrams("abcde xyz ecdab"), 0);
    }

    #[test]
    fn test_anagram_3() {
        assert_eq!(validate_anagrams("a ab abc abd abf abj"), 1);
    }

    #[test]
    fn test_anagram_4() {
        assert_eq!(validate_anagrams("iiii oiii ooii oooi oooo"), 1);
    }

    #[test]
    fn test_anagram_5() {
        assert_eq!(validate_anagrams("oiii ioii iioi iiio"), 0);
    }
}
