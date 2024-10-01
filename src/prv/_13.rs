/// 13. Roman to Integer
///
/// Roman numerals are represented by seven different symbols:
/// I, V, X, L, C, D and M.
///
/// For example, 2 is written as II in Roman numeral, just two
/// ones added together. 12 is written as XII, which is simply
/// X + II. The number 27 is written as XXVII, which is
/// XX + V + II.
///
/// Roman numerals are usually written largest to smallest from
/// left to right. However, the numeral for four is not IIII.
/// Instead, the number four is written as IV. Because the one
/// is before the five we subtract it making four. The same
/// principle applies to the number nine, which is written as
/// IX. There are six instances where subtraction is used:
/// * I can be placed before V (5) and X (10) to make 4 and 9.
/// * X can be placed before L (50) and C (100) to make 40 and 90.
/// * C can be placed before D (500) and M (1000) to make 400 and 900.
///
/// * Given a roman numeral, convert it to an integer.
///
/// Constraints:
/// * 1 <= s.length <= 15
/// * s contains only the characters ('I', 'V', 'X', 'L', 'C', 'D', 'M').
/// * It is guaranteed that s is a valid roman numeral in the
/// range [1, 3999].

fn roman_to_int_c(s: String) -> i32 {
    use std::collections::HashMap;

    let map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();
    let byts = s.as_bytes();
    let mut idx: usize = 0;
    let mut ret: i32 = 0;

    while idx < byts.len() {
        let cur_val = map[&(byts[idx] as char)];
        let nxt_val = if idx + 1 < byts.len() {
            map[&(byts[idx + 1] as char)]
        } else {
            0
        };

        if cur_val < nxt_val {
            ret -= cur_val;
        } else {
            ret += cur_val;
        }

        idx += 1;
    }

    ret
}

fn roman_to_int_b(s: String) -> i32 {
    use std::collections::HashMap;

    // Variables contribute to O(1) space complexity.
    // Create a map of the Roman numerals to base 10 digits.
    let map: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();
    let byts = s.as_bytes();
    let mut idx: usize = 0;
    let mut ret: i32 = 0;

    // Loop through each character.
    // Loop contributes to O(n) time complexity.
    while idx < byts.len() {
        let cur_val = map[&(byts[idx] as char)];
        let nxt_val = if idx + 1 < byts.len() {
            map[&(byts[idx + 1] as char)]
        } else {
            0
        };

        if cur_val < nxt_val {
            ret -= cur_val;
        } else {
            ret += cur_val;
        }

        idx += 1;
    }

    ret
}

fn roman_to_int_a(s: String) -> i32 {
    let mut ret: i32 = 0;

    // `prv` expression is two less due to eager adding approach.
    let mut prv: u8 = 0;
    for &byt in s.as_bytes() {
        ret += match byt {
            b'I' => 1,
            b'V' if prv == b'I' => 3,
            b'V' => 5,
            b'X' if prv == b'I' => 8,
            b'X' => 10,
            b'L' if prv == b'X' => 30,
            b'L' => 50,
            b'C' if prv == b'X' => 80,
            b'C' => 100,
            b'D' if prv == b'C' => 300,
            b'D' => 500,
            b'M' if prv == b'C' => 800,
            b'M' => 1000,
            _ => unsafe { std::hint::unreachable_unchecked() },
        };

        prv = byt;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_roman_to_int_c() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = roman_to_int_c(t.s.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_roman_to_int_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = roman_to_int_b(t.s.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_roman_to_int_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = roman_to_int_a(t.s.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                s: "III".into(),
                ret: 3,
            },
            Tst {
                s: "LVIII".into(),
                ret: 58,
            },
            Tst {
                s: "MCMXCIV".into(),
                ret: 1994,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        s: String,
        ret: i32,
    }
}
