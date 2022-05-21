mod helpers;
mod structs;
use helpers::parse_input;

fn get_error_message(input: String) -> String {
    format!("Error: {} was not a valid question", input)
}

pub fn calculate(input: String) -> String {
    let answer = parse_input(input.clone());
    match answer {
        Ok(answer) => match answer.parse::<i32>() {
            Ok(answer) => answer.to_string(),
            Err(_) => get_error_message(input),
        },
        Err(_) => get_error_message(input),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn simple_add() {
        let question = String::from("1+3");
        let calculated_answer = calculate(question);
        let correct_answer = "4".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn multi_add() {
        let question = String::from("1+2+3");
        let calculated_answer = calculate(question);
        let correct_answer = "6".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_subtract() {
        let question = String::from("3-1");
        let calculated_answer = calculate(question);
        let correct_answer = "2".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn multi_subtract() {
        let question = String::from("10-3-2");
        let calculated_answer = calculate(question);
        let correct_answer = "5".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_multiply() {
        let question = String::from("3*2");
        let calculated_answer = calculate(question);
        let correct_answer = "6".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_division() {
        let question = String::from("10/2");
        let calculated_answer = calculate(question);
        let correct_answer = "5".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_paranthesis() {
        let question = String::from("(2-2)");
        let calculated_answer = calculate(question);
        let correct_answer = "0".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn double_paranthesis() {
        let question = String::from("(2-2)-(2+2)");
        let calculated_answer = calculate(question);
        let correct_answer = "-4".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn combine_operations() {
        let question = String::from("3*(10+2)");
        let calculated_answer = calculate(question);
        let correct_answer = "36".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn combine_operations_2() {
        let question = String::from("3*(10+2)/4");
        let calculated_answer = calculate(question);
        let correct_answer = "9".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn negative_numbers() {
        let question = String::from("-3");
        let calculated_answer = calculate(question);
        let correct_answer = "-3".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn negative_numbers_2() {
        let question = String::from("1--3");
        let calculated_answer = calculate(question);
        let correct_answer = "4".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn mixed() {
        let question = String::from("5*-3+-3/-3");
        let calculated_answer = calculate(question);
        let correct_answer = "-14".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn mixed_2() {
        let question = String::from("5--3+-12/(-3*2)");
        let calculated_answer = calculate(question);
        let correct_answer = "10".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }

    #[test]
    fn mixed_3() {
        let question = String::from("10-(20*2)/4+20");
        let calculated_answer = calculate(question);
        let correct_answer = "20".to_string();
        assert_eq!(calculated_answer, correct_answer);
    }

    #[test]
    fn error_1() {
        let question = String::from("1s1");
        let calculated_answer = calculate(question.clone());
        let correct_error_message = get_error_message(question);
        assert_eq!(calculated_answer, correct_error_message);
    }
    #[test]
    fn error_2() {
        let question = String::from("1+1s");
        let calculated_answer = calculate(question.clone());
        let correct_error_message = get_error_message(question);
        assert_eq!(calculated_answer, correct_error_message);
    }
}
