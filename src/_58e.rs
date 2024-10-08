// Time complexity: O(n), n is the number of characters in string s.
// Space complexity: O(1), constant additional space used.
pub fn length_of_last_word(s: String) -> i32 {
    // Given a string of words and spaces.
    // Return the length of the last word.
    // Consider trailing spaces possible.
    // Use reverse traversal.

    // Initialize variables.
    // Use bytes for efficency.
    let byts = s.as_bytes();
    let mut word_length = 0;
    let mut idx = byts.len() as isize - 1;

    // Skip any trailing space.
    while idx >= 0 && byts[idx as usize] == b' ' {
        idx -= 1;
    }

    // Traverse bytes in reverse for last word length.
    while idx >= 0 && byts[idx as usize] != b' ' {
        idx -= 1;
        word_length += 1;
    }

    word_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_case() {
        assert_eq!(length_of_last_word(String::from("Hello World")), 5);
    }

    #[test]
    fn test_with_multiple_spaces() {
        assert_eq!(
            length_of_last_word(String::from("   fly me   to   the moon  ")),
            4
        );
    }

    #[test]
    fn test_single_word() {
        assert_eq!(length_of_last_word(String::from("rust")), 4);
    }

    #[test]
    fn test_trailing_spaces() {
        assert_eq!(length_of_last_word(String::from("hello   ")), 5);
    }

    #[test]
    fn test_invalid_input() {
        // Assuming empty string is invalid as per problem constraints
        assert_eq!(length_of_last_word(String::from("")), 0);
    }
}
