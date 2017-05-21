pub trait Generator {
    fn get_password(&self, length: i32, characters: &str) -> String;
}

pub struct StandardGenerator {
    generator: random::Generator,
    characters: &str
};

impl StandardGenerator {
    pub fn new(
        generator: random::Generator,
        characters: &str) -> StandardGenerator {
        StandardGenerator {
            generator: generator,
            characters: characters
        }
    }
}

impl Generator for StandardGenerator {
    fn get_password(&mut self, length: i32) -> String {
        let mut password;

        for index in 0..length {
            let character = self.generator.get_char_from_index(
                self.characters, index);

            password.push(character);
        }

        password
    }
}
