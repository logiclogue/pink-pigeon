#[cfg(test)]
mod standard_generator_tests {
    use random;
    use random::Generator as RandomGenerator;
    use password::StandardGenerator;
    use password::Generator;

    #[test]
    fn get_password__called__returns_expected_password() {
        let mut random_generator = random::SeededGenerator::new("test");
        let mut password_generator = StandardGenerator::new(random_generator);
    }
}
