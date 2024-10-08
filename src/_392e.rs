// Time complexity: O(n), n is the combined length of string s and t.
// Space complexity: O(n), n is the combined length of string s and t.
pub fn is_subsequence(s: String, t: String) -> bool {
    // Given two string s and t.
    // Return true if s is a subsequence of t.
    // Use a two-pointer technique.
    // Move from start for each string.

    // Create arrays for indexing.
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    // Initialize variables.
    let mut lft: usize = 0;
    let mut rht: usize = 0;

    while lft < s.len() && rht < t.len() {
        if s[lft] == t[rht] {
            // Move left pointer.
            lft += 1;
        }
        // Always move right pointer.
        rht += 1;
    }

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
