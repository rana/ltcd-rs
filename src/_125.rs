/// 125. Valid Palindrome
///
/// A phrase is a palindrome if, after converting all uppercase
/// letters into lowercase letters and removing all
/// non-alphanumeric characters, it reads the same forward
/// and backward. Alphanumeric characters include letters
/// and numbers.
///
/// Given a string s, return true if it is a palindrome,
/// or false otherwise.
///
/// Constraints:
/// * 1 <= s.length <= 2 * 10^5
/// * s consists only of printable ASCII characters.

fn is_palindrome(s: String) -> bool {
    // Pre-process string `s` to enable character comparisons.
    // Pre-processing contributes to O(n) time complexity.
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>();

    // Check for the minimum edge case.
    if s.len() <= 1 {
        return true;
    }

    // Use two pointers to compare a back element and
    // a front element of the array.
    // Variables contribute to O(1) space complexity.
    let mut lft: usize = 0;
    let mut rht: usize = s.len() - 1;

    // Loop through characters and compare back to front.
    // Loop contributes to O(n) time complexity.
    while lft < rht {
        // Compare the front element to the back element.
        if s[lft] != s[rht] {
            return false;
        }
        // Advance each pointer.
        lft += 1;
        rht -= 1;
    }

    // We have a valid palindrome at this point.
    true
}

fn is_palindrome_b(s: String) -> bool {
    let s = s
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect::<Vec<_>>();

    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        if s[left] != s[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

fn is_palindrome_a(s: String) -> bool {
    let mut s = s.to_uppercase();
    s.retain(|c| (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9'));
    let byts = s.as_bytes();
    if byts.is_empty() {
        return true;
    }
    let mut lft: usize = 0;
    let mut rht: usize = byts.len() - 1;
    while lft < rht {
        if byts[lft] != byts[rht] {
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
    fn test_single_character() {
        assert!(is_palindrome("a".to_string()));
    }

    #[test]
    fn test_single_space() {
        assert!(is_palindrome(" ".to_string()));
    }

    #[test]
    fn test_simple_palindrome() {
        assert!(is_palindrome("A man a plan a canal Panama".to_string()));
    }

    #[test]
    fn test_simple_non_palindrome() {
        assert!(!is_palindrome("hello".to_string()));
    }

    #[test]
    fn test_complex_palindrome() {
        assert!(is_palindrome("No lemon, no melon".to_string()));
    }

    #[test]
    fn test_numerical_palindrome() {
        assert!(is_palindrome("12321".to_string()));
    }

    #[test]
    fn test_numerical_non_palindrome() {
        assert!(!is_palindrome("123456".to_string()));
    }

    #[test]
    fn test_mixed_characters() {
        assert!(is_palindrome(
            "A Toyota! Race fast, safe car: a Toyota.".to_string()
        ));
    }

    #[test]
    fn test_with_spaces_and_punctuation() {
        assert!(is_palindrome("Was it a car or a cat I saw?".to_string()));
    }

    #[test]
    fn test_case_sensitivity() {
        assert!(is_palindrome("Madam".to_string()));
    }
}
