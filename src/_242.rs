/// 242. Valid Anagram
///
/// Given two strings s and t, return true if t is an
/// anagram of s, and false otherwise.
///
/// An Anagram is a word or phrase formed by rearranging
/// the letters of a different word or phrase, typically
/// using all the original letters exactly once.
///
/// Constraints:
/// * 1 <= s.length, t.length <= 5 * 10^4
/// * s and t consist of lowercase English letters.

fn is_anagram(s: String, t: String) -> bool {
    // Check if lengths are the same.
    // Check if character frequencies are equal.
    // Use a perfect hash by subtracting b'a'.
    // Use a single array.
    // Increment character frequencies for one string.
    // Decrement character frequencies for the other string.
    // Compare the frequency counts.
    // Time complexity: O(n).
    //  - n is the length of strings s and t.
    // Space complexity: O(1)
    //  - The hash map uses a constant number of characters.
    //  - 26 characters are used as keys defined by the constraint.
    if s.len() != t.len() {
        return false;
    }

    const CHR_LEN: usize = 26;
    const CHR_FST: u8 = b'a';
    let mut chr_cnts: [i32; CHR_LEN] = [0; CHR_LEN];

    // Increment character frequency of string s.
    // Contributes to O(n) time complexity.
    for &byt_chr in s.as_bytes() {
        let idx = (byt_chr - CHR_FST) as usize;
        chr_cnts[idx] += 1;
    }

    // Decrement character frequency of string t.
    // Contributes to O(n) time complexity.
    for &byt_chr in t.as_bytes() {
        let idx = (byt_chr - CHR_FST) as usize;
        chr_cnts[idx] -= 1;
        // Check for non-equality condition.
        if chr_cnts[idx] < 0 {
            return false;
        }
    }

    true
}

fn is_anagram_b(s: String, t: String) -> bool {
    const CHR_LEN: usize = 26;
    const CHR_FST: u8 = b'a';

    // Check for string length equality.
    // Anagrams have equal lengths.
    if s.len() != t.len() {
        return false;
    }

    // Use i32 to allow negative count.
    // i32 is large enough to count all characters.
    // i32::MIN -2147483648 < t.length 50,000 < i32::MAX 2147483647
    let mut chr_cnts: [i32; CHR_LEN] = [0; CHR_LEN];

    // Increment character frequency for string `s`.
    // Contributes to O(n) time complexity.
    for &byt_chr in s.as_bytes() {
        chr_cnts[(byt_chr - CHR_FST) as usize] += 1;
    }

    // Decrement character frequency for string `t`.
    // Contributes to O(n) time complexity.
    for &byt_chr in t.as_bytes() {
        let idx = (byt_chr - CHR_FST) as usize;
        chr_cnts[idx] -= 1;
        if chr_cnts[idx] < 0 {
            return false;
        }
    }

    true
}

fn is_anagram_a(s: String, t: String) -> bool {
    use std::collections::HashMap;

    if s.len() != t.len() {
        return false;
    }

    let mut char_count: HashMap<char, i32> = HashMap::new();

    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    for ch in t.chars() {
        let count = char_count.entry(ch).or_insert(0);
        *count -= 1;
        if *count < 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_anagram;

    #[test]
    fn test_basic_anagrams() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn test_empty_strings() {
        assert!(is_anagram("".to_string(), "".to_string()));
        assert!(!is_anagram("".to_string(), "a".to_string()));
    }

    #[test]
    fn test_different_lengths() {
        assert!(!is_anagram("short".to_string(), "longer".to_string()));
    }
}
