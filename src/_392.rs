/// 392. Is Subsequence
///
/// Given two strings s and t, return true if s is a
/// subsequence of t, or false otherwise.
///
/// A subsequence of a string is a new string that is
/// formed from the original string by deleting some
/// (can be none) of the characters without disturbing
/// the relative positions of the remaining characters.
/// (i.e., "ace" is a subsequence of "abcde" while
/// "aec" is not).
///
/// Constraints:
/// * 0 <= s.length <= 100
/// * 0 <= t.length <= 10^4
/// * s and t consist only of lowercase English letters.

fn is_subsequence(s: String, t: String) -> bool {
    // We're looking for `s` in `t`.

    // Variables contribute to O(1) space complexity.
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut s_idx: usize = 0;
    let mut t_idx: usize = 0;
    let s_len = s.len();
    let t_len = t.len();

    // Loop through string `s` and string `t`.
    // Loop contributes to O(n) time complexity.
    while s_idx < s_len && t_idx < t_len {
        // Looking for subsequence.

        // Check for character equality.
        if s[s_idx] == t[t_idx] {
            s_idx += 1;
        }

        t_idx += 1;
    }

    s_idx == s.len()
}

fn is_subsequence_b(s: String, t: String) -> bool {
    let mut s_ptr = 0;
    let mut t_ptr = 0;
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();

    while t_ptr < t.len() && s_ptr < s.len() {
        if s_bytes[s_ptr] == t_bytes[t_ptr] {
            s_ptr += 1;
        }
        t_ptr += 1;
    }

    s_ptr == s.len()
}

fn is_subsequence_a(s: String, t: String) -> bool {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut lft: usize = 0;
    let mut rht: usize = 0;
    let lft_lim = s.len();
    let rht_lim = t.len();
    while lft < lft_lim && rht < rht_lim {
        if s[lft] == t[rht] {
            lft += 1;
        }
        rht += 1;
    }

    lft == lft_lim
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_true() {
        assert!(is_subsequence("abc".to_string(), "ahbgdc".to_string()));
    }

    #[test]
    fn test_basic_false() {
        assert!(!is_subsequence("axc".to_string(), "ahbgdc".to_string()));
    }

    #[test]
    fn test_empty_s() {
        assert!(is_subsequence("".to_string(), "ahbgdc".to_string()));
    }

    #[test]
    fn test_empty_t() {
        assert!(!is_subsequence("abc".to_string(), "".to_string()));
    }

    #[test]
    fn test_both_empty() {
        assert!(is_subsequence("".to_string(), "".to_string()));
    }

    #[test]
    fn test_same_strings() {
        assert!(is_subsequence("abc".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_s_longer_than_t() {
        assert!(!is_subsequence("abcdef".to_string(), "abc".to_string()));
    }

    #[test]
    fn test_repeated_characters() {
        assert!(is_subsequence("aaa".to_string(), "abacadaea".to_string()));
    }

    #[test]
    fn test_no_common_characters() {
        assert!(!is_subsequence("xyz".to_string(), "abacadaea".to_string()));
    }

    #[test]
    fn test_s_equal_t_but_different_order() {
        assert!(!is_subsequence("bac".to_string(), "abc".to_string()));
    }
}
