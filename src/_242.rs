// Time complexity: O(n), n is the length of the strings.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/67096707-2c54-8002-80ac-d4520a188ce5
pub fn is_anagram(s: String, t: String) -> bool {
    // Valid anagram
    // Given two string s and t.
    // Determine if t is an anagram of s.
    // Return true if condition met.
    // Use a frequency counting approach.
    // Use a fixed-size array due to 26 character alphabet.

    // Check input length condition.
    // Anagrams have equal length.
    if s.len() != t.len() {
        return false;
    }

    // Create a single frequency map.
    let mut cnts: [i32; 26] = [0; 26];

    // Iterate through each character.
    for idx in 0..s.len() {
        // Calculate the array index for the s character.
        let s_idx: usize = (s.as_bytes()[idx] - b'a') as usize;
        cnts[s_idx] += 1; // Increment count for `s` char.

        // Calculate array index for the t character.
        let t_idx: usize = (t.as_bytes()[idx] - b'a') as usize;
        cnts[t_idx] -= 1; // Decrement count for `t` char.
    }

    // Verify equal character counts.
    for cnt in cnts {
        if cnt != 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn letters_match() {
        let s: String = String::from("anagram");
        let t: String = String::from("nagaram");
        assert!(is_anagram(s, t));
    }

    #[test]
    fn letters_do_not_match() {
        let s: String = String::from("rat");
        let t: String = String::from("car");
        assert!(!is_anagram(s, t));
    }

    #[test]
    fn both_empty() {
        let s: String = String::from("");
        let t: String = String::from("");
        assert!(is_anagram(s, t));
    }

    #[test]
    fn single_character_match() {
        let s: String = String::from("a");
        let t: String = String::from("a");
        assert!(is_anagram(s, t));
    }

    #[test]
    fn single_character_mismatch() {
        let s: String = String::from("a");
        let t: String = String::from("b");
        assert!(!is_anagram(s, t));
    }

    #[test]
    fn unequal_length_strings() {
        let s: String = String::from("a");
        let t: String = String::from("ab");
        assert!(!is_anagram(s, t));
    }
}
