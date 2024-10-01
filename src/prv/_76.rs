/// 76. Minimum Window Substring
///
/// Given two strings s and t of lengths m and n respectively,
/// return the minimum window substring of s such that every
/// character in t (including duplicates) is included in the
/// window. If there is no such substring, return the empty
/// string "".
///
/// The testcases will be generated such that the answer is unique.
///
/// Constraints:
/// * m == s.length
/// * n == t.length
/// * 1 <= m, n <= 10^5
/// * s and t consist of uppercase and lowercase English letters.

fn min_window(s: String, t: String) -> String {
    use std::collections::HashMap;

    // Count expected character frequencies.
    let mut exp_cnts = HashMap::new();
    for chr in t.chars() {
        *exp_cnts.entry(chr).or_insert(0) += 1;
    }

    let mut obs_cnts = HashMap::new();
    let mut vld_chr_cnt: usize = 0;
    let mut lft: usize = 0;
    let mut rht: usize = 0;
    let mut wnd_len_min: usize = usize::MAX;
    let mut ret = "";

    while rht < s.len() {
        let rht_chr = s.chars().nth(rht).unwrap();
        rht += 1;

        // Check for a valid character.
        if exp_cnts.contains_key(&rht_chr) {
            // Increment the observed count.
            *obs_cnts.entry(rht_chr).or_insert(0) += 1;
            // Increment the valid character count.
            if obs_cnts[&rht_chr] == exp_cnts[&rht_chr] {
                vld_chr_cnt += 1;
            }
        }

        // Check for a valid solution.
        while vld_chr_cnt == exp_cnts.len() {
            // Store a valid solution if smallest.
            let wnd_len = rht - lft;
            if wnd_len < wnd_len_min {
                wnd_len_min = wnd_len;
                ret = &s[lft..rht];
            }

            // Shrink the window on the left side.
            let lft_chr = s.chars().nth(lft).unwrap();
            lft += 1;
            // Check for a valid left character.
            if exp_cnts.contains_key(&lft_chr) {
                // Decrement the valid character count.
                if obs_cnts[&lft_chr] == exp_cnts[&lft_chr] {
                    vld_chr_cnt -= 1;
                }
                // Decrement the observed count.
                *obs_cnts.entry(lft_chr).or_insert(0) -= 1;
            }
        }
    }

    ret.to_string()
}

fn min_window_e(s: String, t: String) -> String {
    use std::collections::HashMap;

    // Count expected character frequencies.
    let mut exp_cnts = HashMap::new();
    for chr in t.chars() {
        *exp_cnts.entry(chr).or_insert(0) += 1;
    }

    let mut obs_cnts = HashMap::new();
    let mut lft: usize = 0;
    let mut rht: usize = 0;
    let mut vld_chr_cht: usize = 0;
    let mut wnd_len_min: usize = usize::MAX;
    let mut ret = "";

    // Loop through string `s`.
    while rht < s.len() {
        let rht_chr = s.chars().nth(rht).unwrap();
        rht += 1;

        // Check for valid character.
        if exp_cnts.contains_key(&rht_chr) {
            // Valid character.
            // Increment the observed count.
            *obs_cnts.entry(rht_chr).or_insert(0) += 1;
            // Increment the valid character count.
            if exp_cnts[&rht_chr] == obs_cnts[&rht_chr] {
                vld_chr_cht += 1;
            }
        }

        // Check for a valid window.
        while vld_chr_cht == exp_cnts.len() {
            // Store a solution if smallest length.
            let wnd_len = rht - lft;
            if wnd_len < wnd_len_min {
                wnd_len_min = wnd_len;
                ret = &s[lft..rht];
            }

            // Shrink the window on the left side.
            let lft_chr = s.chars().nth(lft).unwrap();
            lft += 1;
            if exp_cnts.contains_key(&lft_chr) {
                // Increment the valid character count.
                if exp_cnts[&lft_chr] == obs_cnts[&lft_chr] {
                    vld_chr_cht -= 1;
                }
                // Decrement the observed character count.
                *obs_cnts.get_mut(&lft_chr).unwrap() -= 1;
            }
        }
    }

    ret.to_string()
}

fn min_window_d(s: String, t: String) -> String {
    use std::collections::HashMap;

    // Frequency maps to store counts of characters in t and current window
    let mut t_freq = HashMap::new();
    let mut window_freq = HashMap::new();

    // Fill frequency map for characters in t
    for ch in t.chars() {
        *t_freq.entry(ch).or_insert(0) += 1;
    }

    let mut left = 0; // Left pointer
    let mut right = 0; // Right pointer
    let mut valid_chars = 0; // Number of valid characters in the window
    let mut min_len = usize::MAX; // Minimum length of a valid window
    let mut result = ""; // Substring to store result

    while right < s.len() {
        let right_char = s.chars().nth(right).unwrap(); // Get char at right pointer
        right += 1; // Increment right pointer

        if t_freq.contains_key(&right_char) {
            *window_freq.entry(right_char).or_insert(0) += 1;
            if window_freq[&right_char] == t_freq[&right_char] {
                valid_chars += 1;
            }
        }

        // Check if window is valid
        while valid_chars == t_freq.len() {
            let current_len = right - left; // Length of current window
            if current_len < min_len {
                min_len = current_len;
                result = &s[left..right];
            }

            let left_char = s.chars().nth(left).unwrap(); // Get char at left pointer
            left += 1; // Increment left pointer

            if t_freq.contains_key(&left_char) {
                if window_freq[&left_char] == t_freq[&left_char] {
                    valid_chars -= 1;
                }
                *window_freq.get_mut(&left_char).unwrap() -= 1;
            }
        }
    }

    result.to_string() // Return the resulting string
}

fn min_window_c(s: String, t: String) -> String {
    use std::collections::HashMap;

    let mut need: HashMap<char, i32> = HashMap::new();
    t.chars().for_each(|ch| *need.entry(ch).or_insert(0) += 1);

    let (mut left, mut right, mut valid) = (0, 0, 0);
    let mut window: HashMap<char, i32> = HashMap::new();
    let mut min_len = std::usize::MAX;
    let mut start = 0;

    let s_chars: Vec<char> = s.chars().collect();

    while right < s.len() {
        let c = s_chars[right];
        right += 1;

        if need.contains_key(&c) {
            *window.entry(c).or_insert(0) += 1;
            if window.get(&c) == need.get(&c) {
                valid += 1;
            }
        }

        while valid == need.len() {
            if right - left < min_len {
                start = left;
                min_len = right - left;
            }

            let d = s_chars[left];
            left += 1;

            if need.contains_key(&d) {
                if window.get(&d) == need.get(&d) {
                    valid -= 1;
                }
                *window.entry(d).or_default() -= 1;
            }
        }
    }

    if min_len == std::usize::MAX {
        String::new()
    } else {
        s[start..start + min_len].to_string()
    }
}

fn min_window_b(s: String, t: String) -> String {
    /// `Wnd` is a window with index ranges.
    #[derive(Default, Debug, Clone, Copy)]
    struct Wnd {
        /// `lft` is the left index of a window.
        lft: usize,
        /// `rht` is the right index of a window.
        rht: usize,
    }

    // Create a character frequency map.
    // Use an array due to the known character range.
    // b'A':65   b'z':122
    // Add one to the size because b'z' is an inclusive index.
    let mut cnts = [0i32; (b'z' - b'A' + 1) as usize];

    // Use a macro to improve code readability.
    // Allows u8 key to a character frequency count.
    macro_rules! cnts  {
            [$key:expr] => (cnts[($key - b'A') as usize])
        }

    // Count the frequency of valid characters.
    t.bytes().for_each(|byt| cnts![byt] += 1);

    // Initialize sliding window variables.
    let byts = s.as_bytes();
    let str_len = byts.len();
    let mut tgt_wnd_len = t.len();
    let mut min: Option<Wnd> = None;
    let mut cur = Wnd::default();

    // Work towards a target window length of zero.
    while cur.rht < str_len {
        // Expand the window on the right side.

        // Check whether we can approach a valid window.
        // Check whether we have a valid frequency count of a valid character.
        if cnts![byts[cur.rht]] > 0 {
            // Decrement the target window length.
            // Target window length of zero means we have a success condition.
            tgt_wnd_len -= 1;
        }

        // Decrement the frequency count of the character at the right index.
        // Valid characters will approach zero.
        // Invalid characters will approach i32::MIN.
        cnts![byts[cur.rht]] -= 1;

        // Advance the right index of the window.
        cur.rht += 1;

        // Check whether we have a valid window.
        // Valid window is when `tgt_wnd_len == 0`.
        while tgt_wnd_len == 0 {
            // Check for a minimum window.
            if cur.rht - cur.lft < min.map_or(usize::MAX, |min| min.rht - min.lft) {
                min = Some(cur);
            }

            // Shrink the window on the left side.

            // Increment the count of the left character.
            // Valid characters will have a character count above zero.
            // Invalid characters will approach zero.
            cnts![byts[cur.lft]] += 1;

            // Check whether we can increase the target window length.
            if cnts![byts[cur.lft]] > 0 {
                // Increment the target window length.
                // Moves away from a valid window.
                tgt_wnd_len += 1;
            }

            // Increment the left index.
            cur.lft += 1;
        }
    }

    // Return the minimum window string; or, an empty string.
    min.map_or("".into(), |min| s[min.lft..min.rht].into())
}

fn min_window_a(s: String, t: String) -> String {
    // Algorithm: Sliding window with two pointers.

    use std::collections::HashMap;

    /// `Frq` is a frequency count of a character.
    #[derive(Default, Debug)]
    struct Frq {
        /// `max` is the maximum count of a character.
        max: u16,
        /// `obs` is the observed count of a character.
        obs: u16,
    }
    /// `Wnd` is a window with index ranges.
    #[derive(Default, Debug, Clone, Copy)]
    struct Wnd {
        /// `lft` is the left index of a window.
        lft: u32,
        /// `rht` is the right inclusive index of a window.
        rht: u32,
    }
    impl Wnd {
        /// `len` returns the length of the window.
        fn len(&self) -> u32 {
            (self.rht - self.lft).saturating_add(1)
        }
    }

    // Check for invalid edge cases:
    // * t is empty.
    // * s is too small.
    if t.is_empty() || s.len() < t.len() {
        return String::new();
    }

    // Transform characters to bytes for time efficiency and space efficiency.
    let s = s.as_bytes();
    let t = t.as_bytes();

    // Count the frequency of each reference character.
    let mut map = t
        .iter()
        .fold(HashMap::<u8, Frq>::new(), |mut map, &byt_chr| {
            map.entry(byt_chr).or_default().max += 1;
            map
        });

    // Initialize sliding window variables.
    let tgt_len = s.len() as u32;
    let ref_len = t.len() as u32;
    let mut min = Wnd {
        lft: 0,
        rht: u32::MAX,
    };
    let mut cur = Wnd::default();
    // Starting at the first character.
    // Increment the first character's observed count.
    // Possible character is not in reference string.
    if let Some(frq) = map.get_mut(&s[0]) {
        frq.obs += 1;
    }

    // Search for observed characters in the target string `s`.
    loop {
        // Check for success condition.
        // Are all reference character frequencies in the current window?
        // Possible to have more character observances than maximum.
        if cur.lft <= cur.rht && cur.len() >= ref_len {
            let is_frq_match = map.values().all(|frq| frq.obs >= frq.max);
            if is_frq_match {
                // Store index range of valid window if necessary.
                if cur.len() < min.len() {
                    min = cur;
                }

                // Shrink the window.
                // Decrement the left character's observed count.
                if let Some(frq) = map.get_mut(&s[cur.lft as usize]) {
                    frq.obs -= 1;
                }
                // Advance the left index.
                cur.lft += 1;

                // Check for a valid window in next loop.
                // Skip right index advancing.
                continue;
            }
        }

        // Advance the right index.
        cur.rht += 1;

        // Check for loop exit condition.
        if cur.rht == tgt_len {
            break;
        }

        // Increment the right character's observed count.
        if let Some(frq) = map.get_mut(&s[cur.rht as usize]) {
            frq.obs += 1;
        }
    }

    if min.rht == u32::MAX {
        String::new()
    } else {
        let byt_str = s[min.lft as usize..=min.rht as usize].to_vec();
        String::from_utf8(byt_str).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::min_window;

    #[test]
    fn test_common_case_1() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        let expected = String::from("BANC");
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_common_case_2() {
        let s = String::from("a");
        let t = String::from("a");
        let expected = String::from("a");
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_edge_case_empty_s() {
        let s = String::from("");
        let t = String::from("ABC");
        let expected = String::from("");
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_no_valid_window() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("XYZ");
        let expected = String::from("");
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_t_longer_than_s() {
        let s = String::from("A");
        let t = String::from("ABC");
        let expected = String::from("");
        assert_eq!(min_window(s, t), expected);
    }

    #[test]
    fn test_multiple_valid_windows() {
        let s = String::from("ABBCABB");
        let t = String::from("AB");
        let expected = String::from("AB");
        assert_eq!(min_window(s, t), expected);
    }
}
