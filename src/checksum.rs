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
    input.lines().map(|line| {
        line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect()
    }).collect()
}

pub fn calculate(input: &[Vec<u32>]) -> u32 {
    input.iter().fold(0, |acc, row| {
        acc + (row.iter().max().unwrap() - row.iter().min().unwrap())
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = vec![
            vec![5, 1, 9, 5],
            vec![7, 5, 3],
            vec![2, 4, 6, 8]
        ];

        assert_eq!(calculate(&input), 18);
    }
}
