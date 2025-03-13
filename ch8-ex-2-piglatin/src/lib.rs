pub fn to_pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(|s| {
            println!("s: {s}");
            let chars: Vec<char> = s.chars().collect();
            let first_char = chars[0];
            let modifier;
            let mut prefix = String::from(s);
            if "aoiueAOIUE".contains(first_char) {
                modifier = String::from("h");
            } else if chars.len() == 1 || !first_char.is_alphabetic() {
                modifier = String::from("");
            } else {
                modifier = String::from(first_char);
                prefix = chars[1..].iter().collect::<String>()
            }
            format!("{}-{}ay", prefix, &modifier)
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
    fn test_single_letter_uppercase_vowel() {
        assert_eq!(to_pig_latin("A"), "A-hay");
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
    fn test_words_with_newlines() {
        assert_eq!(to_pig_latin("hello\nworld"), "ello-hay orld-way");
    }

    #[test]
    fn test_unicode_chars() {
        assert_eq!(to_pig_latin("über étoile"), "ber-üay toile-éay");
    }

    #[test]
    fn test_long_word() {
        assert_eq!(
            to_pig_latin("supercalifragilisticexpialidocious"),
            "upercalifragilisticexpialidocious-say"
        );
    }

    #[test]
    #[ignore]
    fn test_all_caps() {
        assert_eq!(to_pig_latin("HELLO WORLD"), "ELLO-HAY ORLD-WAY");
    }

    #[test]
    fn test_multiple_hyphens() {
        assert_eq!(to_pig_latin("top-notch-quality"), "op-notch-quality-tay");
    }

    #[test]
    fn test_multiple_apostrophes() {
        assert_eq!(to_pig_latin("rock'n'roll"), "ock'n'roll-ray");
    }

    #[test]
    #[ignore]
    fn test_only_special_chars() {
        assert_eq!(to_pig_latin("!@#$%"), "!@#$%-ay");
    }

    #[test]
    fn test_multiple_consecutive_spaces() {
        assert_eq!(to_pig_latin("hello      world"), "ello-hay orld-way");
    }

    #[test]
    fn test_mixed_whitespace() {
        assert_eq!(to_pig_latin("hello\t \n  world"), "ello-hay orld-way");
    }

    #[test]
    fn test_emoji() {
        assert_eq!(to_pig_latin("👋 world"), "👋-ay orld-way");
    }

    #[test]
    fn test_very_long_input() {
        let long_input = "hello ".repeat(1000);
        assert_eq!(
            to_pig_latin(&long_input),
            "ello-hay ".repeat(999) + "ello-hay"
        );
    }

    #[test]
    fn test_numeric_only_words() {
        assert_eq!(to_pig_latin("123 456"), "123-ay 456-ay");
    }

    #[test]
    #[ignore]
    fn test_mixed_scripts() {
        assert_eq!(to_pig_latin("hello 你好"), "ello-hay 你好-ay");
    }

    #[test]
    #[ignore]
    fn test_zero_width_spaces() {
        assert_eq!(to_pig_latin("hello\u{200B}world"), "ello-hay orld-way");
    }

    #[test]
    #[ignore]
    fn test_rtl_text() {
        assert_eq!(to_pig_latin("hello עולם"), "ello-hay עולם-ay");
    }
}
