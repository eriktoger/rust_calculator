mod helpers;
use helpers::parse;
pub fn calculate(input: String) -> i32 {
    parse(input).parse::<i32>().unwrap()
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
}
