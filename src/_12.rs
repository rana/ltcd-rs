// Time complexity: O(1), constant time due to maximum fixed number 3999.
// Space complexity: O(1), constant space used.
// https://chatgpt.com/c/67046937-6ca0-8002-b453-7a7d3ac702ac
pub fn int_to_roman(mut num: i32) -> String {
    // Convert an integer to roman numeral string.
    // Use a local optimization "greedy" approach.

    // Check for input min edge condition.
    if num <= 0 || num > 3999 {
        return String::new();
    }

    // Create mappings arrays.
    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];
    let mut res = String::new();

    // Iterate through values.
    for (n, &val) in vals.iter().enumerate() {
        // Subtract current value as long as possible.
        while num >= val {
            num -= val;
            res.push_str(syms[n]);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let num = 3;
        let result = int_to_roman(num);
        assert_eq!(result, "III");
    }

    #[test]
    fn test_example2() {
        let num = 58;
        let result = int_to_roman(num);
        assert_eq!(result, "LVIII");
    }

    #[test]
    fn test_example3() {
        let num = 1994;
        let result = int_to_roman(num);
        assert_eq!(result, "MCMXCIV");
    }

    #[test]
    fn test_min_value() {
        let num = 1;
        let result = int_to_roman(num);
        assert_eq!(result, "I");
    }

    #[test]
    fn test_max_value() {
        let num = 3999;
        let result = int_to_roman(num);
        assert_eq!(result, "MMMCMXCIX");
    }

    #[test]
    fn test_invalid_zero() {
        let num = 0;
        let result = int_to_roman(num);
        assert_eq!(result, "");
    }

    #[test]
    fn test_invalid_negative() {
        let num = -5;
        let result = int_to_roman(num);
        assert_eq!(result, "");
    }

    #[test]
    fn test_invalid_large() {
        let num = 4000;
        let result = int_to_roman(num);
        assert_eq!(result, "");
    }
}
