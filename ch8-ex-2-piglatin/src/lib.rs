pub fn to_pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| {
            println!("s: {s}");
            let chars: Vec<char> = s.chars().collect();
            let first_char = chars[0];
            let suffix;
            let prefix;
            if "aoiue".contains(first_char) {
                suffix = String::from("-hay");
                prefix = chars[..].iter().collect::<String>()
            } else if chars.len() == 1 {
                suffix = String::from("-ay");
                prefix = chars[..].iter().collect::<String>()
            } else {
                suffix = format!("-{first_char}ay");
                prefix = chars[1..].iter().collect::<String>()
            }
            prefix + &suffix
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_word_starting_with_consonant() {
        assert_eq!(to_pig_latin("first"), "irst-fay");
    }

    #[test]
    fn test_single_word_starting_with_vowel() {
        assert_eq!(to_pig_latin("apple"), "apple-hay");
    }

    #[test]
    fn test_multiple_words() {
        assert_eq!(to_pig_latin("hello world"), "ello-hay orld-way");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(to_pig_latin(""), "");
    }

    #[test]
    fn test_single_letter_consonant() {
        assert_eq!(to_pig_latin("b"), "b-ay");
    }

    #[test]
    fn test_single_letter_vowel() {
        assert_eq!(to_pig_latin("a"), "a-hay");
    }

    #[test]
    fn test_mixed_case_words() {
        assert_eq!(to_pig_latin("Hello World"), "ello-Hay orld-Way");
    }

    #[test]
    fn test_punctuation() {
        assert_eq!(to_pig_latin("hello, world!"), "ello,-hay orld!-way");
    }

    #[test]
    fn test_numbers_in_words() {
        assert_eq!(to_pig_latin("h3llo w0rld"), "3llo-hay 0rld-way");
    }

    #[test]
    fn test_words_with_hyphens() {
        assert_eq!(to_pig_latin("co-op"), "o-op-cay");
    }

    #[test]
    fn test_words_with_apostrophes() {
        assert_eq!(to_pig_latin("don't"), "on't-day");
    }

    #[test]
    fn test_words_with_mixed_characters() {
        assert_eq!(to_pig_latin("h3ll0 w0rld"), "3ll0-hay 0rld-way");
    }

    #[test]
    fn test_words_with_leading_spaces() {
        assert_eq!(to_pig_latin("  hello world"), "ello-hay orld-way");
    }

    #[test]
    fn test_words_with_trailing_spaces() {
        assert_eq!(to_pig_latin("hello world  "), "ello-hay orld-way");
    }

    #[test]
    fn test_words_with_multiple_spaces_between() {
        assert_eq!(to_pig_latin("hello   world"), "ello-hay orld-way");
    }

    #[test]
    fn test_words_with_tabs() {
        assert_eq!(to_pig_latin("hello\tworld"), "ello-hay orld-way");
    }
}
