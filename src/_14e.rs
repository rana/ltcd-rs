pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // Determine longest common prefix in all strings.
    // Input: strs = ["flower","flow","flight"]
    // Output: "fl"

    // Check for input minimum edge condition.
    if strs.is_empty() {
        return String::new();
    }

    // Use first word as starting prefix.
    let mut prefix = strs[0].clone();

    // Look through remaining strings.
    for s in strs.iter().skip(1) {
        // Check for prefix match.
        while !s.starts_with(&prefix) {
            // Reduce prefix end.
            prefix.pop();

            // Check for non-match exit case.
            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    // Return success match case.
    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prefix() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];
        assert_eq!(longest_common_prefix(strs), "fl");
    }

    #[test]
    fn test_no_common_prefix() {
        let strs = vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car"),
        ];
        assert_eq!(longest_common_prefix(strs), "");
    }

    #[test]
    fn test_single_string() {
        let strs = vec![String::from("alone")];
        assert_eq!(longest_common_prefix(strs), "alone");
    }

    #[test]
    fn test_empty_string_in_vector() {
        let strs = vec![String::from(""), String::from("empty"), String::from("")];
        assert_eq!(longest_common_prefix(strs), "");
    }

    #[test]
    fn test_all_empty_strings() {
        let strs = vec![String::from(""), String::from(""), String::from("")];
        assert_eq!(longest_common_prefix(strs), "");
    }

    #[test]
    fn test_empty_vector() {
        let strs: Vec<String> = vec![];
        assert_eq!(longest_common_prefix(strs), "");
    }
}
