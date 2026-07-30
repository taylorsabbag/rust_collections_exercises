const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn pig_latin(string: &String) -> String {
    if string.is_empty() {
        return String::from("");
    }

    let words: Vec<&str> = string.split_whitespace().collect();
    let mut processed_words = Vec::new();

    for (_index, &word) in words.iter().enumerate() {
        let has_period = word.ends_with('.');
        let clean_word = word.trim_end_matches('.');

        if let Some(first_letter) = clean_word.chars().next() {
            let lower_first = first_letter.to_ascii_lowercase();

            let mut new_word = if VOWELS.iter().any(|&letter| letter == lower_first) {
                format!("{clean_word}-hay")
            } else {
                let remainder: String = clean_word.chars().skip(1).collect();
                format!("{remainder}-{first_letter}ay")
            };

            if has_period {
                new_word.push('.');
            }

            processed_words.push(new_word);
        }
    }

    processed_words.join(" ")
}
