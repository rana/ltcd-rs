// Time complexity: O(n), n is the length of string s. We traverse each character of string s.
// Space complexity: O(n), up to n characters are placed on a stack.
// https://chatgpt.com/c/670d895b-4e7c-8002-94cb-39d15e19eadc
pub fn is_valid(s: String) -> bool {
    // Valid Parenthesis
    // Given a string s.
    // s contains only characters: (){}[]
    // Valid conditions:
    //  * Matching type of bracket for open and close.
    //  * Proper ordering of bracket for open and close.
    // Determine whether string is valid.
    // Return true if conditions met.
    // Use a stack to track bracket pairs.
    // Push closing bracket when open bracket found.

    let mut stk: Vec<char> = Vec::new();

    // Loop through each character.
    for chr in s.chars() {
        match chr {
            // Found open bracket: push close bracket.
            '(' => stk.push(')'),
            '[' => stk.push(']'),
            '{' => stk.push('}'),
            // Found close bracket. Check for close bracket on stack top.
            ')' | ']' | '}' => {
                if stk.pop() != Some(chr) {
                    return false;
                }
            }
            // Found invalid character.
            _ => return false,
        }
    }

    // Empty stack indicates success condition.
    stk.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_simple() {
        let s: String = String::from("()");
        assert!(is_valid(s));
    }

    #[test]
    fn valid_nested() {
        let s: String = String::from("({[]})");
        assert!(is_valid(s));
    }

    #[test]
    fn invalid_unmatched() {
        let s: String = String::from("([)]");
        assert!(!is_valid(s));
    }

    #[test]
    fn invalid_extra_open() {
        let s: String = String::from("(()");
        assert!(!is_valid(s));
    }

    #[test]
    fn empty_string() {
        let s: String = String::from("");
        assert!(is_valid(s));
    }
}
