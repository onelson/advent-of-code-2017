extern crate aoc;

use std::env;
use aoc::captcha;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = &args[1];

    let result = captcha::solve_shift_1(&input);

    println!("{}", result);
}
