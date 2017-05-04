mod random;

extern crate crypto;

use std::env;
use crypto::digest::Digest;
use crypto::md5::Md5;
use random::Generator;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = &args[1];
    let digest = get_hash(input);
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let generator = random::SeededGenerator::new(&digest);
    let index = generator.get_in_range(0, 36);

    println!("{} {} {}", letters, index, get_char_from_index(&letters, index));
}

fn get_char_from_index(letters: &str, index: i32) -> char {
    for (i, value) in letters.chars().enumerate() {
        if i == index as usize {
            return value;
        }
    }

    ' '
}

fn get_hash(input: &str) -> String {
    let mut hash = Md5::new();

    hash.input_str(&input);

    hash.result_str()
}
