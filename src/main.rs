mod calculate;
use calculate::calculate;
use std::io;

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.pop(); // remove newline
    return input;
}

fn main() {
    let input = get_user_input();
    println!("{}", calculate(input));
}
