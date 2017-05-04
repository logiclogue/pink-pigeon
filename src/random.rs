use crypto::md5::Md5;
use crypto::digest::Digest;

pub trait Generator {
    fn get_in_range(&self, min: i32, max: i32) -> i32;
}

pub struct SeededGenerator {
    seed: String
}

impl SeededGenerator {
    pub fn new(seed: &str) -> SeededGenerator {
        SeededGenerator {
            seed: seed.to_string()
        }
    }
}

impl Generator for SeededGenerator {
    fn get_in_range(&self, min: i32, max: i32) -> i32 {
        let digest;
        let s: String;
        let num;
        let mut hash = Md5::new();
        let diff = max - min;
        let result;

        hash.input_str(&self.seed);

        digest = hash.result_str();

        s = digest
            .chars()
            .take(4)
            .collect();

        num = i32::from_str_radix(&s, 16);

        result = match num {
            Ok(input_int) => input_int,
            Err(_) => 0
        };

        (result % diff) + min
    }
}
