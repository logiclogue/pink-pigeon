use crypto::md5::Md5;
use crypto::digest::Digest;

pub trait Generator {
    fn get(&mut self) -> i32;
    fn get_in_range(&mut self, min: i32, max: i32) -> i32;
    fn get_character(&mut self, letters: &str) -> char;
}

pub struct SeededGenerator {
    seed: String,
    hash: Md5
}

impl SeededGenerator {
    pub fn new(seed: &str) -> SeededGenerator {
        SeededGenerator {
            seed: seed.to_string(),
            hash: Md5::new()
        }
    }
}

impl Generator for SeededGenerator {
    fn get(&mut self) -> i32 {
        let digest;
        let s: String;
        let num;

        self.hash.input_str(&self.seed);

        digest = self.hash.result_str();

        s = digest
            .chars()
            .take(4)
            .collect();

        num = i32::from_str_radix(&s, 16);

        match num {
            Ok(input_int) => input_int,
            Err(_) => 0
        }
    }

    fn get_in_range(&mut self, min: i32, max: i32) -> i32 {
        let diff = max - min;

        (self.get() % diff) + min
    }

    fn get_character(&mut self, letters: &str) -> char {
        let max = letters.len() as i32 - 1;
        let index = self.get_in_range(0, max);
        let character = get_char_from_index(letters, index);

        character
    }
}

fn get_char_from_index(letters: &str, index: i32) -> char {
    for (i, value) in letters.chars().enumerate() {
        if i == index as usize {
            return value;
        }
    }

    ' '
}
