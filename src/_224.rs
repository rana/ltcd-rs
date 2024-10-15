// Time complexity: O(n), n is the length of string s. We traverse each character once.
// Space complexity: O(n), each character is placed onto an intermediate stack.
// https://chatgpt.com/c/670eabc0-e978-8002-b826-fb2d90557735
#[allow(unused_assignments)]
pub fn calculate(s: String) -> i32 {
    // Basic Calculator
    // Given a string s.
    // s preresent an arithmetic calculation.
    // Evaluate the expression.
    // Return an integer result.
    // Valid characters: +, -, (), space.
    // Do not use builtin functions.
    // Use a stack to store intermediate results.

    let mut res: i32 = 0;
    let mut sgn: i32 = 1;
    let mut stk: Vec<i32> = Vec::new();

    let chrs: Vec<char> = s.chars().collect();
    let len: usize = chrs.len();
    let mut idx: usize = 0;

    // Lopo through each character.
    while idx < len {
        let chr = chrs[idx];

        // Determine calculator operation.
        if chr.is_ascii_digit() {
            // Parse multiple characters to an integer.
            let mut cur: i32 = 0;
            // Loop through all digits.
            while idx < len && chrs[idx].is_ascii_digit() {
                cur = cur * 10 + chrs[idx].to_digit(10).unwrap() as i32;
                idx += 1;
            }

            // Store current digits and sign.
            res += sgn * cur;

            // Skip incrementing the index again.
            continue;
        } else if chr == '+' {
            // Set the sign to positive.
            sgn = 1;
        } else if chr == '-' {
            sgn = -1;
        } else if chr == '(' {
            // Store result and sign.
            stk.push(res);
            stk.push(sgn);
            // Reset result and sign for sub-expression.
            res = 0;
            sgn = 1;
        } else if chr == ')' {
            // Load previous result and sign.
            let prv_sgn = stk.pop().unwrap();
            let prv_res = stk.pop().unwrap();

            // Combine previous with current.
            res = prv_res + prv_sgn * res;
        }

        idx += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        let s: String = "1 + 1".to_string();
        assert_eq!(calculate(s), 2);
    }

    #[test]
    fn subtraction_and_whitespace() {
        let s: String = " 2-1 + 2 ".to_string();
        assert_eq!(calculate(s), 3);
    }

    #[test]
    fn nested_parentheses() {
        let s: String = "(1+(4+5+2)-3)+(6+8)".to_string();
        assert_eq!(calculate(s), 23);
    }

    #[test]
    fn large_number() {
        let s: String = "2147483647".to_string();
        assert_eq!(calculate(s), 2147483647);
    }

    #[test]
    fn multiple_nested_parentheses() {
        let s: String = "((1+(3)))+(4+(5+2))".to_string();
        assert_eq!(calculate(s), 15);
    }
}
