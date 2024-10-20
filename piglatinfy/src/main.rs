use std::io;

fn main() {
    println!("Please enter your sentence.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("reading of input errored.");

    let parts: Vec<String> = input.split_whitespace().map(String::from).collect();
    let mut output: Vec<String> = Vec::new();

    //@dev reimplement this
    for p in &parts {
        let (word_part, punct_part) = split_words_from_punctuation(p);

        if !word_part.is_empty() {
            match pig_latinfy(&word_part) {
                Some(mut pig_word) => {
                    pig_word.push_str(&punct_part);
                    output.push(pig_word);
                },
                None => panic!("Unexpected empty word in pig_latinfy: {}", p),
            }
        } else if !punct_part.is_empty() {
            output.push(punct_part);
        }
    }

    println!("in-ay igPay atinLay isthay entencesay is-ay: {}", output.join(" "));
}

fn split_words_from_punctuation(part: &str) -> (String, String) {
    let word_part: String = part.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let punct_part: String = part.chars().skip(word_part.len()).collect();
    (word_part, punct_part)
}

fn pig_latinfy(word: &str) -> Option<String> {
    let mut chars: Vec<char> = word.chars().collect();

    if chars.is_empty() {
        return None;
    }

    let first_char = chars[0].to_lowercase().next().unwrap();
    
    // If it starts with a vowel, just add "ay"
    if matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u') {
        chars.extend(['-', 'a', 'y']);
    }
    // because I check if it's is_ascii_alphabetic() before, I can move straight to an else
    else {
        let first = chars.remove(0);
        chars.extend([first, 'a', 'y']);
    }

    Some(chars.into_iter().collect())
}