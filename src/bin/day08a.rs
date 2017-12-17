extern crate aoc;

use std::env;
use aoc::recursive_circus;


fn main() {
    let args = env::args().collect::<Vec<String>>();
    let fp = &args[1];


    let data = aoc::read_file(fp);


    let node_map = recursive_circus::build_node_map(&data);

    let result = recursive_circus::find_root(&node_map);

    println!("{:?}", result);
}
