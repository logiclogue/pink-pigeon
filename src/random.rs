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
        let s = self.seed
            .chars()
            .take(4)
            .collect();
        let random_hash;

        let mut hash = Md5::new();

        hash.input_str(&self.seed);

        random_hash = hash.result_str();

        let num = i32::from_str_radix(&s, 16);

        match num {
            Ok(min) => num % max,
            Err(_) => 0
        }
    }
}
