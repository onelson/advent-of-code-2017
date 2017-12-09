extern crate aoc;

use std::env;
use aoc::memory_reallocation;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = &args[1];

    println!("reading memory banks: {}", fp);

    let data = aoc::read_file(fp);
    let memory = data.split_whitespace()
        .map(|v| v.parse::<u32>().unwrap())
        .collect();
    let result = memory_reallocation::execute(memory);
    println!("iterations: {}", result);
}
