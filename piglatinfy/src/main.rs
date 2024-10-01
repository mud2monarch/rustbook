use std::io;

fn main() {
    println!("Please enter your sentence.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("reading of input errored.");

    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut output: Vec<&str> = Vec::new();

    for word in &parts {
        if is_alphabetic(word) {
            match pig_latinfy(word) {
                Some(pig_word) => output.push(pig_word),
                None => panic!("Unexpected empty word in pig_latinfy: {}", word),
            }
        }
        // @dev TODO: next up
    }
}

fn is_alphabetic (s: &str) -> bool {
    for c in s.chars() {
        if !c.is_ascii_alphabetic() {
            return false;
        }
    }
    return true;
}

// note: usage should check if the full input is_ascii_alphabetic() prior to passing to this function.
fn pig_latinfy(word: &str) -> Option<String> {
    let mut chars: Vec<char> = word.chars().collect();

    if chars.is_empty() {
        return None;
    }

    let first_char = chars[0].to_lowercase().next().unwrap();
    
    // If it starts with a vowel, just add "ay"
    if matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u') {
        chars.extend(['a', 'y']);
    }
    // because I check if it's is_ascii_alphabetic() before, I can move straight to an else
    else {
        let first = chars.remove(0);
        chars.extend([first, 'a', 'y']);
    }

    Some(chars.into_iter().collect())
}