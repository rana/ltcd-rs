// Time complexity: O(n), where n is the length of string s. We traverse each character once.
// Space complexity: O(n), additional space used by rows vector.
// https://chatgpt.com/c/67057c77-e334-8002-8a32-cfb71f73c826
pub fn convert(s: String, num_rows: i32) -> String {
    // Check for input minimum edge condition.
    if num_rows == 1 || s.len() <= num_rows as usize {
        return s;
    }

    // Initialize variables.
    let mut rows = vec![String::new(); s.len().min(num_rows as usize)];
    let mut cur_row = 0;
    let mut going_down = false;

    // Loop through each character.
    for c in s.chars() {
        // Append char to current row.
        rows[cur_row].push(c);
        // Change zig-zag direction at top or bottom.
        if cur_row == 0 || cur_row == num_rows as usize - 1 {
            going_down = !going_down;
        }
        // Move to the next row.
        cur_row = if going_down { cur_row + 1 } else { cur_row - 1 };
    }

    // Concat all rows.
    rows.concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 3;
        assert_eq!(convert(s, num_rows), "PAHNAPLSIIGYIR");
    }

    #[test]
    fn example2() {
        let s = String::from("PAYPALISHIRING");
        let num_rows = 4;
        assert_eq!(convert(s, num_rows), "PINALSIGYAHRPI");
    }

    #[test]
    fn single_row() {
        let s = String::from("HELLOWORLD");
        let num_rows = 1;
        assert_eq!(convert(s, num_rows), "HELLOWORLD");
    }

    #[test]
    fn num_rows_exceeds_length() {
        let s = String::from("SHORT");
        let num_rows = 10;
        assert_eq!(convert(s, num_rows), "SHORT");
    }

    #[test]
    fn empty_string() {
        let s = String::new();
        let num_rows = 3;
        assert_eq!(convert(s, num_rows), "");
    }
}
