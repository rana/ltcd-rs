/// 12. Integer to Roman
///
/// Roman numerals are represented by seven different symbols:
/// I, V, X, L, C, D and M.
/// Symbol       Value
/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
///
/// For example, 2 is written as II in Roman numeral, just two one's
/// added together. 12 is written as XII, which is simply X + II. The
/// number 27 is written as XXVII, which is XX + V + II.
///
/// Roman numerals are usually written largest to smallest from left
/// to right. However, the numeral for four is not IIII. Instead,
/// the number four is written as IV. Because the one is before the
/// five we subtract it making four. The same principle applies to
/// the number nine, which is written as IX. There are six instances
/// where subtraction is used:
/// * I can be placed before V (5) and X (10) to make 4 and 9.
/// * X can be placed before L (50) and C (100) to make 40 and 90.
/// * C can be placed before D (500) and M (1000) to make 400 and 900.
///
/// Given an integer, convert it to a roman numeral.
///
/// Constraints:
/// * 1 <= num <= 3999

fn int_to_roman_b(mut num: i32) -> String {
    // Variables contribute to O(1) space complexity.
    let mut ret = String::with_capacity(16);
    let roman_values = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    // Loop contributes to O(n) time complexity.
    // Starting with the largest Roman numerals is key to this working.
    for (roman, value) in roman_values {
        while num >= value {
            ret.push_str(roman);
            num -= value;
        }
        if num == 0 {
            break;
        }
    }

    ret
}

fn int_to_roman_a(mut num: i32) -> String {
    let mut ret = String::with_capacity(16);
    let roman_values = [
        ("M", 1000),
        ("CM", 900),
        ("D", 500),
        ("CD", 400),
        ("C", 100),
        ("XC", 90),
        ("L", 50),
        ("XL", 40),
        ("X", 10),
        ("IX", 9),
        ("V", 5),
        ("IV", 4),
        ("I", 1),
    ];

    for (roman, value) in roman_values {
        while num >= value {
            ret.push_str(roman);
            num -= value;
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_int_to_roman_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = int_to_roman_b(t.num);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_int_to_roman_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = int_to_roman_a(t.num);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                num: 3,
                ret: "III".into(),
            },
            Tst {
                num: 58,
                ret: "LVIII".into(),
            },
            Tst {
                num: 1994,
                ret: "MCMXCIV".into(),
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        num: i32,
        ret: String,
    }
}
