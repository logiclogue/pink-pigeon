#[cfg(test)]
mod standard_generator_tests {
    use random;
    use random::Generator;
    use password;

    #[test]
    fn test_function_name() {
        let mut random_generator = random::SeededGenerator::new("test");
        let character = random_generator.get_character("ABC");

        assert_eq!('A', character);
    }
}
