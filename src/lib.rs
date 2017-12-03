
pub mod captcha {
    //! Day 1: Captcha

    pub fn shift_chars(input: &str, index: usize) -> String {
        format!("{}{}", &input[index..], &input[..index])
    }

    pub fn solve_shift_1(input: &str) -> u32 {
        let shifted = shift_chars(input, 1);
        input
            .chars()
            .zip(shifted.chars())
            .filter(|t| t.0 == t.1)
            .fold(0, |acc, t| acc + t.0.to_string().parse::<u32>().unwrap())
    }

    pub fn solve_shift_half(input: &str) -> u32 {
        let shifted = shift_chars(input, input.len() / 2);
        input
            .chars()
            .zip(shifted.chars())
            .filter(|t| t.0 == t.1)
            .fold(0, |acc, t| acc + t.0.to_string().parse::<u32>().unwrap())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn shift_abc_by_1() {
            assert_eq!(&shift_chars("abc", 1), "bca");
        }

        #[test]
        fn shift_abc_by_half() {
            let input = "abcd";
            assert_eq!(&shift_chars(input, input.len() / 2), "cdab");
        }

        #[test]
        fn solve_1122() {
            assert_eq!(solve_shift_1("1122"), 3);
        }

        #[test]
        fn solve_1111() {
            assert_eq!(solve_shift_1("1111"), 4);
        }

        #[test]
        fn solve_1234() {
            assert_eq!(solve_shift_1("1234"), 0);
        }

        #[test]
        fn solve_91212129() {
            assert_eq!(solve_shift_1("91212129"), 9);
        }
    }
}
