
// Time complexity: O(n), n is the length of characters in s.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/670961e8-7a2c-8002-aaee-dfe79ed59359
pub fn word_pattern(pat: String, s_str: String) -> bool {
    // Word pattern
    // Given a string `pattern`.
    // Given a string `s` with spaced words.
    // Determine whether a bijective relation
    // between character and word exists.
    // Return true if condition met.
    // Use two HashMaps.
    
    use std::collections::HashMap;
    
    // Split string to words.
    let wrds: Vec<&str> = s_str.split_whitespace().collect();

    // Check matching lengths
    if pat.len() != wrds.len() {
        return false;
    }

    // Initialize two HashMaps for bijection.
    let mut chr_map: HashMap<char, &str> = HashMap::new();
    let mut wrd_map: HashMap<&str, char> = HashMap::new();

    // Iterate through pattern characters and corresponding words.
    for (chr, wrd) in pat.chars().zip(wrds.iter()) {
        // chr to wrd: check and update mapping.
        if let Some(&mapped_wrd) = chr_map.get(&chr) {
            if mapped_wrd != *wrd {
                return false;
            }
        } else {
            chr_map.insert(chr, wrd);
        }

        // wrd to chr: check and update mapping.
        if let Some(&mapped_chr) = wrd_map.get(wrd) {
            if mapped_chr != chr {
                return false;
            }
        } else {
            wrd_map.insert(wrd, chr);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ptn_wrd_match_simple() {
        let pat: String = "abba".to_string();
        let s_str: String = "dog cat cat dog".to_string();
        assert!(word_pattern(pat, s_str));
    }

    #[test]
    fn ptn_wrd_mismatch_length() {
        let pat: String = "abba".to_string();
        let s_str: String = "dog cat cat".to_string();
        assert!(!word_pattern(pat, s_str));
    }

    #[test]
    fn ptn_wrd_no_bijection() {
        let pat: String = "abba".to_string();
        let s_str: String = "dog cat cat fish".to_string();
        assert!(!word_pattern(pat, s_str));
    }

    #[test]
    fn ptn_wrd_single_char() {
        let pat: String = "a".to_string();
        let s_str: String = "dog".to_string();
        assert!(word_pattern(pat, s_str));
    }

    #[test]
    fn ptn_wrd_multiple_bijections() {
        let pat: String = "abcabc".to_string();
        let s_str: String = "one two three one two three".to_string();
        assert!(word_pattern(pat, s_str));
    }
}
