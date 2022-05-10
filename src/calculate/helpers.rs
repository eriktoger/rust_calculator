pub fn parse(input: String) -> String {
    let addition_handled = handle_operation(input, '+');
    let subtraction_handled = handle_operation(addition_handled, '-');
    let multiplication_handled = handle_operation(subtraction_handled, '*');
    let division_handled = handle_operation(multiplication_handled, '/');
    let parenthesis_handled = handle_parenthesis(division_handled);

    return parenthesis_handled;
}

fn handle_parenthesis(input: String) -> String {
    let first = input.chars().next().unwrap();
    let last = input.chars().last().unwrap();
    if first == '(' && last == ')' {
        let trimmed_input = input[1..input.len() - 1].to_string();
        return parse(trimmed_input);
    }
    return input;
}

fn get_values(input: String, index: usize) -> (i32, i32) {
    let before = input[..index].to_string();
    let after = input[index + 1..].to_string();
    let first = parse(before).parse::<i32>();
    let second = parse(after).parse::<i32>();
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
        if c == '(' {
            left_paranthesis += 1;
        } else if c == ')' {
            right_paranthesis += 1;
        }

        if left_paranthesis == right_paranthesis && c == operator {
            let (first_int, second_int) = get_values(input, index_counter);
            match operator {
                '+' => return (first_int + second_int).to_string(),
                '-' => return (first_int - second_int).to_string(),
                '/' => return (first_int / second_int).to_string(),
                '*' => return (first_int * second_int).to_string(),
                _ => panic!("Operator not implemented"),
            }
        }
    }
    return input;
}
