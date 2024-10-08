use core::num;

// Time complexity: O(n), n is the length of string s.
// Space complexity: O(n), n is the number of chars stored in rows array.
pub fn convert(s: String, num_rows: i32) -> String {
    // Check for input minimum condition.
    if num_rows == 1 {
        return s;
    }

    // Initialize variables.
    let mut rows = vec![String::new(); s.len().min(num_rows as usize)];
    let mut row = 0;
    let mut going_down = false;

    // Loop through each character.
    for c in s.chars() {
        // Append char to row.
        rows[row].push(c);

        // Update direction.
        if row == 0 || row == num_rows as usize - 1 {
            going_down = !going_down;
        }

        // Increment row index.
        row = if going_down { row + 1 } else { row - 1 };
    }

    // Concatenate final string.
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
