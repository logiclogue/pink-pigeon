mod random;

extern crate crypto;

use std::env;
use crypto::digest::Digest;
use crypto::md5::Md5;
use random::Generator;
use random::SeededGenerator;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = &args[1];
    let mut generator = SeededGenerator::new(input);
    let index = generator.get_in_range(0, 35);
}
