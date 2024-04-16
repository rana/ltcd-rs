/// 151. Reverse Words in a String
///
/// Given an input string s, reverse the order
/// of the words.
///
/// A word is defined as a sequence of non-space
/// characters. The words in s will be separated
/// by at least one space.
///
/// Return a string of the words in reverse order
/// concatenated by a single space.
///
/// Note that s may contain leading or trailing spaces
/// or multiple spaces between two words. The returned
/// string should only have a single space separating
/// the words. Do not include any extra spaces.
///
/// Constraints:
/// * 1 <= s.length <= 10^4
/// * s contains English letters
/// (upper-case and lower-case), digits, and spaces ' '.
/// * There is at least one word in s.

fn reverse_words_b(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}

fn reverse_words_a(s: String) -> String {
    s.split_whitespace()
        .rev()
        .fold(String::with_capacity(s.len()), |mut acc, val| {
            if !acc.is_empty() {
                acc.push(' ');
            }
            acc.push_str(val);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_reverse_words_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = reverse_words_a(t.s.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                s: "the sky is blue".into(),
                ret: "blue is sky the".into(),
            },
            Tst {
                s: "  hello world  ".into(),
                ret: "world hello".into(),
            },
            Tst {
                s: "a good   example".into(),
                ret: "example good a".into(),
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        s: String,
        ret: String,
    }
}
