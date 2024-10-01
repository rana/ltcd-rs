/// 3. Longest Substring Without Repeating Characters
///
/// Given a string s, find the length of the longest
/// substring without repeating characters.
///
/// Constraints:
/// * 0 <= s.length <= 5 * 10^4
/// * s consists of English letters,
/// digits, symbols and spaces.

fn length_of_longest_substring(s: String) -> i32 {
    // Variables contribute to O(1) space complexity.
    let mut map: [usize; 128] = [0; 128];
    let mut idx_fst: usize = 0;
    let mut max_len: usize = 0;

    // Loop contributes to O(n) time complexity.
    for (idx, chr) in s.chars().enumerate() {
        // Lookup the last known position
        // of the current character.
        idx_fst = idx_fst.max(map[chr as usize]);

        // Calculate the length of the current substring.
        max_len = max_len.max(idx - idx_fst + 1);

        // Update the current character position.
        // Increment by one sets it to a starting index.
        map[chr as usize] = idx + 1;
    }

    max_len as i32
}

fn length_of_longest_substring_b(s: String) -> i32 {
    use std::collections::HashMap;

    let mut char_indices = HashMap::new();
    let (mut max_len, mut start) = (0, 0);

    for (index, ch) in s.chars().enumerate() {
        if let Some(prev_index) = char_indices.get(&ch) {
            start = start.max(prev_index + 1);
        }
        max_len = max_len.max(index - start + 1);
        char_indices.insert(ch, index);
    }

    max_len as i32
}

fn length_of_longest_substring_a(s: String) -> i32 {
    let mut max_len: usize = 0;

    let mut pos: [usize; 128] = [0; 128];
    let mut fst: usize = 0;
    for (lst, chr) in s.chars().enumerate() {
        fst = fst.max(pos[chr as usize]);
        max_len = max_len.max(lst - fst + 1);
        pos[chr as usize] = lst + 1;
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn test_all_same_characters() {
        assert_eq!(length_of_longest_substring("aaaa".to_string()), 1);
    }

    #[test]
    fn test_no_repeats() {
        assert_eq!(length_of_longest_substring("abc".to_string()), 3);
    }

    #[test]
    fn test_with_repeats() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_long_string_with_repeats() {
        assert_eq!(length_of_longest_substring("abba".to_string()), 2);
    }

    #[test]
    fn test_string_with_non_alphanumeric_characters() {
        assert_eq!(length_of_longest_substring("a!b!c!".to_string()), 3);
    }

    #[test]
    fn test_string_with_spaces() {
        assert_eq!(length_of_longest_substring("pwwkew ".to_string()), 4);
    }
}
