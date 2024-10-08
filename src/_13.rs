// Time Complexity: O(n), where n is the length of string s.
// Space complexity: O(1), constant space used.
// https://chatgpt.com/c/6704650a-0b58-8002-be29-2d795b44909e
pub fn roman_to_int(s: String) -> i32 {
    // Use a helper function for mapping character to integer.
    fn value(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }

    // Initialize variables.
    let mut total = 0;
    let mut prv_value = 0;

    // Iterate through each element in reverse.
    for c in s.chars().rev() {
        // Translate current value.
        let cur_value = value(c);
        
        // Compare current to previous to determine subtraction.
        if cur_value < prv_value {
            // Subtract per roman numeral definition.
            total -= cur_value;
        } else {
            // Add per roman numeral definition.
            total += cur_value;
            prv_value = cur_value;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iii() {
        assert_eq!(roman_to_int("III".to_string()), 3);
    }

    #[test]
    fn test_iv() {
        assert_eq!(roman_to_int("IV".to_string()), 4);
    }

    #[test]
    fn test_ix() {
        assert_eq!(roman_to_int("IX".to_string()), 9);
    }

    #[test]
    fn test_lviii() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_mcmxciv() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
