// Time complexity: O(S), S is the sum of all characters in all strings.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/6705722f-ed00-8002-9732-3aba309eb7bb
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // Use horizontal scanning of each string.

    // Check input minimum edge case.
    if strs.is_empty() {
        return String::new();
    }

    // Initialize a prefix with the first string.
    // Clone because we may reduce the prefix length.
    let mut prefix = strs[0].clone();

    // Iterate through remaining strings.
    for s in strs.iter().skip(1) {
        // Look for the common prefix in comparison strings.
        while !s.starts_with(&prefix) {
            // Remove the last char from the reference prefix.
            prefix.pop();

            // Check for the min prefix condition.
            if prefix.is_empty() {
                return String::new();
            }
        }
    }

    // Common prefix found.
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
