use std::io;

fn main() {
    println!("Welcome to my calculator! What would you like to compute?");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Uh oh, your system is totally fked!");

    let (first_operand, symbol, second_operand) = input.split_whitespace()

    println!("input so far is {}", input);
}

enum MathProblem {
    Add,
    Subtract,
    Multiply,
    Divide
}