/// 205. Isomorphic Strings
///
/// Given two strings s and t, determine if they
/// are isomorphic.
///
/// Two strings s and t are isomorphic if the characters
/// in s can be replaced to get t.
///
/// All occurrences of a character must be replaced
/// with another character while preserving the order
/// of characters. No two characters may map to the
/// same character, but a character may map to itself.
///
/// Constraints:
/// * 1 <= s.length <= 5 * 10^4
/// * t.length == s.length
/// * s and t consist of any valid ascii character.

fn is_isomorphic(s: String, t: String) -> bool {
    // Map each character in s to t.
    // Map each character in t to s.
    // Return false, if more than one mapping is attempted.
    // Time complexity: O(n).
    //  - n is the length of both string s and string t.
    // Space complexity: O(1).
    //  - Constant space dues to 128 ASCII letters.
    const CHR_LEN: usize = 128;

    let mut s_to_t: [Option<u8>; CHR_LEN] = [None; CHR_LEN];
    let mut t_to_s: [Option<u8>; CHR_LEN] = [None; CHR_LEN];

    for (&s_chr, &t_chr) in s.as_bytes().iter().zip(t.as_bytes()) {
        if let Some(t_map) = s_to_t[s_chr as usize] {
            if t_map != t_chr {
                return false;
            }
        } else {
            s_to_t[s_chr as usize] = Some(t_chr)
        }

        if let Some(s_map) = t_to_s[t_chr as usize] {
            if s_map != s_chr {
                return false;
            }
        } else {
            t_to_s[t_chr as usize] = Some(s_chr);
        }
    }

    true
}

fn is_isomorphic_c(s: String, t: String) -> bool {
    // Map each character from s to t.
    // Map each character from t to s.
    // Return false if a character has already been mapped.
    // Use an array as a map and perfect hash.
    // The perfect hash is to use the character digit as the key.
    // Time complexity: O(n). n is the length of both string s and t.
    // Space complexity: O(1). Constant space due to using 128 ASCII characters.
    const CHR_LEN: usize = 128;

    let mut s_to_t: [Option<u8>; CHR_LEN] = [None; CHR_LEN];
    let mut t_to_s: [Option<u8>; CHR_LEN] = [None; CHR_LEN];

    for (&s_chr, &t_chr) in s.as_bytes().iter().zip(t.as_bytes()) {
        if let Some(t_map) = s_to_t[s_chr as usize] {
            //  Check whether s to t isomorphic constraint is met.
            if t_map != t_chr {
                return false;
            }
        } else {
            // Map s character to t character.
            s_to_t[s_chr as usize] = Some(t_chr);
        }

        if let Some(s_map) = t_to_s[t_chr as usize] {
            //  Check whether t to s isomorphic constraint is met.
            if s_map != s_chr {
                return false;
            }
        } else {
            // Map s character to t character.
            t_to_s[t_chr as usize] = Some(s_chr);
        }
    }

    true
}

fn is_isomorphic_b(s: String, t: String) -> bool {
    // Use character digit value as key to array map.
    const CHR_LEN: usize = 128;

    // Create character maps.
    let mut s_to_t: [Option<u8>; CHR_LEN] = [None; CHR_LEN];
    let mut t_to_s: [Option<u8>; CHR_LEN] = [None; CHR_LEN];

    // Loop through each character simultaneously.
    // Contributes to O(n) time complexity.
    for (&s_chr, &t_chr) in s.as_bytes().iter().zip(t.as_bytes().iter()) {
        if let Some(t_map) = s_to_t[s_chr as usize] {
            if t_map != t_chr {
                return false;
            }
        } else {
            s_to_t[s_chr as usize] = Some(t_chr)
        }

        if let Some(s_map) = t_to_s[t_chr as usize] {
            if s_map != s_chr {
                return false;
            }
        } else {
            t_to_s[t_chr as usize] = Some(s_chr);
        }
    }

    true
}

fn is_isomorphic_a(s: String, t: String) -> bool {
    use std::collections::HashMap;

    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        if let Some(&mapped_t) = s_to_t.get(&sc) {
            if mapped_t != tc {
                return false;
            }
        } else {
            s_to_t.insert(sc, tc);
        }

        if let Some(&mapped_s) = t_to_s.get(&tc) {
            if mapped_s != sc {
                return false;
            }
        } else {
            t_to_s.insert(tc, sc);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::is_isomorphic;

    #[test]
    fn test_basic_case_1() {
        assert_eq!(is_isomorphic("egg".to_string(), "add".to_string()), true);
    }

    #[test]
    fn test_basic_case_2() {
        assert_eq!(is_isomorphic("foo".to_string(), "bar".to_string()), false);
    }

    #[test]
    fn test_all_unique() {
        assert_eq!(is_isomorphic("abcd".to_string(), "wxyz".to_string()), true);
    }

    #[test]
    fn test_repeating_characters() {
        assert_eq!(is_isomorphic("abab".to_string(), "cdcd".to_string()), true);
    }

    #[test]
    fn test_repeating_characters_mismatch() {
        assert_eq!(is_isomorphic("abab".to_string(), "cddc".to_string()), false);
    }

    #[test]
    fn test_empty_strings() {
        assert_eq!(is_isomorphic("".to_string(), "".to_string()), true);
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(is_isomorphic("!@#".to_string(), "$%^".to_string()), true);
    }

    #[test]
    fn test_special_characters_mismatch() {
        assert_eq!(is_isomorphic("!@#".to_string(), "$$$".to_string()), false);
    }

    #[test]
    fn test_mixed_characters() {
        assert_eq!(is_isomorphic("a1b2".to_string(), "x9y8".to_string()), true);
    }
}
