/// 289. Game of Life
///
/// According to Wikipedia's article: "The Game of Life,
/// also known simply as Life, is a cellular automaton
/// devised by the British mathematician John Horton Conway
/// in 1970."
///
/// The board is made up of an m x n grid of cells, where
/// each cell has an initial state: live (represented by a 1)
/// or dead (represented by a 0). Each cell interacts with its
/// eight neighbors (horizontal, vertical, diagonal) using
/// the following four rules (taken from the above
/// Wikipedia article):
///
/// 1. Any live cell with fewer than two live neighbors
/// dies as if caused by under-population.
///
/// 2. Any live cell with two or three live neighbors
/// lives on to the next generation.
///
/// 3. Any live cell with more than three live neighbors
/// dies, as if by over-population.
///
/// 4. Any dead cell with exactly three live neighbors
/// becomes a live cell, as if by reproduction.
///
/// The next state is created by applying the above rules
/// simultaneously to every cell in the current state,
/// where births and deaths occur simultaneously. Given
/// the current state of the m x n grid board, return the
/// next state.
///
/// Constraints:
/// * m == board.length
/// * n == board[i].length
/// * 1 <= m, n <= 25
/// * board[i][j] is 0 or 1.

fn game_of_life(brd: &mut Vec<Vec<i32>>) {
    let rows = brd.len();
    let cols = brd[0].len();
    let orig = brd.clone();

    // Create function to count live neighbors.
    let cnt_liv_nei = |y: usize, x: usize| -> i32 {
        let mut cnt: i32 = 0;

        // Search neighboring cells.
        // Include horizontal, vertical, and diagonal.
        let y_min = y.saturating_sub(1);
        let y_max = (y + 1).min(rows - 1);
        for y_cur in y_min..=y_max {
            let x_min = x.saturating_sub(1);
            let x_max = (x + 1).min(cols - 1);
            for x_cur in x_min..=x_max {
                if (y, x) != (y_cur, x_cur) && orig[y_cur][x_cur] == 1 {
                    // Found a live neighbor cell.
                    cnt += 1;
                }
            }
        }

        cnt
    };

    // Loop through each matrix cell.
    // Contributes to O(m) time complexity.
    for y in 0..rows {
        for x in 0..cols {
            let liv_cnt = cnt_liv_nei(y, x);

            // Apply game rules.
            if orig[y][x] == 1 {
                // Apply live cell rules.
                if !(2..=3).contains(&liv_cnt) {
                    // Cell dies.
                    // Update board for next state.
                    brd[y][x] = 0;
                }
            } else {
                // Apply dead cell rules.
                if liv_cnt == 3 {
                    // Cell lives.
                    // Update board for next state.
                    brd[y][x] = 1;
                }
            }
        }
    }
}

fn game_of_life_a(board: &mut Vec<Vec<i32>>) {
    use std::cmp;

    let rows = board.len();
    if rows == 0 {
        return;
    }
    let cols = board[0].len();

    // Copy the board to create an auxiliary state
    let original = board.clone();

    // Helper function to count live neighbors
    let count_live_neighbors = |x: usize, y: usize| -> i32 {
        let mut live_neighbors = 0;
        for i in
            cmp::max(0, x as i32 - 1) as usize..=cmp::min(rows as i32 - 1, x as i32 + 1) as usize
        {
            for j in cmp::max(0, y as i32 - 1) as usize
                ..=cmp::min(cols as i32 - 1, y as i32 + 1) as usize
            {
                if (i, j) != (x, y) && original[i][j] == 1 {
                    live_neighbors += 1;
                }
            }
        }
        live_neighbors
    };

    // Apply the rules to each cell
    for x in 0..rows {
        for y in 0..cols {
            let live_neighbors = count_live_neighbors(x, y);

            // Apply Game of Life rules
            if original[x][y] == 1 {
                if live_neighbors < 2 || live_neighbors > 3 {
                    board[x][y] = 0; // Cell dies
                }
            } else {
                if live_neighbors == 3 {
                    board[x][y] = 1; // Cell becomes alive
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_boards_equal(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) {
        assert_eq!(a.len(), b.len());
        for (row_a, row_b) in a.iter().zip(b) {
            assert_eq!(row_a, row_b);
        }
    }

    #[test]
    fn test_single_cell() {
        let mut board = vec![vec![1]];
        let expected = vec![vec![0]]; // Single cell dies due to underpopulation
        game_of_life(&mut board);
        assert_boards_equal(&board, &expected);
    }

    #[test]
    fn test_oscillator() {
        let mut board = vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        let expected = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]]; // Blinker oscillator after one step
        game_of_life(&mut board);
        assert_boards_equal(&board, &expected);
    }

    #[test]
    fn test_block() {
        let mut board = vec![vec![1, 1], vec![1, 1]];
        let expected = vec![vec![1, 1], vec![1, 1]]; // Block stays the same
        game_of_life(&mut board);
        assert_boards_equal(&board, &expected);
    }

    #[test]
    fn test_edge_cases() {
        let mut board = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        game_of_life(&mut board);
        assert_boards_equal(&board, &expected);
    }
}
