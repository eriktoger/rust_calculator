mod operations;
use operations::{addition, subtraction};
use std::io;

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input;
}

fn main() {
    let input = get_user_input();
    if input.contains("+") {
        println!("{}", addition(input));
    } else if input.contains("-") {
        println!("{}", subtraction(input));
    }
}
