mod random;

extern crate crypto;

use std::env;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = &args[1];
    let digest = get_hash(input);
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let index = get_random_number(&digest) % 36;

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

fn get_random_number(input: &str) -> i32 {
    let s = input
        .chars()
        .take(4)
        .collect();

    let num = i32::from_str_radix(&s, 16);

    match num {
        Ok(input_int) => input_int,
        Err(_) => 0
    }
}
