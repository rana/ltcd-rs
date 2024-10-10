// Time complexity: O(m + n), m is the length of string s. n is the length of string t.
// Space complexity: O(k), k is the number of unique letters in t.
// https://chatgpt.com/c/6706fa92-a168-8002-9016-a1b77418a3bb
pub fn min_window(s: String, t: String) -> String {
    // Minimum window substring.
    // Given string s with length m.
    // Given string t with length n.
    // String t may have duplicate chars.
    // Find minimum window in s of t.
    // Return minimum window substring; or empty string.
    // Use a sliding window two-pointer technique.
    // Use character frequency counts for valid window detection.
    use std::collections::HashMap;

    // Character counts in t.
    let mut t_cnt: HashMap<char, i32> = HashMap::new();
    for ch in t.chars() {
        *t_cnt.entry(ch).or_insert(0) += 1;
    }

    // Initialize variables.
    let mut win_cnt: HashMap<char, i32> = HashMap::new();
    let (mut lft, mut rht): (usize, usize) = (0, 0);
    let mut min_len: Option<usize> = None;
    let mut min_start: usize = 0;
    let mut formed: usize = 0;
    let req_cnt: usize = t_cnt.len();

    let s_chars: Vec<char> = s.chars().collect();

    // Loop through each character in s.
    // Sliding window.
    while rht < s_chars.len() {
        let ch: char = s_chars[rht];
        *win_cnt.entry(ch).or_insert(0) += 1;

        // Check if current character completes a requirement.
        if t_cnt.contains_key(&ch) && win_cnt[&ch] == t_cnt[&ch] {
            formed += 1;
        }

        // Try to contract the window.
        // lft <= rht supports single char case: s="a", t="a"
        while lft <= rht && formed == req_cnt {
            let cur_len: usize = rht - lft + 1;

            // Update min window if current is smaller.
            if min_len.is_none() || cur_len < min_len.unwrap() {
                min_len = Some(cur_len);
                min_start = lft;
            }

            let ch: char = s_chars[lft];
            *win_cnt.entry(ch).or_insert(0) -= 1;

            // Update formed count if necessary.
            if t_cnt.contains_key(&ch) && win_cnt[&ch] < t_cnt[&ch] {
                formed -= 1;
            }

            lft += 1;
        }

        rht += 1;
    }

    match min_len {
        Some(len) => s_chars[min_start..min_start + len].iter().collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_case1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        assert_eq!(min_window(s, t), "BANC");
    }

    #[test]
    fn valid_case2() {
        let s = "a".to_string();
        let t = "a".to_string();
        assert_eq!(min_window(s, t), "a");
    }

    #[test]
    fn valid_case3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        assert_eq!(min_window(s, t), "");
    }

    #[test]
    fn invalid_case1() {
        let s = "".to_string();
        let t = "a".to_string();
        assert_eq!(min_window(s, t), "");
    }
}
