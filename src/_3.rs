// Time complexity: O(n), n is the length of the string s. We traverse s once.
// Space complexity: O(min(m, n)), m is the size of the character set. We store characters in a HashMap.
// https://chatgpt.com/c/6706d5f9-93b8-8002-bdb5-62ecc38bb2da
pub fn length_of_longest_substring(s: String) -> i32 {
    // Given a string s.
    // Determine the max length of a substring.
    // Repeating characters not allowed.
    // Return the max length of the substring.
    // Use a sliding window two-pointer technique.

    use std::collections::HashMap;

    // Map to store last index of each character.
    let mut ch_idx = HashMap::new();
    // Max length of substring without repeating characters.
    let mut max_len: usize = 0;
    // Left index of sliding window.
    let mut lft: usize = 0;

    // Loop through each character of the string.
    for (rht, ch) in s.chars().enumerate() {
        // Check whether ch is in the window.
        if let Some(&prv_idx) = ch_idx.get(&ch) {
            if prv_idx >= lft {
                // Move lft pointer after right-most repeating char.
                lft = prv_idx + 1;
            }
        }

        // Update the character's last index.
        ch_idx.insert(ch, rht);

        // Determine current window length.
        let cur_len = rht - lft + 1;

        // Determine the maximum window length.
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
