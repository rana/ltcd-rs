pub fn roman_to_int(s: String) -> i32 {
    // Convert roman numerals to an integer.
    // Use a helper function to map a roman numeral to integer.
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

    // Loop through each character in reverse.
    // Reverse enables comparison to previous for subtraction.
    for c in s.chars().rev() {
        let cur_value = value(c);
        if cur_value < prv_value {
            total -= cur_value;
        } else {
            total += cur_value;
        }
        prv_value = cur_value;
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
