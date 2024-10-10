pub fn length_of_longest_substring(s: String) -> i32 {
    // Longest substring without repeating characters
    // Given a string s.
    // Determine the max length substring.
    // Disallow repeating characters.
    // Return the max length substring.
    // Use a sliding window two-pointer technique.
    // Map character to index to track repeating characters.

    use std::collections::HashMap;

    // Initialize variables.
    let mut max_len: usize = 0;
    let mut lft: usize = 0;
    let mut ch_idx: HashMap<char, usize> = HashMap::new();

    // Loop through each character.
    for (rht, ch) in s.chars().enumerate() {
        // Search for duplicate character in window.
        if let Some(&prv_idx) = ch_idx.get(&ch) {
            // Check whether character is in window.
            if prv_idx >= lft {
                // Shrink left-side of window.
                lft = prv_idx + 1;
            }
        }

        // Track the character index for duplicates.
        ch_idx.insert(ch, rht);

        // Measure the window length.
        let cur_len = rht - lft + 1;

        // Determine maximum window length.
        if cur_len > max_len {
            max_len = cur_len;
        }
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::length_of_longest_substring;

    #[test]
    fn empty_string() {
        let s = String::from("");
        assert_eq!(length_of_longest_substring(s), 0);
    }

    #[test]
    fn single_character() {
        let s = String::from("a");
        assert_eq!(length_of_longest_substring(s), 1);
    }

    #[test]
    fn all_unique() {
        let s = String::from("abcdef");
        assert_eq!(length_of_longest_substring(s), 6);
    }

    #[test]
    fn repeated_characters() {
        let s = String::from("abba");
        assert_eq!(length_of_longest_substring(s), 2);
    }

    #[test]
    fn typical_case() {
        let s = String::from("pwwkew");
        assert_eq!(length_of_longest_substring(s), 3);
    }

    #[test]
    fn numeric_characters() {
        let s = String::from("1231234");
        assert_eq!(length_of_longest_substring(s), 4);
    }

    #[test]
    fn special_characters() {
        let s = String::from("!@# !@#");
        assert_eq!(length_of_longest_substring(s), 4);
    }

    #[test]
    fn long_string() {
        let s = "a".repeat(10000);
        assert_eq!(length_of_longest_substring(s), 1);
    }
}
