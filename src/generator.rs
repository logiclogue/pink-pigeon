pub trait Generator {
    fn get_password(&self, length: i32, characters: &str) -> String;
}

pub struct StandardGenerator;

impl Generator for StandardGenerator {
    
}
