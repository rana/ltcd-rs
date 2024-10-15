pub fn calculate(s: String) -> i32 {
    // Basic Calculator
    // Given a string s.
    // String is an arithmetic expression with infix notation.
    // Characters are: digits + - () space.
    // + is not unary.
    // - may be unary.
    // Use a stack to maintain intermediate state.

    let mut stk: Vec<i32> = Vec::new();
    let mut res: i32 = 0; // result
    let mut cur: i32 = 0; // current value
    let mut sgn: i32 = 0; // sign
    let mut idx: usize = 0; // index
    let chrs: Vec<char> = s.chars().collect();
    let len = chrs.len();

    // Loop through each character.
    while idx < len {
        let chr = chrs[idx];

        // Check character operations.
        if chr.is_ascii_digit() {
            // Parse digit.
            cur = 0;
            while idx < len && chrs[idx].is_ascii_digit() {
                cur = cur * 10 + chrs[idx].to_digit(10).unwrap() as i32;
                idx += 1;
            }

            // Add current number to result.
            res += sgn * cur;

            // Skip incrementing index again.
            continue;
        } else if chr == '+' {
            // Set sign to positive.
            sgn = 1;
        } else if chr == '-' {
            // Set sign to negative.
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

            // Combine previous result and sign
            res = prv_res + prv_sgn * res;
        }

        idx += 1;
    }

    cur
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
