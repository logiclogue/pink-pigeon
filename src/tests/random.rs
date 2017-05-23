#[cfg(test)]
mod seeded_generator_tests {
    use random::Generator;
    use random::SeededGenerator;

    #[test]
    fn get__called__returns_expected_number() {
        // arrange
        let seed = "test";
        let mut generator = SeededGenerator::new(seed);
        let result;

        // act
        result = generator.get();

        // assert
        assert_eq!(2447, result);
    }
    
    #[test]
    fn get_in_range__called__returns_expected_number() {
        // arrange
        let seed = "test";
        let mut generator = SeededGenerator::new(seed);
        let result;
        let min = 1;
        let max = 6;
        let expected_result = (2447 % 5) + 1;

        // act
        result = generator.get_in_range(min, max);

        // assert
        assert_eq!(expected_result, result);
    }

    #[test]
    fn get_character__called__returns_expected_character() {
        // arrange
        let seed = "test";
        let mut generator = SeededGenerator::new(seed);
        let letters = "ABCDE";
        let expected_result = 'D';
        let result;

        // act
        result = generator.get_character(letters);

        // assert
        assert_eq!(expected_result, result);
    }
}
