// Time complexity: O(n), n is the length of the array tkns. We traverse the array once.
// Space complexity: O(n), we push each string onto a stack.
pub fn eval_rpn(tkns: Vec<String>) -> i32 {
    // Evaluate Reverse Polish Notation
    // Given an array of strings `tkns`.
    // Strings are arithmetic expression in post-fix form.
    // Valid operators are: + - * /
    // Evaluate all expressions.
    // Return an integer result of the evaluation.
    // Use a stack to maintain expression state.

    // Initialize variables.
    let mut stk: Vec<i32> = Vec::new();

    // Loop through each expression string.
    for tkn in tkns {
        match tkn.as_str() {
            "+" => {
                // Pop last two operands.
                let b: i32 = stk.pop().unwrap();
                let a: i32 = stk.pop().unwrap();
                // Add two operands.
                stk.push(a + b);
            }
            "-" => {
                // Pop last two operands.
                let b: i32 = stk.pop().unwrap();
                let a: i32 = stk.pop().unwrap();
                // Subtract two operands.
                stk.push(a - b);
            }
            "*" => {
                // Pop last two operands.
                let b: i32 = stk.pop().unwrap();
                let a: i32 = stk.pop().unwrap();
                // Multiply two operands.
                stk.push(a * b);
            }
            "/" => {
                // Pop last two operands.
                let b: i32 = stk.pop().unwrap();
                let a: i32 = stk.pop().unwrap();
                // Divide two operands.
                stk.push(a / b);
            },
            _ => {
                // Parse string to integer.
                let num = tkn.parse::<i32>().unwrap();
                stk.push(num);
            }
        }
    }

    // Last stack value is result.
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
