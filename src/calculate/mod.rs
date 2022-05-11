mod helpers;
use helpers::parse_input;
pub fn calculate(input: String) -> i32 {
    parse_input(input).parse::<i32>().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn simple_add() {
        let question = String::from("1+3");
        let calculated_answer = calculate(question);
        let correct_answer = 4;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn multi_add() {
        let question = String::from("1+2+3");
        let calculated_answer = calculate(question);
        let correct_answer = 6;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_subtract() {
        let question = String::from("3-1");
        let calculated_answer = calculate(question);
        let correct_answer = 2;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn multi_subtract() {
        let question = String::from("10-3-2");
        let calculated_answer = calculate(question);
        let correct_answer = 5;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_multiply() {
        let question = String::from("3*2");
        let calculated_answer = calculate(question);
        let correct_answer = 6;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_division() {
        let question = String::from("10/2");
        let calculated_answer = calculate(question);
        let correct_answer = 5;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_paranthesis() {
        let question = String::from("(2-2)");
        let calculated_answer = calculate(question);
        let correct_answer = 0;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn double_paranthesis() {
        let question = String::from("(2-2)-(2+2)");
        let calculated_answer = calculate(question);
        let correct_answer = -4;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn combine_operations() {
        let question = String::from("3*(10+2)");
        let calculated_answer = calculate(question);
        let correct_answer = 36;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn combine_operations_2() {
        let question = String::from("3*(10+2)/4");
        let calculated_answer = calculate(question);
        let correct_answer = 9;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn negative_numbers() {
        let question = String::from("-3");
        let calculated_answer = calculate(question);
        let correct_answer = -3;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn negative_numbers_2() {
        let question = String::from("1--3");
        let calculated_answer = calculate(question);
        let correct_answer = 4;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn mixed() {
        let question = String::from("5*-3+-3/-3");
        let calculated_answer = calculate(question);
        let correct_answer = -14;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn mixed_2() {
        let question = String::from("5--3+-12/(-3*2)");
        let calculated_answer = calculate(question);
        let correct_answer = 10;
        assert_eq!(calculated_answer, correct_answer);
    }

    #[test]
    fn mixed_3() {
        let question = String::from("10-(20*2)/4+20");
        let calculated_answer = calculate(question);
        let correct_answer = 20;
        assert_eq!(calculated_answer, correct_answer);
    }
}
