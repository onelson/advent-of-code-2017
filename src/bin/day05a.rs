extern crate aoc;

use std::env;
use aoc::trampoline_maze;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = &args[1];

    println!("reading instruction set: {}", fp);

    let data = aoc::read_file(fp);
    let mut instructions = trampoline_maze::parse_instructions(&data);
    let result = trampoline_maze::execute(&mut instructions, trampoline_maze::jump);
    println!("{}", result);
}
