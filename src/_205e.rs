// Time complexity: O(n), n is the length of strings s and t. Each string is iterated once.
// Space complexity: O(1), constant space used. Fized-size character set used in HashMaps.
pub fn is_isomorphic(s: String, t: String) -> bool {
    // Isomorphic Strings
    // Given two string s and t.
    // Determine if the two strings are isomorphic.
    // Same as bijective one-to-one mapping of each character
    // between strings.
    // Characters are ASCII.
    // Strings have equal lengths.
    // Use two HashMaps to determine success condition.

    use std::collections::HashMap;

    // Initialize two HashMaps.
    let mut s_to_t: HashMap<char, char> = HashMap::new();
    let mut t_to_s: HashMap<char, char> = HashMap::new();

    // Iterate through each character simultaneously.
    for (chr_s, chr_t) in s.chars().zip(t.chars()) {
        // s to t: check and insert mapping.
        if let Some(&mapped_chr_t) = s_to_t.get(&chr_s) {
            // Check failure condition.
            if mapped_chr_t != chr_t {
                return false;
            }
        } else {
            s_to_t.insert(chr_s, chr_t);
        }

        // t to s: check and insert mapping.
        if let Some(&mapped_chr_s) = t_to_s.get(&chr_t) {
            // Check failure condition.
            if mapped_chr_s != chr_s {
                return false;
            }
        } else {
            t_to_s.insert(chr_t, chr_s);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_example1() {
        let s: String = "egg".to_string();
        let t: String = "add".to_string();
        assert!(is_isomorphic(s, t));
    }

    #[test]
    fn valid_example2() {
        let s: String = "paper".to_string();
        let t: String = "title".to_string();
        assert!(is_isomorphic(s, t));
    }

    #[test]
    fn invalid_example1() {
        let s: String = "foo".to_string();
        let t: String = "bar".to_string();
        assert!(!is_isomorphic(s, t));
    }

    #[test]
    fn invalid_example2() {
        let s: String = "ab".to_string();
        let t: String = "aa".to_string();
        assert!(!is_isomorphic(s, t));
    }
}
