use std::io;

fn main() {
    println!("Welcome to my calculator! What would you like to compute?");
    
    let mut input = String::new();

    loop {
        println!("Please input your equation.");

        input.clear();

        io::stdin().read_line(&mut input).expect("Uh oh, your system is totally fked!");

        if input.split_whitespace().count() != 3 {
            println!("Input must have exactly three parts: two operands and one operator, separated by whitespace. (e.g., \"3 + 5\"). Try again.");
            continue;
        }

        break;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();

    let num1 = parts[0].parse::<i32>();
    let op = parts[1].parse::<char>();
    let num2 = parts[2].parse::<char>();

    // now check

    println!("Valid input! It was: {}", input);
}