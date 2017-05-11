pub trait Generator {
    fn get_password(&self, length: i32, characters: &str) -> String;
}
