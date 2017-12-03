//! Day 2: Corruption Checksum
//!
//! As you walk through the door, a glowing humanoid shape yells in your direction. "You there!
//! Your state appears to be idle. Come help us repair the corruption in this spreadsheet - if we
//! take another millisecond, we'll have to display an hourglass cursor!"
//! The spreadsheet consists of rows of apparently-random numbers. To make sure the recovery
//! process is on the right track, they need you to calculate the spreadsheet's checksum. For each
//! row, determine the difference between the largest value and the smallest value; the checksum is
//! the sum of all of these differences.
//!
//! For example, given the following spreadsheet:
//! 5 1 9 5
//! 7 5 3
//! 2 4 6 8
//!
//! The first row's largest and smallest values are 9 and 1, and their difference is 8.
//! The second row's largest and smallest values are 7 and 3, and their difference is 4.
//! The third row's difference is 6.
//! In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
//!
//! --- Part Two ---
//!
//! "Great work; looks like we're on the right track after all. Here's a star for your effort."
//! However, the program seems a little worried. Can programs be worried?
//!
//! "Based on what we're seeing, it looks like all the User wanted is some information about the
//! evenly divisible values in the spreadsheet. Unfortunately, none of us are equipped for that
//! kind of calculation - most of us specialize in bitwise operations."
//! It sounds like the goal is to find the only two numbers in each row where one evenly divides
//! the other - that is, where the result of the division operation is a whole number. They would
//! like you to find those numbers on each line, divide them, and add up each line's result.
//! For example, given the following spreadsheet:
//!
//! 5 9 2 8
//! 9 4 7 3
//! 3 8 6 5
//!
//! In the first row, the only two numbers that evenly divide are 8 and 2; the result of this
//! division is 4.
//! In the second row, the two numbers are 9 and 3; the result is 3.
//! In the third row, the result is 2.
//! In this example, the sum of the results would be 4 + 3 + 2 = 9.


use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf).unwrap();
    buf
}

pub fn parse_cells(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

pub fn calculate_min_max(input: &[Vec<u32>]) -> u32 {
    input.iter().fold(0, |acc, row| {
        acc + (row.iter().max().unwrap() - row.iter().min().unwrap())
    })
}

pub fn calculate_div(input: &[Vec<u32>]) -> u32 {
    // We're looking for a pair of numbers that can be divided without a remainder.
    // I assume we only expect one such pair per row...

    // This is going to be fairly high complexity -- O(n^2)? Something like that...
    // Woof this is gross. Will plan to revisit.
    input.iter().fold(0, |acc, row| {
        let pair = row.iter()
            .map(|x| {
                let candidate = row.iter().filter(|y| *y != x).find(|y| x % *y == 0);
                candidate.map(|y| (x, y))
            })
            .filter(|o| o.is_some())
            .next();

        let result = match pair {
            Some(Some((a, b))) => (a / b),
            _ => 0,
        };
        acc + result
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];

        assert_eq!(calculate_min_max(&input), 18);
    }

    #[test]
    fn test_example_2() {
        let input = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];

        assert_eq!(calculate_div(&input), 9);
    }
}
