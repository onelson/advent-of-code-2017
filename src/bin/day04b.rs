extern crate aoc;

use std::env;
use aoc::passphrases;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = &args[1];
    println!("reading spreadsheet: {}", fp);
    let data = aoc::read_file(fp);
    let result = passphrases::validate_anagrams(&data);

    println!("{}", result);
}
