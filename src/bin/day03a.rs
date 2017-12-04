extern crate aoc;

use std::env;
use aoc::spiral_memory;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = &args[1];
    println!("walking to cell: {}", input);
    let result = spiral_memory::walk(input.parse::<u64>().unwrap());
    println!("{}", result);
}
