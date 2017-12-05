
pub mod captcha;
pub mod checksum;
pub mod spiral_memory;
pub mod passphrases;
pub mod trampoline_maze;

use std::path::Path;
use std::fs::File;
use std::io::Read;

pub fn read_file<P: AsRef<Path>>(path: P) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf).unwrap();
    buf
}
