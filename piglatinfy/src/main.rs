use std::io;

fn main() {
    println!("Please enter your sentence.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("reading of input errored.");

    let parts: Vec<&str> = input.split_whitespace().collect();
}

fn is_alphabetic (s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_alphabetic() {
            return false;
        }
    }
    return true;
}