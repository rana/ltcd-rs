fn str_str(haystack: String, needle: String) -> i32 {
    // Compute the longest prefix suffix (LPS) array.
    fn compute_lps(needle: &[u8], lps: &mut [usize]) {
        let mut length = 0;
        let mut i = 1;
        lps[0] = 0; // lps[0] is always 0

        while i < needle.len() {
            if needle[i] == needle[length] {
                length += 1;
                lps[i] = length;
                i += 1;
            } else if length != 0 {
                length = lps[length - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    // Edge case for an empty pattern.
    if needle.is_empty() {
        return 0;
    }

    let (haystack, needle) = (haystack.as_bytes(), needle.as_bytes());
    let (h_len, n_len) = (haystack.len(), needle.len());
    let mut lps = vec![0; n_len];

    // Build the partial match table.
    compute_lps(needle, &mut lps);

    let mut h = 0; // index for haystack
    let mut n = 0; // index for needle

    // Search for the substring.
    while h < h_len {
        // When characters match, move to the next characters.
        if needle[n] == haystack[h] {
            h += 1;
            n += 1;
        }

        // If we've matched the entire pattern, return the start index.
        if n == n_len {
            return (h - n) as i32;
        }

        if h < h_len && needle[n] != haystack[h] {
            if n != 0 {
                // Mismatch after n matches,
                // use the `lps` table to skip characters in the pattern.
                n = lps[n - 1];
            } else {
                // Mismatch at the first character of the pattern,
                // move to the next character in the text.
                h += 1;
            }
        }
    }

    // No match found.
    -1
}

fn str_str_a(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        None => -1,
        Some(idx) => idx as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_needle() {
        assert_eq!(str_str("hello".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_empty_haystack() {
        assert_eq!(str_str("".to_string(), "hello".to_string()), -1);
    }

    #[test]
    fn test_empty_haystack_and_needle() {
        assert_eq!(str_str("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn test_needle_not_in_haystack() {
        assert_eq!(str_str("hello".to_string(), "world".to_string()), -1);
    }

    #[test]
    fn test_needle_at_beginning() {
        assert_eq!(str_str("hello".to_string(), "hel".to_string()), 0);
    }

    #[test]
    fn test_needle_at_end() {
        assert_eq!(str_str("hello".to_string(), "llo".to_string()), 2);
    }

    #[test]
    fn test_needle_is_haystack() {
        assert_eq!(str_str("hello".to_string(), "hello".to_string()), 0);
    }

    #[test]
    fn test_longer_needle_than_haystack() {
        assert_eq!(str_str("hi".to_string(), "hello".to_string()), -1);
    }

    #[test]
    fn test_common_use_case() {
        assert_eq!(str_str("abcde".to_string(), "cd".to_string()), 2);
    }

    #[test]
    fn test_abc() {
        assert_eq!(str_str("ABC ABCDAB ABCDABCDABDE".to_string(), "ABCDABD".to_string()), 2);
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(str_str("abcdefg".to_string(), "hij".to_string()), -1);
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(str_str("abababab".to_string(), "aba".to_string()), 0);
    }

    #[test]
    fn test_long_haystack_short_needle() {
        assert_eq!(str_str("the quick brown fox jumps over the lazy dog".to_string(), "fox".to_string()), 16);
    }

    #[test]
    fn test_needle_occurs_multiple_times() {
        assert_eq!(str_str("mississippi".to_string(), "iss".to_string()), 1);
    }

    #[test]
    fn test_end_of_haystack() {
        assert_eq!(str_str("hello world".to_string(), "world".to_string()), 6);
    }
}
