// Time complexity: O(1), constant time to traverse matrix.
// Space complexity: O(1), constant additional space used.
pub fn is_valid_sudoku(brd: Vec<Vec<char>>) -> bool {
    // Valid sudoku
    // Determine whether a 9x9 sudoku board is valid.
    // Rules:
    //  Each row has a single digit 1-9.
    //  Each column has a single digit 1-9.
    //  Each box has a single digit 1-9.
    // Use HashSet to validate conditions.

    use std::collections::HashSet;

    let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut boxs: Vec<HashSet<char>> = vec![HashSet::new(); 9];

    // Validate rows.
    for r in 0..9 {
        for c in 0..9 {
            // Get character at current cell.
            let chr: char = brd[r][c];

            // Skip non-digit character.
            if chr == '.' {
                continue;
            }

            // Insert row character. Check sudoku duplicate rule.
            if !rows[r].insert(chr) {
                return false;
            }

            // Insert col character. Check sudoku rule.
            if !cols[c].insert(chr) {
                return false;
            }

            // Insert box character. Check sudoku rule.
            let idx: usize = (r / 3) * 3 + (c / 3);
            if !boxs[idx].insert(chr) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_board() {
        let brd: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(is_valid_sudoku(brd));
    }

    #[test]
    fn invalid_row() {
        let brd: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(brd));
    }

    #[test]
    fn invalid_column() {
        let brd: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '9', '.', '.', '8', '.', '.', '7', '9'], // Duplicate '9' in last column
        ];
        assert!(!is_valid_sudoku(brd));
    }

    #[test]
    fn invalid_subgrid() {
        let brd: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '3', '1', '9', '5', '.', '.', '.'], // Duplicate '3' in top-left sub-grid
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(!is_valid_sudoku(brd));
    }
}
