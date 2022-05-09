fn get_values(input: &str, delimiter: char) -> (i32, i32) {
    let split = input.split(delimiter);
    let vec: Vec<&str> = split.collect();
    let first = vec[0].parse::<i32>().unwrap();
    let second = vec[1].parse::<i32>().unwrap();
    (first, second)
}

pub fn addition(input: String) -> i32 {
    let (first, second) = get_values(&input, '+');
    return first + second;
}

pub fn subtraction(input: String) -> i32 {
    let (first, second) = get_values(&input, '-');
    return first - second;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_add() {
        let question = String::from("1+3");
        let calculated_answer = addition(question);
        let correct_answer = 4;
        assert_eq!(calculated_answer, correct_answer);
    }
    #[test]
    fn simple_subtract() {
        let question = String::from("3-1");
        let calculated_answer = subtraction(question);
        let correct_answer = 2;
        assert_eq!(calculated_answer, correct_answer);
    }
}
