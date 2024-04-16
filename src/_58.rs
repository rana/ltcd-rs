/// 58. Length of Last Word
///
/// Given a string s consisting of words and spaces,
/// return the length of the last word in the string.
///
/// A word is a maximal substring consisting of
/// non-space characters only.
///
/// Constraints:
/// * 1 <= s.length <= 104
/// * s consists of only English letters and spaces ' '.
/// * There will be at least one word in s.

fn length_of_last_word_b(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
}

fn length_of_last_word_a(s: String) -> i32 {
    let s = s.trim();
    let len = s.len();
    let byts = s.as_bytes();
    const SPC: u8 = b' ';

    // Search from the last index for the first ' ' char.
    // Search for last char to avoid trailing spaces.
    for idx in (0..len - 1).rev() {
        if byts[idx] == SPC {
            return (len - idx - 1) as i32;
        }
    }

    len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_length_of_last_word_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = length_of_last_word_a(t.s.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                s: "Hello World".into(),
                ret: 5,
            },
            Tst {
                s: "   fly me   to   the moon  ".into(),
                ret: 4,
            },
            Tst {
                s: "luffy is still joyboy".into(),
                ret: 6,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        s: String,
        ret: i32,
    }
}
