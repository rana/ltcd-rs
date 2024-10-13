pub fn is_anagram(s: String, t: String) -> bool {
    // Given two string s and t.
    // Determine if t is an anagram of s.
    // Return true if condition is met.
    // Character set is 26 lowercase English letters.
    // Use a fixed-size frequency counter map.
    // Use a single frequency counter map.
    // Success condition when frequency counters are all zero.

    // Check input string lengths equal.
    // Condition of anagram.
    if s.len() != t.len() {
        return false;
    }

    // Create a fixed-size frequency counter map.
    let mut cnts: [i32; 26] = [0; 26];

    // Iterate through each character simultaneously.
    for (s_chr, t_chr) in s.chars().zip(t.chars()) {
        // Create map index for s character.
        let idx: usize = (s_chr as u8 - b'a') as usize;
        // Increment frequency count of s characters.
        cnts[idx] += 1;

        // Create map index for t character.
        let idx: usize = (t_chr as u8 - b'a') as usize;
        // Decrement frequency count for t characters.
        cnts[idx] -= 1;
    }

    // Validate anagram condition.
    if cnts.iter().any(|&cnt| cnt < 0) {
        return false;
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
