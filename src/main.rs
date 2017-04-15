extern crate crypto;

use std::env;
use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    let mut hash = Md5::new();
    let digest;
    let args: Vec<_> = env::args().collect();
    let input = &args[1];

    hash.input_str(&input);

    digest = hash.result_str();

    println!("{}", digest);
}
