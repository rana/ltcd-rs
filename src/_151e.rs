pub fn reverse_words(s: String) -> String {
    // Given a string of words.
    // Reverse order of words.
    // Return a concatenated word with single space separator.
    // Use library functions for simplicity.

    let mut wrds: Vec<&str> = s.split_whitespace().collect();
    wrds.reverse();
    wrds.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        let s = String::from("the sky is blue");
        assert_eq!(reverse_words(s), "blue is sky the");
    }

    #[test]
    fn test_leading_trailing_spaces() {
        let s = String::from("  hello world  ");
        assert_eq!(reverse_words(s), "world hello");
    }

    #[test]
    fn test_multiple_spaces_between_words() {
        let s = String::from("a   b    c");
        assert_eq!(reverse_words(s), "c b a");
    }

    #[test]
    fn test_empty_string() {
        let s = String::from("");
        assert_eq!(reverse_words(s), "");
    }

    #[test]
    fn test_spaces_only() {
        let s = String::from("     ");
        assert_eq!(reverse_words(s), "");
    }

    #[test]
    fn test_single_word() {
        let s = String::from("hello");
        assert_eq!(reverse_words(s), "hello");
    }
}
