pub struct Question {
    pub id: i32,
    pub options: Vec<String>,
    pub correct_option_index: usize,
    pub time_limit: u32,
}

pub struct Result {
    pub score_earned: i32,
    pub response_time: f32,
}

impl Question {
    pub fn validate_answer(&self, index: usize) -> core::result::Result<bool, String> {
        if index >= self.options.len() {
            return Err(format!("Index {} out of bounds", index));
        }
        Ok(index == self.correct_option_index)
    }
}

impl Result {
    pub fn calculate_points(response_time: f32, time_limit: u32) -> i32 {
        if response_time > time_limit as f32 || response_time < 0.0 {
            return 0;
        }

        let multiplier = 1.0 - (response_time / time_limit as f32);
        (500.0 + (500.0 * multiplier)) as i32
    }
}

pub struct GameSession {
    pub current_question: Question,
}

impl GameSession {
    pub fn submit_answer(&self, answer_index: usize, time_taken: f32) -> i32 {
        match self.current_question.validate_answer(answer_index) {
            Ok(true) => Result::calculate_points(time_taken, self.current_question.time_limit),
            _ => 0,
        }
    }
}