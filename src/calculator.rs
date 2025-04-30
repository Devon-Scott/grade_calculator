pub struct Calculator {}

impl Calculator {
    pub fn calculate_str_2_f(tokens: &String) -> Option<f32>{
        let parsed = Calculator::parse(tokens);
        if parsed != None {
            return Some(0.69)
        }
        None
    }

    pub fn weighted_sum(grades: &Vec<f32>, weights: &Vec<f32>) -> Option<f32> {
        if grades.len() != weights.len() {
            return None
        }
        let mut sum = 0.0;
        for i in 0..grades.len() {
            sum += grades[i] * weights[i];
        }
        Some(sum)
    }

    fn parse(tokens: &String) -> Option<&String> {
        if tokens == ""{
            return Some(tokens)
        }
        None
    }
}