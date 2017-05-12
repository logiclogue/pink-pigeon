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
    let digest = get_hash(input);
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut generator = SeededGenerator::new(&digest);
    let index = generator.get_in_range(0, 35);

    println!("{} {} {}", letters, index, get_char_from_index(&letters, index));
}
