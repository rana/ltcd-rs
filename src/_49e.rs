// Time complexity: O(n klogk), n is the number of words. k is the length of the longest word. klogj to sort the characters of each word.
// Space complexity: O(n * k), for keys to the HashMap.
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // Group Anagrams
    // Given and array of strings.
    // Group anagrams.
    // Return an array of anagram groups.
    // Character set is 26 lowercase English letters.
    // Use a Hashmap to group anagrams.
    // Use an sorted anagram as the HashMap key.

    use std::collections::HashMap;

    // Initialize HashMap for grouping anagrams.
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // Iterate through each word.
    for wrd in strs {
        // Create an array of chars for sorting.
        let mut chrs: Vec<char> = wrd.chars().collect();
        // Sort characters for anagram key.
        chrs.sort_unstable();
        // Create map key.
        let key: String = chrs.iter().collect();

        // Group word by sorted key.
        map.entry(key).or_default().push(wrd);
    }

    // Return grouped anagrams.
    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let wrds: Vec<String> = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let mut res = group_anagrams(wrds);
        // Sorting the inner vectors for consistent comparison.
        for group in &mut res {
            group.sort();
        }
        res.sort_by(|a, b| a[0].cmp(&b[0]));
        let expected: Vec<Vec<String>> = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn empty_input() {
        let wrds: Vec<String> = vec![];
        let res = group_anagrams(wrds);
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn single_word() {
        let wrds: Vec<String> = vec!["abc".to_string()];
        let res = group_anagrams(wrds);
        let expected: Vec<Vec<String>> = vec![vec!["abc".to_string()]];
        assert_eq!(res, expected);
    }

    #[test]
    fn no_anagrams() {
        let wrds: Vec<String> = vec!["abc".to_string(), "def".to_string(), "ghi".to_string()];
        let mut res = group_anagrams(wrds);
        // Sorting the outer vector for consistent comparison.
        res.sort();
        let expected: Vec<Vec<String>> = vec![
            vec!["abc".to_string()],
            vec!["def".to_string()],
            vec!["ghi".to_string()],
        ];
        assert_eq!(res, expected);
    }
}
