pub fn addition(input: String) -> i32 {
    let split = input.split(|c| c == '+' || c == '\n');
    let vec: Vec<&str> = split.collect();
    let first = vec[0].parse::<i32>().unwrap();
    let second = vec[1].parse::<i32>().unwrap();
    return first + second;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add() {
        let question = String::from("1+3");
        let calculated_answer = addition(question);
        let correct_answer = 4;
        assert_eq!(calculated_answer, correct_answer);
    }
}
