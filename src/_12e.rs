// Time complexity: O(1), max integer is fixed at 3999 by definition.
// Space complexity: O(1), constant space is used.
pub fn int_to_roman(mut num: i32) -> String {
    // Use table lookup and mapping between integer
    // roman numerals.

    // Check for input edge conditions.
    if num <= 0 || num > 3999 {
        return String::new();
    }

    // Define mapping tables.
    // Start with largest integers.
    // Include subtracted variant `IV = 4`.
    let vals = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let syms = [
        "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
    ];

    let mut res = String::new();

    // Loop through vals.
    // Descending order of vals critical.
    for (&val, &sym) in vals.iter().zip(syms.iter()) {
        // Subtract out largest integers while possible.
        while num >= val {
            num -= val;
            // Translate integer to roman numeral.
            res.push_str(sym);
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
