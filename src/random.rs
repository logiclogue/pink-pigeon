use crypto::md5::Md5;
use crypto::digest::Digest;

pub trait RandomNumberGenerator {
    fn get_in_range(&self, min: i32, max: i32) -> i32;
}

pub struct SeededRandomNumberGenerator {
    seed: String
}

impl SeededRandomNumberGenerator {
    fn new(seed: String) -> SeededRandomNumberGenerator {
        SeededRandomNumberGenerator {
            seed: seed
        }
    }
}

impl RandomNumberGenerator for SeededRandomNumberGenerator {
    fn get_in_range(&self, min: i32, max: i32) -> i32 {
        let digest;
        let s;
        let num;
        let mut hash = Md5::new();

        hash.input_str(&self.seed);

        digest = hash.result_str();

        s = digest
            .chars()
            .take(4)
            .collect();

        num = i32::from_str_radix(&s, 16);

        match num {
            Ok(num) => num % max,
            Err(_) => 0
        }
    }
}
