use super::structs::MathSymbols;
use regex::Regex;

fn handle_addition(input: String) -> Result<String, ()> {
    let new_input = replace_double_negative(input);
    handle_operation(Ok(new_input), MathSymbols::ADDITION)
}

fn handle_subtraction(input: Result<String, ()>) -> Result<String, ()> {
    match input {
        Ok(input) => {
            let new_input = replace_negative(input);
            handle_operation(Ok(new_input), MathSymbols::SUBTRACTION)
        }
        Err(_) => Err(()),
    }
}

pub fn parse_input(input: String) -> Result<String, ()> {
    let addition_handled = handle_addition(input);
    let subtraction_handled = handle_subtraction(addition_handled);
    let multiplication_handled = handle_operation(subtraction_handled, MathSymbols::MULTIPLICATION);
    let division_handled = handle_operation(multiplication_handled, MathSymbols::DIVISION);
    return handle_parenthesis(division_handled);
}

fn handle_parenthesis(result: Result<String, ()>) -> Result<String, ()> {
    if result.is_err() {
        return result;
    }
    let input = result.unwrap();
    if input.is_empty() {
        return Err(());
    }

    let first = input.chars().next().unwrap();
    let last = input.chars().last().unwrap();
    if first == MathSymbols::LEFT_PARANTHESIS && last == MathSymbols::RIGHT_PARANTHESIS {
        let trimmed_input = input[1..input.len() - 1].to_string();
        return parse_input(trimmed_input);
    }
    return Ok(input);
}

fn replace_negative(input: String) -> String {
    let regex_string = format!(r"(.*[\d|\)])({})([\d|\(].*)$", MathSymbols::NEGATIVE);
    let find_negative = Regex::new(&regex_string).unwrap();

    let with_subtraction = format!("${{1}}{}${{3}}", MathSymbols::SUBTRACTION);
    return find_negative
        .replace_all(&input, with_subtraction)
        .to_string();
}
fn replace_double_negative(input: String) -> String {
    input.replace("--", "+")
}

fn get_values(input: String, index: usize) -> Result<(i32, i32), ()> {
    if index >= input.len() {
        return Err(());
    }
    let before = input[..index].to_string();
    let after = input[index + 1..].to_string();
    let first = parse_input(before);
    let second = parse_input(after);
    match (first, second) {
        (Ok(first), Ok(second)) => match (first.parse::<i32>(), second.parse::<i32>()) {
            (Ok(first), Ok(second)) => Ok((first, second)),
            _ => Err(()),
        },
        _ => Err(()),
    }
}

fn handle_operation(result: Result<String, ()>, operator: char) -> Result<String, ()> {
    if result.is_err() {
        return result;
    }
    let input = result.unwrap();
    let mut left_paranthesis = 0;
    let mut right_paranthesis = 0;
    let mut index_counter = input.len();
    for c in input.chars().rev() {
        index_counter -= 1;
        match c {
            MathSymbols::LEFT_PARANTHESIS => left_paranthesis += 1,
            MathSymbols::RIGHT_PARANTHESIS => right_paranthesis += 1,
            _ => (),
        }
        if left_paranthesis == right_paranthesis && c == operator {
            return apply_operation(input, index_counter, operator);
        }
    }
    return Ok(input);
}

fn apply_operation(input: String, index_counter: usize, operator: char) -> Result<String, ()> {
    let values = get_values(input, index_counter);
    match values {
        Ok(values) => {
            let (first, second) = values;
            match operator {
                MathSymbols::ADDITION => return Ok((first + second).to_string()),
                MathSymbols::SUBTRACTION => return Ok((first - second).to_string()),
                MathSymbols::DIVISION => return Ok((first / second).to_string()),
                MathSymbols::MULTIPLICATION => return Ok((first * second).to_string()),
                _ => panic!("Operator not implemented"),
            }
        }
        Err(_) => return Err(()),
    }
}
