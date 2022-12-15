const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

pub fn from(s: &str) -> String {
    s.split_whitespace()
        .map(translate_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn translate_word(s: &str) -> String {
    let s_only_letters = only_letters(s);
    let mut translated_word = String::new();

    for (idx, c) in s_only_letters.chars().enumerate() {
        if idx == 0 && is_vowel(&c) {
            translated_word.push_str(&s_only_letters);
            translated_word.push_str("-hay");
            break;
        } else if is_consonant(&c) {
            translated_word.push_str(&s_only_letters[idx + 1..]);
            translated_word.push('-');
            translated_word.push(c.to_ascii_lowercase());
            translated_word.push_str("ay");
            break;
        } else {
            translated_word.push(c);
        }
    }

    translated_word
}

fn only_letters(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<String>()
}

fn is_vowel(c: &char) -> bool {
    VOWELS.contains(&c.to_ascii_lowercase())
}

fn is_consonant(c: &char) -> bool {
    !is_vowel(c) && c.is_alphabetic()
}
