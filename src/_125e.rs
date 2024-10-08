pub fn is_palindrome(s: String) -> bool {
    // Given a string with alphanumeric, punctuation and mixed case.
    // Remove punctuation and mixed case.
    // Determine whether is palindrome.
    // Return true if palindrome.
    // Use two-pointer technique.

    // Filter and transform string for comparison.
    let chrs: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    // Check input minium edge case.
    if chrs.len() <= 1 {
        return true;
    }

    // Initialize variables.
    let mut lft: usize = 0;
    let mut rht: usize = chrs.len() - 1;

    // Loop through characters.
    while lft < rht {
        // Check palindrome condition.
        if chrs[lft] != chrs[rht] {
            return false;
        }
        lft += 1;
        rht -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_palindrome_simple() {
        assert!(is_palindrome("A man, a plan, a canal: Panama".to_string()));
    }

    #[test]
    fn valid_palindrome_single_char() {
        assert!(is_palindrome("a".to_string()));
    }

    #[test]
    fn valid_palindrome_single_char_and_period() {
        assert!(is_palindrome("a.".to_string()));
    }

    #[test]
    fn invalid_palindrome_with_non_alpha() {
        assert!(!is_palindrome("race a car".to_string()));
    }

    #[test]
    fn valid_palindrome_empty_string() {
        assert!(is_palindrome("".to_string()));
    }

    #[test]
    fn valid_palindrome_numbers() {
        assert!(is_palindrome("12321".to_string()));
    }

    #[test]
    fn invalid_palindrome_case_sensitive() {
        assert!(is_palindrome("No lemon, no melon.".to_string()));
    }
}
