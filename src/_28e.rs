pub fn str_str(haystack: String, needle: String) -> i32 {
    // Use library functions for simplicity and maintainability.

    // Check input minimum edge case.
    if needle.is_empty() {
        return 0;
    }

    match haystack.find(&needle) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_needle() {
        assert_eq!(str_str("hello".to_string(), "".to_string()), 0);
    }

    #[test]
    fn needle_not_in_haystack() {
        assert_eq!(str_str("hello".to_string(), "world".to_string()), -1);
    }

    #[test]
    fn needle_at_start() {
        assert_eq!(str_str("hello".to_string(), "he".to_string()), 0);
    }

    #[test]
    fn needle_at_end() {
        assert_eq!(str_str("hello".to_string(), "lo".to_string()), 3);
    }

    #[test]
    fn needle_in_middle() {
        assert_eq!(str_str("ababcabc".to_string(), "abc".to_string()), 2);
    }

    #[test]
    fn needle_repeated() {
        assert_eq!(str_str("aaaaa".to_string(), "aaa".to_string()), 0);
    }

    #[test]
    fn haystack_empty() {
        assert_eq!(str_str("".to_string(), "a".to_string()), -1);
    }

    #[test]
    fn haystack_and_needle_empty() {
        assert_eq!(str_str("".to_string(), "".to_string()), 0);
    }

    // #[test]
    // fn unicode_characters() {
    //     assert_eq!(str_str("こんにちは世界".to_string(), "にち".to_string()), 2);
    // }

    #[test]
    fn special_characters() {
        assert_eq!(str_str("!@#$%^&*()".to_string(), "^&*".to_string()), 5);
    }
}
