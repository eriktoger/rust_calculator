use regex::Regex;

struct MathSymbols;

impl MathSymbols {
    const LEFT_PARANTHESIS: char = '(';
    const RIGHT_PARANTHESIS: char = ')';
    const ADDITION: char = '+';
    const NEGATIVE: char = '-';
    const SUBTRACTION: char = '_';
    const MULTIPLICATION: char = '*';
    const DIVISION: char = '/';
}

fn handle_addition(input: String) -> String {
    let new_input = replace_double_negative(input);
    handle_operation(new_input, MathSymbols::ADDITION)
}

fn handle_subtraction(input: String) -> String {
    let new_input = replace_negative(input);
    handle_operation(new_input, MathSymbols::SUBTRACTION)
}

pub fn parse_input(input: String) -> String {
    let addition_handled = handle_addition(input);
    let subtraction_handled = handle_subtraction(addition_handled);
    let multiplication_handled = handle_operation(subtraction_handled, MathSymbols::MULTIPLICATION);
    let division_handled = handle_operation(multiplication_handled, MathSymbols::DIVISION);
    return handle_parenthesis(division_handled);
}

fn handle_parenthesis(input: String) -> String {
    let first = input.chars().next().unwrap();
    let last = input.chars().last().unwrap();
    if first == MathSymbols::LEFT_PARANTHESIS && last == MathSymbols::RIGHT_PARANTHESIS {
        let trimmed_input = input[1..input.len() - 1].to_string();
        return parse_input(trimmed_input);
    }
    return input;
}

fn replace_negative(input: String) -> String {
    let regex_string = format!(r"(.*\d)({})(\d.*)$", MathSymbols::NEGATIVE);
    let find_negative = Regex::new(&regex_string).unwrap();
    let with_subtraction = format!("${{1}}{}${{3}}", MathSymbols::SUBTRACTION);
    return find_negative
        .replace_all(&input, with_subtraction)
        .to_string();
}
fn replace_double_negative(input: String) -> String {
    input.replace("--", "+")
}

fn get_values(input: String, index: usize) -> (i32, i32) {
    let before = input[..index].to_string();
    let after = input[index + 1..].to_string();
    let first = parse_input(before).parse::<i32>();
    let second = parse_input(after).parse::<i32>();
    let first_int = first.unwrap();
    let second_int = second.unwrap();
    return (first_int, second_int);
}

fn handle_operation(input: String, operator: char) -> String {
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
            let (first_int, second_int) = get_values(input, index_counter);
            match operator {
                MathSymbols::ADDITION => return (first_int + second_int).to_string(),
                MathSymbols::SUBTRACTION => return (first_int - second_int).to_string(),
                MathSymbols::DIVISION => return (first_int / second_int).to_string(),
                MathSymbols::MULTIPLICATION => return (first_int * second_int).to_string(),
                _ => panic!("Operator not implemented"),
            }
        }
    }
    return input;
}
