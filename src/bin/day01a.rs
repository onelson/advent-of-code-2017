//! Captcha

use std::env;

fn shift_chars(input: &str) -> String {
    let mut shifted = String::from(&input[1..]);
    shifted.push(input.chars().next().unwrap());
    shifted
}

fn solve(input: &str) -> u32 {
    let shifted = shift_chars(input);
    input
        .chars()
        .zip(shifted.chars())
        .filter(|t| t.0 == t.1)
        .fold(0, |acc, t| acc + t.0.to_string().parse::<u32>().unwrap())
}


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = &args[2];

    let result = solve(&input);

    println!("{}", result);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shift_abc() {
        assert_eq!(&shift_chars("abc"), "bca");
    }

    #[test]
    fn solve_1122() {
        assert_eq!(solve("1122"), 3);
    }

    #[test]
    fn solve_1111() {
        assert_eq!(solve("1111"), 4);
    }

    #[test]
    fn solve_1234() {
        assert_eq!(solve("1234"), 0);
    }

    #[test]
    fn solve_91212129() {
        assert_eq!(solve("91212129"), 9);
    }


}
