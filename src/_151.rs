// Time complexity: O(n), traverse the string for splitting, reverseing, and combining.
// Space complexity: O(n), store n characters during splitting.
// https://chatgpt.com/c/67057993-4dfc-8002-9971-d3ad5a07f101
pub fn reverse_words(s: String) -> String {
    // Use library standard functions for simplicity.

    // Split the string into words.
    // split_whitespace ignores multiple whitespaces.
    let mut words: Vec<&str> = s.split_whitespace().collect();

    // Reverse the the words.
    words.reverse();

    // Join words with space.
    words.join(" ")
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
