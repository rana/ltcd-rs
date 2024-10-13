// Time complexity: O(n), n is the length of the strings. We iterate over each string once.
// Space complexity: O(1), constant additional space used. Constant space based on ASCII character set used in HashMaps.
// https://chatgpt.com/c/67095c50-dbb4-8002-afa4-bf93a04f87a9
pub fn is_isomorphic(s: String, t: String) -> bool {
    // Given two strings.
    // Determine if the strings have a one-to-one isomorphic mapping.
    // Return true if one-to-one.
    // Use two HashMaps.

    use std::collections::HashMap;

    // Create two hashmaps for bidirectional mapping.
    let mut s_to_t: HashMap<char, char> = HashMap::new();
    let mut t_to_s: HashMap<char, char> = HashMap::new();

    // Iterate over both strings simultaneously.
    for (ch_s, ch_t) in s.chars().zip(t.chars()) {
        // s to t: check and update mapping.
        match s_to_t.get(&ch_s) {
            Some(&mapped_ch_t) => {
                if mapped_ch_t != ch_t {
                    return false;
                }
            }
            None => {
                s_to_t.insert(ch_s, ch_t);
            }
        }

        // t to s: check and update mapping.
        match t_to_s.get(&ch_t) {
            Some(&mapped_ch_s) => {
                if mapped_ch_s != ch_s {
                    return false;
                }
            }
            None => {
                t_to_s.insert(ch_t, ch_s);
            }
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
