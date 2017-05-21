#[cfg(test)]
mod standard_generator_tests {
    use super::super::super::password::Generator;
    use super::super::super::password::StandardGenerator;
    use super::super::super::random::Generator;
    use super::super::super::random::SeededGenerator;

    #[test]
    fn test_function_name() {
        let random_generator = random::SeededGenerator::new("test");
        let password_generator = password::StandardGenerator::new(
            random_generator,
            "ABC");

        assert_eq!("A", random_generator.get_character("ABC"));
    }
}
