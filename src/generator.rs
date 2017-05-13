pub trait Generator {
    fn get_password(&self, length: i32, characters: &str) -> String;
}

pub struct StandardGenerator;

impl StandardGenerator {
    pub fn new(seed: &str) -> StandardGenerator {
        StandardGenerator
    }
}

impl Generator for StandardGenerator {
    fn get_password(&mut self, length: i32, characters: &str) -> String {

    }
}
