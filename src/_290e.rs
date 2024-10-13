pub fn word_pattern(ptn: String, s_str: String) -> bool {
    // Word Pattern
    // Given strings `pattern` and `s_str`.
    // Determine whether there is bijection between
    // characters from `pattern` and words in `s_str`.
    // Bijection = isomorphic = one-to-one mapping.
    // Use two HashMaps to evaluate success condition.
    // Return true if sucess condition met.

    use std::collections::HashMap;

    // Split string to words.
    let wrds: Vec<&str> = s_str.split_whitespace().collect();

    // Check for equal lengths.
    // Condition of bijection.
    if ptn.len() != wrds.len() {
        return false;
    }

    // Initialize two HashMaps.
    let mut chr_to_wrd: HashMap<char, &str> = HashMap::new();
    let mut wrd_to_chr: HashMap<&str, char> = HashMap::new();

    // Iterate through each character and word simultaneously.
    for (chr, wrd) in ptn.chars().zip(wrds.iter()) {
        // chr to wrd: check and insert mapping.
        if let Some(mapped_wrd) = chr_to_wrd.get(&chr) {
            // Check failure condition.
            if mapped_wrd != wrd {
                return false;
            }
        } else {
            chr_to_wrd.insert(chr, wrd);
        }

        // wrd to chr: check and insert mapping.
        if let Some(&mapped_chr) = wrd_to_chr.get(wrd) {
            if mapped_chr != chr {
                return false;
            }
        } else {
            wrd_to_chr.insert(wrd, chr);
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
