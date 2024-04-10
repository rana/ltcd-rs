use std::collections::HashMap;

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
pub fn min_window_b(s: String, t: String) -> String {
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

pub fn min_window_b_mcr(s: String, t: String) -> String {
    /// `Wnd` is a window with index ranges.
    #[derive(Default, Debug, Clone, Copy)]
    struct Wnd {
        /// `lft` is the left index of a window.
        /// Use `u32` as an optimization.
        lft: u32,
        /// `rht` is the right index of a window.
        /// Use `u32` as an optimization.
        rht: u32,
    }

    // Create a character frequency map.
    // Use an array due to the known character range.
    // b'A':65   b'z':122
    // Add one to the size because b'z' is an inclusive index.
    let mut cnts = [0i32; (b'z' - b'A' + 1) as usize];

    // Use a macro as an optimization.
    // Allows u8 as a key to a frequency count.
    macro_rules! cnts  {
            [$key:expr] => (cnts[($key - b'A') as usize])
        }

    // Count the frequency of valid characters.
    t.bytes().for_each(|byt| cnts![byt] += 1);

    // Initialize sliding window variables.
    // Use `u32` as an optimization.
    let byts = s.as_bytes();
    let str_len = byts.len() as u32;
    let mut tgt_wnd_len = t.len() as u32;
    let mut min: Option<Wnd> = None;
    let mut cur = Wnd::default();

    // Work towards a target window length of zero.
    while cur.rht < str_len {
        // Expand the window on the right side.

        // Check whether we can approach a valid window.
        // Check whether we have a valid frequency count of a valid character.
        if cnts![byts[cur.rht as usize]] > 0 {
            // Decrement the target window length.
            // Target window length of zero means we have a success condition.
            tgt_wnd_len -= 1;
        }

        // Decrement the frequency count of the character at the right index.
        // Valid characters will approach zero.
        // Invalid characters will approach i32::MIN.
        cnts![byts[cur.rht as usize]] -= 1;

        // Advance the right index of the window.
        cur.rht += 1;

        // Check whether we have a valid window.
        // Valid window is when `tgt_wnd_len == 0`.
        while tgt_wnd_len == 0 {
            // Check for a minimum window.
            if cur.rht - cur.lft < min.map_or(u32::MAX, |min| min.rht - min.lft) {
                min = Some(cur);
            }

            // Shrink the window on the left side.

            // Increment the count of the left character.
            // Valid characters will have a character count above zero.
            // Invalid characters will approach zero.
            cnts![byts[cur.lft as usize]] += 1;

            // Check whether we can increase the target window length.
            if cnts![byts[cur.lft as usize]] > 0 {
                // Increment the target window length.
                // Moves away from a valid window.
                tgt_wnd_len += 1;
            }

            // Increment the left index.
            cur.lft += 1;
        }
    }

    // Return the minimum window string; or, an empty string.
    min.map_or("".into(), |min| {
        s[min.lft as usize..min.rht as usize].into()
    })
}

pub fn min_window_b_mcr_idx_usize(s: String, t: String) -> String {
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

    // Use a macro as an optimization.
    // Allows u8 as a key to a frequency count.
    macro_rules! cnts  {
            [$key:expr] => (cnts[($key - b'A') as usize])
        }

    // Count the frequency of valid characters.
    t.bytes().for_each(|byt| cnts![byt] += 1);

    // Initialize sliding window variables.
    // Use `u32` as an optimization.
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

pub fn min_window_b_arr(s: String, t: String) -> String {
    /// `Wnd` is a window with index ranges.
    #[derive(Default, Debug, Clone, Copy)]
    struct Wnd {
        /// `lft` is the left index of a window.
        /// Use `u32` as an optimization.
        lft: u32,
        /// `rht` is the right index of a window.
        /// Use `u32` as an optimization.
        rht: u32,
    }

    // Create a character frequency map.
    // Use an array due to the known character range.
    // b'A':65   b'z':122
    // Add one to the size because b'z' is an inclusive index.
    let mut cnts = [0i32; (b'z' - b'A' + 1) as usize];

    // Use a macro as an optimization.
    // Allows u8 as a key to a frequency count.
    // macro_rules! cnts  {
    //         [$key:expr] => (cnts[($key - b'A') as usize])
    //     }

    // Count the frequency of valid characters.
    t.bytes().for_each(|byt| cnts[(byt - b'A') as usize] += 1);

    // Initialize sliding window variables.
    // Use `u32` as an optimization.
    let byts = s.as_bytes();
    let str_len = byts.len() as u32;
    let mut tgt_wnd_len = t.len() as u32;
    let mut min: Option<Wnd> = None;
    let mut cur = Wnd::default();

    // Work towards a target window length of zero.
    while cur.rht < str_len {
        // Expand the window on the right side.

        // Check whether we can approach a valid window.
        // Check whether we have a valid frequency count of a valid character.
        if cnts[(byts[cur.rht as usize] - b'A') as usize] > 0 {
            // Decrement the target window length.
            // Target window length of zero means we have a success condition.
            tgt_wnd_len -= 1;
        }

        // Decrement the frequency count of the character at the right index.
        // Valid characters will approach zero.
        // Invalid characters will approach i32::MIN.
        cnts[(byts[cur.rht as usize] - b'A') as usize] -= 1;

        // Advance the right index of the window.
        cur.rht += 1;

        // Check whether we have a valid window.
        // Valid window is when `tgt_wnd_len == 0`.
        while tgt_wnd_len == 0 {
            // Check for a minimum window.
            if cur.rht - cur.lft < min.map_or(u32::MAX, |min| min.rht - min.lft) {
                min = Some(cur);
            }

            // Shrink the window on the left side.

            // Increment the count of the left character.
            // Valid characters will have a character count above zero.
            // Invalid characters will approach zero.
            cnts[(byts[cur.lft as usize] - b'A') as usize] += 1;

            // Check whether we can increase the target window length.
            if cnts[(byts[cur.lft as usize] - b'A') as usize] > 0 {
                // Increment the target window length.
                // Moves away from a valid window.
                tgt_wnd_len += 1;
            }

            // Increment the left index.
            cur.lft += 1;
        }
    }

    // Return the minimum window string; or, an empty string.
    min.map_or("".into(), |min| {
        s[min.lft as usize..min.rht as usize].into()
    })
}

pub fn min_window_a(s: String, t: String) -> String {
    // Algorithm: Sliding window with two pointers.

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

    // Transform characters to bytes for time efficency and space efficency.
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
    use super::*;
    use anyhow::{bail, Result};
    use ben::*;
    use std::fmt;
    use Lbl::*;

    #[test]
    fn tst_min_window_b() {
        for tst in tsts() {
            assert_eq!(min_window_b(tst.s, tst.t), tst.ret);
        }
    }

    #[test]
    fn tst_min_window_b_mcr() {
        for tst in tsts() {
            assert_eq!(min_window_b_mcr(tst.s, tst.t), tst.ret);
        }
    }

    #[test]
    fn tst_min_window_b_mcr_idx_usize() {
        for tst in tsts() {
            assert_eq!(min_window_b_mcr_idx_usize(tst.s, tst.t), tst.ret);
        }
    }

    #[test]
    fn tst_min_window_b_arr() {
        for tst in tsts() {
            assert_eq!(min_window_b_arr(tst.s, tst.t), tst.ret);
        }
    }

    #[test]
    fn tst_min_window_a() {
        for tst in tsts() {
            assert_eq!(min_window_a(tst.s, tst.t), tst.ret);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                s: "ADOBECODEBANC".into(),
                t: "ABC".into(),
                ret: "BANC".into(),
            },
            Tst {
                s: "a".into(),
                t: "a".into(),
                ret: "a".into(),
            },
            Tst {
                s: "a".into(),
                t: "aa".into(),
                ret: "".into(),
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        s: String,
        t: String,
        ret: String,
    }

    #[test]
    fn mtr() {
        let mut stdy = Stdy::new();
        let itr: u16 = 64;

        // Register metric functions.

        stdy.reg_bld(&[MinWin, A], |x| {
            x.ins_prm(Len(1), |tme| {
                let tst = tsts()[0].clone();
                tme.borrow_mut().start();
                let ret = min_window_a(tst.s, tst.t);
                tme.borrow_mut().stop();
                ret
            });
        });
        stdy.reg_bld(&[MinWin, B, Mcr], |x| {
            x.ins_prm(Len(1), |tme| {
                let tst = tsts()[0].clone();
                tme.borrow_mut().start();
                let ret = min_window_b_mcr(tst.s, tst.t);
                tme.borrow_mut().stop();
                ret
            });
        });
        stdy.reg_bld(&[MinWin, B, Mcr, Idx], |x| {
            x.ins_prm(Len(1), |tme| {
                let tst = tsts()[0].clone();
                tme.borrow_mut().start();
                let ret = min_window_b_mcr_idx_usize(tst.s, tst.t);
                tme.borrow_mut().stop();
                ret
            });
        });
        stdy.reg_bld(&[MinWin, B, Arr], |x| {
            x.ins_prm(Len(1), |tme| {
                let tst = tsts()[0].clone();
                tme.borrow_mut().start();
                let ret = min_window_b_arr(tst.s, tst.t);
                tme.borrow_mut().stop();
                ret
            });
        });

        // Define function queries.
        let mut qry = QryBld::new();
        let a_id = qry.sel(&[MinWin, A]);
        let b_mcr_id = qry.sel(&[MinWin, B, Mcr]);
        let b_mcr_idx_id = qry.sel(&[MinWin, B, Mcr, Idx]);
        let b_arr_id = qry.sel(&[MinWin, B, Arr]);

        qry.cmp(a_id, b_mcr_id);
        qry.cmp(b_arr_id, b_mcr_id);
        qry.cmp(b_mcr_idx_id, b_mcr_id);

        // Run metric functions.
        stdy.run(qry, itr).expect("err");
    }

    /// Benchmark labels.
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Lbl {
        A,
        B,
        MinWin,
        Arr,
        Mcr,
        Idx,
        Len(u32),
    }
    impl fmt::Display for Lbl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                A => write!(f, "a"),
                B => write!(f, "b"),
                MinWin => write!(f, "minwin"),
                Arr => write!(f, "arr"),
                Mcr => write!(f, "mcr"),
                Idx => write!(f, "idx"),
                Len(x) => {
                    if f.alternate() {
                        write!(f, "len")
                    } else {
                        write!(f, "len({})", x)
                    }
                }
            }
        }
    }
    impl EnumStructVal for Lbl {
        fn val(&self) -> Result<u32> {
            match *self {
                Len(x) => Ok(x),
                _ => bail!("label '{}' isn't a struct enum", self),
            }
        }
    }
    impl Label for Lbl {}
}
