extern crate aoc;

use std::env;
use aoc::registers;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = &args[1];


    let data = aoc::read_file(fp);
    let result = registers::execute(data.lines().map(|line| registers::Instruction::from_string(line)).collect());
    println!("{:?}", result);
}
