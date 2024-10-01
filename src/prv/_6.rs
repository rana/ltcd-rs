/// 6. Zigzag Conversion
///
/// The string "PAYPALISHIRING" is written in a zigzag
/// pattern on a given number of rows like this:
/// (you may want to display this pattern in a fixed
/// font for better legibility)
///
/// P   A   H   N
/// A P L S I I G
/// Y   I   R
///
/// And then read line by line: "PAHNAPLSIIGYIR"
///
/// Write the code that will take a string and
/// make this conversion given a number of rows.
///
/// Constraints:
/// * 1 <= s.length <= 1000
/// * s consists of English letters
/// (lower-case and upper-case), ',' and '.'.
/// * 1 <= numRows <= 1000

fn convert_c(s: String, num_rows: i32) -> String {
    let num_rows = num_rows as usize;

    // Check for edge cases, and return early.
    if num_rows <= 1 || s.len() <= num_rows {
        return s;
    }

    // Variables contribute to O(1) space complexity.
    let mut rows: Vec<String> = Vec::with_capacity(num_rows);
    let mut idx_row: usize = 0;
    let mut going_down = false;
    let len_str = s.len();
    // `len_cycle` is the number of characters in one zig.
    let len_cycle = (2 * num_rows) - 2;
    // `cap_row` is the maximum length of a row string.
    // Add 1 to handle a partial cycle.
    let cap_row = (len_str / len_cycle) + 1;

    // Initialize row strings with estimated capacity.
    for _ in 0..num_rows {
        rows.push(String::with_capacity(cap_row));
    }

    // Loop through characters.
    // Loop contributes to O(n) time complexity.
    for chr in s.chars() {
        rows[idx_row].push(chr);

        // Check whether to update direction.
        if idx_row == 0 || idx_row == num_rows-1 {
            going_down = !going_down;
        }

        // Update row index based on zig-zag logic.
        if going_down {
            idx_row += 1;
        } else {
            idx_row -= 1;
        }
    }

    rows.concat()
}

fn convert_b(s: String, num_rows: i32) -> String {
    if num_rows == 1 {
        return s;
    }

    // Produce row-character mappings.
    // Use rules based on challenge.
    let mut vs: Vec<_> = (0..num_rows)
        .chain((1..num_rows - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();

    // Sort row-character tuple based on row.
    vs.sort_by_key(|&(row, _)| row);

    // Collect character for final result.
    vs.into_iter().map(|(_, chr)| chr).collect()
}

fn convert_a(s: String, num_rows: i32) -> String {
    if s.len() == 1 || num_rows == 1 {
        return s;
    }

    let mut zigzags: Vec<_> = (0..num_rows)
        .chain((1..num_rows - 1).rev())
        .cycle()
        .zip(s.chars())
        .collect();

    zigzags.sort_by_key(|&(row, _)| row);

    zigzags.into_iter().map(|(_, c)| c).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_convert_c() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = convert_c(t.s.clone(), t.num_rows);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_convert_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = convert_b(t.s.clone(), t.num_rows);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_convert_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = convert_a(t.s.clone(), t.num_rows);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                s: "PAYPALISHIRING".into(),
                num_rows: 3,
                ret: "PAHNAPLSIIGYIR".into(),
            },
            Tst {
                s: "PAYPALISHIRING".into(),
                num_rows: 4,
                ret: "PINALSIGYAHRPI".into(),
            },
            Tst {
                s: "A".into(),
                num_rows: 1,
                ret: "A".into(),
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        s: String,
        num_rows: i32,
        ret: String,
    }
}
