extern crate aoc;

use std::env;
use aoc::checksum;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = &args[1];
    println!("reading spreadsheet: {}", fp);
    let data = checksum::read_file(fp);
    let cells = checksum::parse_cells(&data);

    let result = checksum::calculate_div(&cells);

    println!("{}", result);
}
