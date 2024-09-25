use std::io;

fn main() {
    println!("Welcome to my calculator! What would you like to compute?");
    
    let mut input = String::new();

    loop {
        input.clear();
        println!("Please input your equation.");

        io::stdin().read_line(&mut input).expect("Uh oh, your system is totally fked!"); // this should not err lmao

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!("Input must have exactly three parts: two operands and one operator, separated by whitespace. (e.g., \"3 + 5\"). Try again.");
            continue;
        }

        // sorry for the non-idiomatic indenting, it helps me see the code.
        match (
            parts[0].parse::<i32>(), // parse parts[0] as an i32. this returns a Result<i32, Err(E)> (?)
            parts[1].chars().next(), // take parts[1], return a character Iterator, then take the first value in the Iterator. This returns an Option
            parts[2].parse::<i32>() // parse parts[0] as an i32. this returns a Result<i32, Err(E)> (?)
            ) {
                (
                    Ok(num1),
                    Some(op1),
                    Ok(num2)
                ) if "+-/*".contains(op1) => { // the syntax feels a little weird, but four conditions need to be true
                    println!("Valid input! It was: {}", input);
                    break;
                }
                _ => {
                    println!("Something is wrong with your input! Try again.");
                    continue;
                }
            }
    }

    println!("Valid input! It was: {}", input);
}