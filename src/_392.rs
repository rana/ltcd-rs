// Time complexity: O(n), n is the combined length of strings s and t.
// Space complexity: O(n), n is the combined length of strings s and t.
// https://chatgpt.com/c/6705b31c-2a28-8002-8f94-497aa28ff206
pub fn is_subsequence(s: String, t: String) -> bool {
    // Use a two-pointer technique.
    // Traverse from start of each string.

    // Initialize variables.
    let mut lft: usize = 0;
    let mut rht: usize = 0;
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    while lft < s.len() && rht < t.len() {
        if s[lft] == t[rht] {
            // Move lft if there's a match.
            lft += 1;
        }
        // Always move rht.
        rht += 1;
    }

    // Check success condition.
    lft == s.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_s() {
        // Test case where s is empty
        assert_eq!(is_subsequence("".to_string(), "abc".to_string()), true);
    }

    #[test]
    fn empty_t() {
        // Test case where t is empty
        assert_eq!(is_subsequence("abc".to_string(), "".to_string()), false);
    }

    #[test]
    fn s_is_subsequence_of_t() {
        // Valid case where s is a subsequence of t
        assert_eq!(
            is_subsequence("abc".to_string(), "ahbgdc".to_string()),
            true
        );
    }

    #[test]
    fn s_is_not_subsequence_of_t() {
        // Invalid case where s is not a subsequence of t
        assert_eq!(
            is_subsequence("axc".to_string(), "ahbgdc".to_string()),
            false
        );
    }

    #[test]
    fn identical_strings() {
        // Test case where s and t are identical
        assert_eq!(is_subsequence("abc".to_string(), "abc".to_string()), true);
    }

    #[test]
    fn s_longer_than_t() {
        // Invalid case where s is longer than t
        assert_eq!(is_subsequence("abcd".to_string(), "abc".to_string()), false);
    }
}
