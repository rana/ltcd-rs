/// 36. Valid Sudoku
///
/// Determine if a 9 x 9 Sudoku board is valid.
/// Only the filled cells need to be validated
/// according to the following rules:
///
/// 1. Each row must contain the digits 1-9
/// without repetition.
///
/// 2. Each column must contain the digits 1-9
/// without repetition.
///
/// 3. Each of the nine 3 x 3 sub-boxes of the
/// grid must contain the digits 1-9 without repetition.
///
/// Note:
/// * A Sudoku board (partially filled) could be
/// valid but is not necessarily solvable.
/// * Only the filled cells need to be validated
/// according to the mentioned rules.
///
/// Constraints:
/// * board.length == 9
/// * board[i].length == 9
/// * board[i][j] is a digit 1-9 or '.'.

fn is_valid_sudoku(brd: Vec<Vec<char>>) -> bool {
    // Time complexity: O(1), a constant number of matrix elements 
    // is specified in the challenge 9x9=81.
    // Space complexity: O(1), constant number of supporting variables.

    // `BRD_LEN` is the width and height of the Sudoku board.
    const BRD_LEN: usize = 9;
    const CHR_SKP: char = '.';

    // Create an array of HashSet for each row, column, and box.
    use std::collections::HashSet;
    let mut rows = vec![HashSet::<char>::new(); BRD_LEN];
    let mut cols = vec![HashSet::<char>::new(); BRD_LEN];
    let mut boxs = vec![HashSet::<char>::new(); BRD_LEN];

    // Loop through each matrix element.
    for y in 0..BRD_LEN {
        for x in 0..BRD_LEN {
            // Get the current character.
            let chr = brd[y][x];
            // Check whether to skip the character.
            if chr == CHR_SKP {
                continue;
            }

            // Calculate the box index.
            let b = ((y / 3) * 3) + (x / 3);

            // Insert the character into a row, column, and box.
            // Check whether the character was already inserted.
            if !rows[y].insert(chr) || !cols[x].insert(chr) || !boxs[b].insert(chr) {
                // If there is a duplicate insertion,
                // that means the Sudoku board is invalid.
                return false;
            }
        }
    }

    true
}

fn is_valid_sudoku_b(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    let mut cols = vec![HashSet::new(); 9];
    let mut rows = vec![HashSet::new(); 9];
    let mut boxs = vec![HashSet::new(); 9];

    for idx_row in 0..9 {
        for idx_col in 0..9 {
            // Get the current character.
            let c = board[idx_row][idx_col];
            // Check for the skip character.
            if c == '.' {
                continue;
            }

            // Calculate the box index.
            let idx_box = ((idx_row / 3) * 3) + idx_col / 3;
            println!("idx_row:{idx_row} idx_col:{idx_col} idx_box:{idx_box}");
            println!("((idx_row / 3) * 3):{}", ((idx_row / 3) * 3));

            // Attempt to insert the character.
            if !rows[idx_col].insert(c) || !cols[idx_row].insert(c) || !boxs[idx_box].insert(c) {
                return false;
            }
        }
    }

    true
}

fn is_valid_sudoku_a(board: Vec<Vec<char>>) -> bool {
    use std::collections::HashSet;

    let mut rows = vec![HashSet::new(); 9];
    let mut cols = vec![HashSet::new(); 9];
    let mut boxes = vec![HashSet::new(); 9];

    for i in 0..9 {
        for j in 0..9 {
            let c = board[i][j];
            if c == '.' {
                continue;
            }

            let box_index = (i / 3) * 3 + j / 3;

            if !rows[i].insert(c) || !cols[j].insert(c) || !boxes[box_index].insert(c) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_from_chars(char_vec: Vec<&str>) -> Vec<Vec<char>> {
        char_vec.iter().map(|&row| row.chars().collect()).collect()
    }

    #[test]
    fn test_valid_sudoku_filled() {
        let board = board_from_chars(vec![
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ]);
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn test_valid_sudoku_partially_filled() {
        let board = board_from_chars(vec![
            "53.......",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_row() {
        let board = board_from_chars(vec![
            "533678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ]);
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_column() {
        let mut board = board_from_chars(vec![
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ]);
        board[2][0] = '5'; // Introducing duplicate '5' in the first column
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_sudoku_duplicate_in_box() {
        let mut board = board_from_chars(vec![
            "534678912",
            "672195348",
            "198342567",
            "859761423",
            "426853791",
            "713924856",
            "961537284",
            "287419635",
            "345286179",
        ]);
        board[0][2] = '3'; // Introducing duplicate '3' in the first 3x3 box
        assert!(!is_valid_sudoku(board));
    }

    #[test]
    fn test_empty_board() {
        let board = board_from_chars(vec![
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
            ".........",
        ]);
        assert!(is_valid_sudoku(board));
    }

    #[test]
    fn test_invalid_sudoku_empty_and_duplicates() {
        let mut board = board_from_chars(vec![
            "5........",
            "6..195...",
            ".98....6.",
            "8...6...3",
            "4..8.3..1",
            "7...2...6",
            ".6....28.",
            "...419..5",
            "....8..79",
        ]);
        board[8][8] = '5'; // Introducing duplicate '5' in the bottom right 3x3 box
        assert!(!is_valid_sudoku(board));
    }
}
