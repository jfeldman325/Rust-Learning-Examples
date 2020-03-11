fn main() {
    let s = String::from("This is a great string. the best really.");
    println!("{}", find_word(&s, 6));
}

fn find_word(s: &str, word: u32) -> &str {
    let bytes = s.as_bytes();
    let mut word_counter: u32 = 0;
    let mut word_start_index = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            word_counter += 1;
            if word_counter == word {
                return &s[word_start_index..i];
            } else if i < s.len() - 1 {
                word_start_index = i + 1;
            }
        }
    }
    "don't know what you mean!"
}
