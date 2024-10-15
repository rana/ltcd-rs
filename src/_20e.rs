pub fn is_valid(s: String) -> bool {
    // Valid Parenthesis
    // Given a string s.
    // String contains characters: (){}[]
    // Determine condition:
    //  * Equal type bracket open and close.
    //  * Proper ordering bracket open close.
    // Return true if conditions met.
    // Use a stack to apply rules.

    // Initialize variable.
    let mut stk: Vec<char> = Vec::new();

    // Loop through each character in string s.
    for chr in s.chars() {
        match chr {
            // Open bracket: place closing bracket.
            '(' => stk.push(')'),
            '{' => stk.push('}'),
            '[' => stk.push(']'),
            // Closing bracket: validate stack top.
            ')' | '}' | ']' => {
                if stk.pop() != Some(chr) {
                    return false;
                }
            }
            // Invalid character.
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
