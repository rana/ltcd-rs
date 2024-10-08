// Time complexity: O(n), n is the length of string s. We traverse each character once.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/6705a06b-4f44-8002-a2a6-882eabc1c45e
pub fn is_palindrome(s: String) -> bool {
    // Use a two-pointer technique.

    // Filter and transform string.
    let chrs: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    // Check input minimum edge condition.
    if chrs.len() <= 1 {
        return true;
    }

    // Initialize variables.
    // Use isize to support reverse iteration.
    let mut lft = 0;
    let mut rht = chrs.len() as isize - 1;

    // Loop until two pointers meet.
    while lft < rht as usize {
        // Check palindrome condition.
        if chrs[lft] != chrs[rht as usize] {
            return false;
        }

        // Progess pointers.
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
