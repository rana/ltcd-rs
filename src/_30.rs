// Time complexity: O(N * M * L), N is the length of string s. M is the length of array wrds. L is the length of each word.
// Space complexity: O(M), for storing frequency maps of words.
// https://chatgpt.com/c/6706dea1-e244-8002-b304-28ba0b0b9881
pub fn find_substring(s: String, wrds: Vec<String>) -> Vec<i32> {
    // You are given an array of words.
    // You are given a string s.
    // Array words may have duplicates.
    // String s may have concat permutations of `words`.
    // String s may have other characters and words.
    // Return and array of starting indexes of whole concat permutations.
    // Use a sliding window two-pointer technique.

    use std::collections::HashMap;

    let mut res: Vec<i32> = Vec::new();
    let s_len: usize = s.len();
    let wrds_len: usize = wrds.len();

    // Check input minimum edge condition.
    if wrds_len == 0 {
        return res;
    }

    let wrd_len: usize = wrds[0].len();
    let ttl_len: usize = wrd_len * wrds_len;

    // Check input minimum edge condition.
    if s_len < ttl_len {
        return res;
    }

    // Create a frequency map of the words.
    // Used to accomodate duplicate words.
    let mut wrd_cnt: HashMap<&str, i32> = HashMap::new();
    for wrd in &wrds {
        *wrd_cnt.entry(wrd).or_insert(0) += 1;
    }

    // Iterate over string s.
    // Look at multiple starting indexes due to filler
    // characters in string s.
    for n in 0..wrd_len {
        let mut lft: usize = n;
        let mut rht: usize = n;
        // For duplicate word tracking.
        let mut cur_cnt: HashMap<&str, i32> = HashMap::new();
        // For permutation completion tracking.
        let mut prm_cnt: usize = 0; 

        // Ensure rht is within bounds.
        while rht + wrd_len <= s_len {
            // Get candidate word.
            let wrd = &s[rht..rht + wrd_len];
            // Prepare for next word.
            rht += wrd_len;

            // Check for word match.
            if wrd_cnt.contains_key(wrd) {
                // Single word match.
                // Increment word count.
                *cur_cnt.entry(wrd).or_insert(0) += 1;
                prm_cnt += 1;

                // Shrink sliding window if possible.
                while cur_cnt[wrd] > wrd_cnt[wrd] {
                    let lft_wrd = &s[lft..lft + wrd_len];
                    *cur_cnt.get_mut(lft_wrd).unwrap() -= 1;
                    lft += wrd_len;
                    prm_cnt -= 1;
                }

                // Check for success condition: permutation found.
                if prm_cnt == wrds_len {
                    res.push(lft as i32);
                }
            } else {
                // No word match.
                cur_cnt.clear();
                prm_cnt = 0;
                lft = rht;
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let expected = vec![0, 9];
        let mut result = find_substring(s, words);
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let expected: Vec<i32> = vec![];
        let result = find_substring(s, words);
        assert_eq!(result, expected);
    }

    #[test]
    fn example3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let expected = vec![6, 9, 12];
        let mut result = find_substring(s, words);
        result.sort();
        assert_eq!(result, expected);
    }
}
