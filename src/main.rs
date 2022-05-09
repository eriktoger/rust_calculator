mod operations;
use operations::addition;
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let answer = addition(buffer);
    println!("Answer: {}", answer);
}
