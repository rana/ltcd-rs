/// 30. Substring with Concatenation of All Words
///
/// You are given a string s and an array of strings
/// words. All the strings of words are of the same
/// length.
///
/// A concatenated string is a string that exactly
/// contains all the strings of any permutation of
/// words concatenated.
///
/// For example, if words = ["ab","cd","ef"], then
/// "abcdef", "abefcd", "cdabef", "cdefab", "efabcd",
/// and "efcdab" are all concatenated strings. "acdbef"
/// is not a concatenated string because it is not
/// the concatenation of any permutation of words.
///
/// Return an array of the starting indices of all
/// the concatenated substrings in s. You can return
/// the answer in any order.
///
/// Constraints:
/// * 1 <= s.length <= 10^4
/// * 1 <= words.length <= 5000
/// * 1 <= words[i].length <= 30
/// * s and words[i] consist of lowercase English letters.

fn find_substring(s: String, wrds: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut ret = vec![];
    let s = s.as_bytes();
    let wrd_len = wrds[0].len();
    let prm_len = wrd_len * wrds.len();

    // Check for a minimum edge condition.
    if s.len() < prm_len {
        return ret;
    }

    // Calculate words frequencies.
    let mut exp_cnts: HashMap<&[u8], u16> = HashMap::new();
    for wrd in &wrds {
        *exp_cnts.entry(wrd.as_bytes()).or_insert(0) += 1;
    }

    // Loop through character offsets.
    // Possible that substring begins after first character.
    for idx_chr in 0..wrd_len {
        let mut obs_cnts: HashMap<&[u8], u16> = HashMap::new();
        let mut wrd_cnt: u16 = 0;
        let mut lft = idx_chr;

        for rht in (idx_chr..=s.len() - wrd_len).step_by(wrd_len) {
            // Search for the next word in the target string.
            let rht_wrd = &s[rht..rht + wrd_len];

            if let Some(&exp_cnt) = exp_cnts.get(rht_wrd) {
                // Found a valid word.
                // Increment the observed count.
                *obs_cnts.entry(rht_wrd).or_insert(0) += 1;
                wrd_cnt += 1;

                // Check whether to shrink the window on the left.
                while obs_cnts[rht_wrd] > exp_cnt {
                    // Shrink the window.
                    let lft_wrd = &s[lft..lft + wrd_len];
                    *obs_cnts.get_mut(lft_wrd).unwrap() -= 1;
                    wrd_cnt -= 1;
                    lft += wrd_len;
                }

                // Check whether we found a solution.
                if wrd_cnt == wrds.len() as u16 {
                    ret.push(lft as i32);
                }
            } else {
                // No word found.
                // Reset the tracking variables.
                obs_cnts.clear();
                wrd_cnt = 0;
                lft = rht + wrd_len;
            }
        }
    }

    ret
}

fn find_substring_c(s: String, wrds: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut ret = vec![];
    let wrd_len = wrds[0].len();
    let total_len = wrd_len * wrds.len();

    // Check for minimum edge condition.
    // The target string length must be
    // equal to or great than the permutation length.
    if s.len() < total_len {
        return ret;
    }

    // Calculate expected word frequencies.
    let mut exp_cnts: HashMap<&str, i32> = HashMap::new();
    for wrd in &wrds {
        *exp_cnts.entry(wrd.as_str()).or_insert(0) += 1;
    }

    // Loop through length of word.
    // Substring maybe offset by some characters.
    for idx_chr in 0..wrd_len {
        let mut wnd_cnts: HashMap<&str, i32> = HashMap::new();
        let mut prm_cnt: usize = 0;
        let mut lft = idx_chr;

        for rht in (idx_chr..=s.len() - wrd_len).step_by(wrd_len) {
            // Look for a word in the target substring.
            let rht_wrd = &s[rht..rht + wrd_len];

            if let Some(&exp_cnt) = exp_cnts.get(rht_wrd) {
                // Found a word.
                // Increment the observed word count.
                *wnd_cnts.entry(rht_wrd).or_insert(0) += 1;
                prm_cnt += 1;

                // Check whether to shrink the window on the left side.
                while wnd_cnts[rht_wrd] > exp_cnt {
                    let lft_wrd = &s[lft..lft + wrd_len];
                    *wnd_cnts.get_mut(lft_wrd).unwrap() -= 1;
                    lft += wrd_len;
                    prm_cnt -= 1;
                }

                // Check whether we've found a solution.
                if prm_cnt == wrds.len() {
                    ret.push(lft as i32);
                }
            } else {
                // No word found.
                // Reset tracking variables.
                wnd_cnts.clear();
                prm_cnt = 0;
                lft = rht + wrd_len;
            }
        }
    }

    ret
}

fn find_substring_b(s: String, words: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut result = vec![];

    if words.is_empty() || s.is_empty() {
        return result;
    }

    let word_len = words[0].len();
    let total_len = words.len() * word_len;

    if s.len() < total_len {
        return result;
    }

    let mut word_count: HashMap<&str, i32> = HashMap::new();

    for word in &words {
        *word_count.entry(word.as_str()).or_insert(0) += 1;
    }

    for i in 0..word_len {
        let mut left = i;
        let mut window_word_count: HashMap<&str, i32> = HashMap::new();
        let mut count = 0;

        for j in (i..=s.len() - word_len).step_by(word_len) {
            let word = &s[j..j + word_len];

            if let Some(expected_count) = word_count.get(word) {
                *window_word_count.entry(word).or_insert(0) += 1;
                count += 1;

                while window_word_count[word] > *expected_count {
                    let left_word = &s[left..left + word_len];
                    *window_word_count.get_mut(left_word).unwrap() -= 1;
                    left += word_len;
                    count -= 1;
                }

                if count == words.len() {
                    result.push(left as i32);
                }
            } else {
                window_word_count.clear();
                count = 0;
                left = j + word_len;
            }
        }
    }

    result
}

fn find_substring_a(s: String, wrds: Vec<String>) -> Vec<i32> {
    use std::collections::HashMap;

    #[derive(Default, Debug)]
    struct Frq {
        /// `max` is the maximum frequency of a reference word.
        max: u16,
        /// `obs` is the observed frequency of a reference word.
        obs: u16,
    }

    // Convert string to bytes.
    let s = s.as_bytes();
    let str_len = s.len();
    let wrd_len = wrds[0].len();
    let sub_str_len = wrds.len() * wrd_len;

    // Check for early exit condition.
    if wrd_len > str_len {
        return vec![];
    }

    // Count word frequency in the reference array.
    let mut map = wrds
        .iter()
        .fold(HashMap::<&[u8], Frq>::new(), |mut map, wrd| {
            map.entry(wrd.as_bytes()).or_default().max += 1;
            map
        });

    let mut ret = vec![];
    let mut reset_observed = false;

    // Start sliding window at each single word character.
    for idx_chr in 0..wrd_len {
        // Check whether to reset the observed word counts.
        if reset_observed {
            map.iter_mut().for_each(|(_, v)| v.obs = 0);
            reset_observed = false;
        }

        // Initialize a new sliding window.
        let mut lft = idx_chr;
        let mut rht = idx_chr;
        while rht <= str_len - wrd_len {
            let rht_wrd = &s[rht..rht + wrd_len];
            println!(
                "rht:{rht} rht_wrd:{:?}",
                String::from_utf8(rht_wrd.to_vec()).unwrap()
            );
            match map.get_mut(rht_wrd) {
                None => {
                    // No word match at current right position.
                    rht += wrd_len;
                    lft = rht;
                    if reset_observed {
                        map.iter_mut().for_each(|(_, v)| v.obs = 0);
                        reset_observed = false;
                    }
                }
                Some(rht_frq) => {
                    // Found a matching word.
                    rht_frq.obs += 1;
                    rht += wrd_len;
                    reset_observed = true;

                    println!("rht_frq:{:?}", rht_frq);

                    // Check whether there are too many observed words.
                    if rht_frq.obs > rht_frq.max {
                        loop {
                            // Remove one observed word from the left.
                            let lft_wrd = &s[lft..lft + wrd_len];
                            println!(
                                "lft:{lft} lft_wrd:{:?}",
                                String::from_utf8(lft_wrd.to_vec()).unwrap()
                            );
                            let lft_frq = map.get_mut(lft_wrd).unwrap();
                            println!("lft_frq:{:?}", lft_frq);
                            lft += wrd_len;
                            lft_frq.obs -= 1;
                            if lft_frq.obs == lft_frq.max {
                                break;
                            }
                        }
                    }
                }
            }

            // Check whether we found a valid solution.
            if rht - lft == sub_str_len {
                ret.push(lft as i32);
                map.get_mut(&s[lft..lft + wrd_len]).unwrap().obs -= 1;
                lft += wrd_len;
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_longer_than_string() {
        assert_eq!(
            find_substring("abc".to_string(), vec!["abcd".to_string()]),
            vec![]
        );
    }

    #[test]
    fn test_valid_concatenation() {
        assert_eq!(
            find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            vec![0, 9]
        );

        assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );

        assert_eq!(
            find_substring(
                "lingmindraboofooowingdingbarrwingmonkeypoundcake".to_string(),
                vec![
                    "fooo".to_string(),
                    "barr".to_string(),
                    "wing".to_string(),
                    "ding".to_string(),
                    "wing".to_string()
                ]
            ),
            vec![13]
        );
    }
}
