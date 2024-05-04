/// 49. Group Anagrams
///
/// Given an array of strings strs, group the anagrams
/// together. You can return the answer in any order.
///
/// An Anagram is a word or phrase formed by rearranging
/// the letters of a different word or phrase, typically
/// using all the original letters exactly once.
///
/// Constraints:
/// * 1 <= strs.length <= 10^4
/// * 0 <= strs[i].length <= 100
/// * strs[i] consists of lowercase English letters.

fn group_anagrams(strs: Vec<&str>) -> Vec<Vec<String>> {
    // Insert each word into a HashMap with a sorted word key.
    // Sort the characters of each word.
    // Use the sorted word as a key.
    // Use the word as a value in an array.
    // Time complexity: O(m * nlogn).
    //  - m is the number of elements in strs.
    //  - nlogn is to sort each element in strs.
    // Space complexity: O(m * n).
    //  - m is the number of elements in strs.
    //  - n is the average length of an element in strs.
    use std::collections::HashMap;

    // Create a map to store anagrams.
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // Loop through each word.
    // Contributes O(m) portion of O(m * nlogn) time complexity.
    for wrd in strs {
        // Create a sorted word.
        // Contributes O(nlogn) portion of O(m * nlogn) time complexity.
        let mut wrd_vec: Vec<char> = wrd.chars().collect();
        wrd_vec.sort_unstable();
        let wrd_srt: String = wrd_vec.iter().collect();

        // Insert sorted word key with anagram.
        map.entry(wrd_srt).or_default().push(wrd.to_string());
    }

    map.into_values().collect()
}

fn group_anagrams_b(strs: Vec<&str>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // Loop through each word.
    // Contributes O(m) portion of O(m * nlogn) time complexity.
    for wrd in strs {
        // Sort the characters of the word.
        // Contributes O(nlogn) portion of O(m * nlogn) time complexity.
        let mut chr_vec: Vec<char> = wrd.chars().collect();
        chr_vec.sort_unstable();
        let wrd_srt = chr_vec.into_iter().collect::<String>();

        // Insert the word using the sorted word as a key.
        map.entry(wrd_srt).or_default().push(wrd.to_string());
    }

    map.into_values().collect()
}

fn group_anagrams_a(strs: Vec<&str>) -> Vec<Vec<String>> {
    use std::collections::HashMap;

    let mut anagram_map = HashMap::new();

    for s in strs {
        let mut char_vec: Vec<char> = s.chars().collect();
        char_vec.sort_unstable();
        let sorted_str = char_vec.into_iter().collect::<String>();
        anagram_map
            .entry(sorted_str)
            .or_insert_with(Vec::new)
            .push(s.to_string());
    }

    anagram_map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::group_anagrams;

    #[test]
    fn test_basic_case() {
        let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut result = group_anagrams(input);
        result.sort(); // Sort the list of lists for comparison

        let mut expected = vec![
            vec!["eat", "tea", "ate"]
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            vec!["tan", "nat"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            vec!["bat"].into_iter().map(|s| s.to_string()).collect(),
        ];
        expected.sort(); // Sort the list of lists for comparison

        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = vec![];
        let result = group_anagrams(input);
        let expected: Vec<Vec<String>> = vec![];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_string() {
        let input = vec!["apple"];
        let result = group_anagrams(input);
        let expected = vec![vec!["apple".to_string()]];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_anagrams() {
        let input = vec!["cat", "dog", "bird"];
        let mut result = group_anagrams(input);
        result.sort();

        let mut expected = vec![
            vec!["cat".to_string()],
            vec!["dog".to_string()],
            vec!["bird".to_string()],
        ];
        expected.sort();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_large_input() {
        let input = vec!["abc", "cba", "bac", "cab", "acb", "bca"];
        let mut result = group_anagrams(input);
        result.sort();

        let mut expected: Vec<Vec<String>> = vec![vec!["abc", "cba", "bac", "cab", "acb", "bca"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()];
        expected.sort();

        assert_eq!(result, expected);
    }
}
