use random;

pub trait Generator {
    fn get_password(&mut self, length: i32, characters: &str) -> String;
}

pub struct StandardGenerator {
    generator: &'static mut random::Generator
}

impl StandardGenerator {
    pub fn new(generator: &'static mut random::Generator) -> StandardGenerator {
        StandardGenerator {
            generator: generator
        }
    }
}

impl Generator for StandardGenerator {
    fn get_password(&mut self, length: i32, characters: &str) -> String {
        let mut password = String::new();

        for index in 0..length {
            let character = self.generator.get_character(characters);

            password.push(character);
        }

        password
    }
}
