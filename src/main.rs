extern crate crypto;

use std::env;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let args: Vec<_> = env::args().collect();
    let input = &args[1];
    let digest = get_hash(input);

    println!("{}", get_random_number(&digest) % 36);
}

fn get_hash(input: &str) -> String {
    let mut hash = Md5::new();
    let digest;

    hash.input_str(&input);

    digest = hash.result_str();

    digest
}

fn get_random_number(input: &str) -> i32 {
    let s: String = input
        .chars()
        .take(4)
        .collect();

    let num = i32::from_str_radix(&s, 16);

    match num {
        Ok(input_int) => input_int,
        Err(_) => 0
    }
}
