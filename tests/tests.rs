use units::*;

#[cfg(test)]
mod tests {
    use super::*;

    // --- Тести для Question::validate_answer (EP + BVA) ---

    #[test]
    fn test_1_validate_first_option_correct() {
        // BVA: Позитивний, нижня межа індексу (0)
        // Arrange
        let q = Question { id: 1, options: vec!["A".into(), "B".into()], correct_option_index: 0, time_limit: 10 };
        // Act
        let result = q.validate_answer(0);
        // Assert
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_2_validate_last_option_correct() {
        // BVA: Позитивний, верхня допустима межа індексу
        // Arrange
        let q = Question { id: 1, options: vec!["A".into(), "B".into()], correct_option_index: 1, time_limit: 10 };
        // Act
        let result = q.validate_answer(1);
        // Assert
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_3_validate_wrong_index() {
        // EP: Позитивний, допустимий клас (неправильна відповідь)
        // Arrange
        let q = Question { id: 1, options: vec!["A".into(), "B".into()], correct_option_index: 0, time_limit: 10 };
        // Act
        let result = q.validate_answer(1);
        // Assert
        assert_eq!(result.unwrap(), false);
    }

    #[test]
    fn test_4_validate_out_of_bounds_high() {
        // BVA: Негативний, перше значення за межами (index == len)
        // Arrange
        let q = Question { id: 1, options: vec!["A".into()], correct_option_index: 0, time_limit: 10 };
        // Act
        let result = q.validate_answer(1);
        // Assert
        assert!(result.is_err());
    }

    // --- Тести для Result::calculate_points (EP + BVA) ---

    #[test]
    fn test_5_points_instant_response() {
        // BVA: Позитивний, нижня межа часу (0.0)
        // Arrange
        let (time, limit) = (0.0, 10);
        // Act
        let points = Result::calculate_points(time, limit);
        // Assert
        assert_eq!(points, 1000);
    }

    #[test]
    fn test_6_points_at_exact_limit() {
        // BVA: Позитивний, верхня межа часу (рівно ліміту)
        // Arrange
        let (time, limit) = (10.0, 10);
        // Act
        let points = Result::calculate_points(time, limit);
        // Assert
        assert_eq!(points, 500);
    }

    #[test]
    fn test_7_points_just_below_limit() {
        // BVA: Позитивний, значення безпосередньо біля межі (limit - 0.1)
        // Arrange
        let (time, limit) = (9.9, 10);
        // Act
        let points = Result::calculate_points(time, limit);
        // Assert
        assert!(points > 500);
    }

    #[test]
    fn test_8_points_over_limit_negative() {
        // EP: Негативний, час більше ліміту
        // Arrange
        let (time, limit) = (11.0, 10);
        // Act
        let points = Result::calculate_points(time, limit);
        // Assert
        assert_eq!(points, 0);
    }

    #[test]
    fn test_9_points_negative_time_input() {
        // EP: Негативний, недопустиме від'ємне значення
        // Arrange
        let (time, limit) = (-1.0, 10);
        // Act
        let points = Result::calculate_points(time, limit);
        // Assert
        assert_eq!(points, 0);
    }

    // --- Тести для GameSession::submit_answer (Інтеграційні/EP) ---

    #[test]
    fn test_10_submit_correct_mid_time() {
        // EP: Позитивний, типовий сценарій (середній час)
        // Arrange
        let q = Question { id: 1, options: vec!["A".into()], correct_option_index: 0, time_limit: 10 };
        let session = GameSession { current_question: q };
        // Act
        let score = session.submit_answer(0, 5.0);
        // Assert
        assert_eq!(score, 750);
    }

    #[test]
    fn test_11_submit_with_error_handling() {
        // EP: Негативний, обробка помилкового індексу через сесію
        // Arrange
        let q = Question { id: 1, options: vec!["A".into()], correct_option_index: 0, time_limit: 10 };
        let session = GameSession { current_question: q };
        // Act
        let score = session.submit_answer(99, 1.0);
        // Assert
        assert_eq!(score, 0);
    }
    
    #[test]
    fn test_12_points_at_boundary_minus_epsilon() {
        // BVA: Значення безпосередньо біля нижньої межі (-0.1)
        // Перевіряє гілку response_time < 0.0
        let points = Result::calculate_points(-0.1, 10);
        assert_eq!(points, 0);
    }
    
    #[test]
    fn test_13_points_at_boundary_plus_epsilon() {
        // BVA: Значення безпосередньо біля верхньої межі (limit + 0.1)
        // Перевіряє гілку response_time > limit
        let points = Result::calculate_points(10.1, 10);
        assert_eq!(points, 0);
    }
    
    #[test]
    fn test_14_validate_index_at_upper_boundary() {
        // BVA: Перевірка індексу, що дорівнює довжині списку (options.len())
        // Гарантує, що оператор >= спрацьовує вірно для запобігання panic
        let q = Question { id: 1, options: vec!["A".into(), "B".into()], correct_option_index: 0, time_limit: 10 };
        let result = q.validate_answer(2);
        assert!(result.is_err());
    }
}