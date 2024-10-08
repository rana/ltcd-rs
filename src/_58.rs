// Time complexity: O(n), where n is the length of string s.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/67056c80-1d08-8002-9608-91869489e728
pub fn length_of_last_word(s: String) -> i32 {
    // Use reverse traversal and in-place computation.

    // Convert string to bytes for effcient indexing.
    let byts = s.as_bytes();
    // Start from the last index.
    // Use isize for reverse traversal.
    let mut idx = byts.len() as isize - 1;
    let mut wrd_len = 0;

    // Skip any trailing spaces at the end of the string.
    while idx >= 0 && byts[idx as usize] == b' ' {
        idx -= 1;
    }

    // Count the last word length.
    while idx >= 0 && byts[idx as usize] != b' ' {
        wrd_len += 1;
        idx -= 1;
    }

    wrd_len
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
