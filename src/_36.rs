// Time complexity: O(1), constant number of operations for 9x9 board.
// Space complexity: O(1), constant amount of additional space,
// https://chatgpt.com/c/67080f89-72f4-8002-9f3e-cd16ce15260d
pub fn is_valid_sudoku(brd: Vec<Vec<char>>) -> bool {
    // Valid sudoku
    // Given a sudoku board.
    // Determine whether board is valid.
    // Return true if board is valid.
    // Use HashSets for rows, cols, boxes.

    use std::collections::HashSet;

    let mut rws: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut cls: Vec<HashSet<char>> = vec![HashSet::new(); 9];
    let mut bxs: Vec<HashSet<char>> = vec![HashSet::new(); 9];

    for rws_idx in 0..9 {
        for cls_idx in 0..9 {
            // Get the cell value.
            let val: char = brd[rws_idx][cls_idx];

            // Skip the dot char.
            if val == '.' {
                continue;
            }

            // Insert row entry. Check if it's the first entry.
            if !rws[rws_idx].insert(val) {
                return false;
            }

            // Insert column entry. Check if it's the first entry.
            if !cls[cls_idx].insert(val) {
                return false;
            }

            // Insert box entry. Check if it's the first entry.
            let idx: usize = (rws_idx / 3) * 3 + cls_idx / 3;
            if !bxs[idx].insert(val) {
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
