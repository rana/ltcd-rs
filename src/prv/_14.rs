/// 14. Longest Common Prefix
///
/// Write a function to find the longest common prefix
/// string amongst an array of strings.
///
/// If there is no common prefix, return an empty
/// string "".
///
/// Constraints:
/// * 1 <= strs.length <= 200
/// * 0 <= strs[i].length <= 200
/// * strs[i] consists of only lowercase English letters.

fn longest_common_prefix_b(strs: Vec<String>) -> String {
    // Variables contribute to O(1) space complexity.
    let mut prefix = strs[0].clone();

    // Loop contributes to O(S) time complexity.
    for s in strs.iter().skip(1) {
        while !s.starts_with(&prefix) {
            // Remove the last character of the prefix.
            prefix.pop();
            if prefix.is_empty() {
                return "".to_string();
            }
        }
    }

    prefix
}

fn longest_common_prefix_a(mut strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut prefix = strs.pop().unwrap();
    for word in strs.iter_mut() {
        while !prefix.is_empty() {
            if word.starts_with(&prefix) {
                break;
            } else {
                prefix.pop();
            }
        }
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_longest_common_prefix_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = longest_common_prefix_b(t.strs.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_longest_common_prefix_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = longest_common_prefix_a(t.strs.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                strs: vec!["flower".into(), "flow".into(), "flight".into()],
                ret: "fl".into(),
            },
            Tst {
                strs: vec!["dog".into(), "racecar".into(), "car".into()],
                ret: "".into(),
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        strs: Vec<String>,
        ret: String,
    }
}
