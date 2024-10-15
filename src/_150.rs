// Time complexity: O(n), n is the length of the array tkns. We traverse the array once.
// Space complexity: O(n), all elements are pushed onto a stack.
// https://chatgpt.com/c/670ea714-4310-8002-aae5-91a9d8721bee
pub fn eval_rpn(tkns: Vec<String>) -> i32 {
    // Evaluate Reverse Polish Notation
    // Given an array of string `tkns`.
    // String contains arithmetic expressions in post-fix.
    // Evaluate all expressions.
    // Return the result of evaluating all expressions.
    // Use a stack to place operands.
    // Pop operands when an operator is encountered.
    // Operands: +, -, *, /

    // Initialize variables.
    let mut stk: Vec<i32> = Vec::new();

    // Loop through each token.
    for tkn in tkns {
        match tkn.as_str() {
            "+" => {
                let b = stk.pop().unwrap();
                let a = stk.pop().unwrap();
                stk.push(a + b);
            }
            "-" => {
                let b = stk.pop().unwrap();
                let a = stk.pop().unwrap();
                stk.push(a - b);
            }
            "*" => {
                let b = stk.pop().unwrap();
                let a = stk.pop().unwrap();
                stk.push(a * b);
            }
            "/" => {
                let b = stk.pop().unwrap();
                let a = stk.pop().unwrap();
                stk.push(a / b);
            }
            _ => {
                // Parse string to number.
                let num = tkn.parse::<i32>().unwrap();
                stk.push(num);
            }
        }
    }

    // Return final result of whole evaluation.
    stk.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_expr_1() {
        let tokens = vec![
            "2".to_string(),
            "1".to_string(),
            "+".to_string(),
            "3".to_string(),
            "*".to_string(),
        ];
        let result = eval_rpn(tokens);
        assert_eq!(result, 9); // (2 + 1) * 3 = 9
    }

    #[test]
    fn valid_expr_2() {
        let tokens = vec![
            "4".to_string(),
            "13".to_string(),
            "5".to_string(),
            "/".to_string(),
            "+".to_string(),
        ];
        let result = eval_rpn(tokens);
        assert_eq!(result, 6); // 4 + (13 / 5) = 6
    }

    #[test]
    fn valid_expr_3() {
        let tokens = vec![
            "10".to_string(),
            "6".to_string(),
            "9".to_string(),
            "3".to_string(),
            "+".to_string(),
            "-11".to_string(),
            "*".to_string(),
            "/".to_string(),
            "*".to_string(),
            "17".to_string(),
            "+".to_string(),
            "5".to_string(),
            "+".to_string(),
        ];
        let result = eval_rpn(tokens);
        assert_eq!(result, 22); // Complex expression evaluation
    }

    #[test]
    fn invalid_token() {
        let tokens = vec!["2".to_string(), "a".to_string(), "+".to_string()];
        let result = std::panic::catch_unwind(|| eval_rpn(tokens));
        assert!(result.is_err()); // Should panic on invalid input
    }
}
