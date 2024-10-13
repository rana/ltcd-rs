// Time complexity: O(m * n), m is the number of rows. n is the number of columns.
// Space complexity: O(1), constant additional space used.
fn game_of_life(mtx: &mut Vec<Vec<i32>>) {
    // Gane of life
    // Given an MxN matrix of integers.
    // Each cell is zero or one.
    // Return the next state.
    // Apply rules:
    // `cell = 1`, `live neighbor < 2` -> `cell = 0`
    // `cell = 1`, `live neighbor = 2,3` -> `cell = 1`
    // `cell = 1`, `live neighbor > 3` -> `cell = 0`
    // `cell = 0`, `live neighbor = 3` -> `cell = 1`
    // Use two passes with in-place encoding.
    // Encoding:
    //  0: Dead to Dead
    //  1: Alive to Alive
    //  2: Alive to Dead
    //  3: Dead to Alive
    const DEAD_DEAD: i32 = 0;
    const ALIVE_ALIVE: i32 = 1;
    const ALIVE_DEAD: i32 = 2;
    const DEAD_ALIVE: i32 = 3;

    // Define cell directions to traverse.
    let dirs: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let row_len: usize = mtx.len();
    let col_len: usize = mtx[0].len();

    // First pass: determine and encode next state.
    for r in 0..row_len {
        for c in 0..col_len {
            // Calculate live count for current cell.
            let mut liv_cnt: i32 = 0;

            // Traverse directions from current cell.
            for &(dr, dc) in dirs.iter() {
                // Calculate neighbor cell index.
                let nr: isize = r as isize + dr;
                let nc: isize = c as isize + dc;

                // Check indexes in bounds.
                if nr >= 0 && nr < row_len as isize && nc >= 0 && nc < col_len as isize {
                    // Check which neighbors are alive.
                    // Encoding states: 1,2 mean alive neighbor.
                    if mtx[nr as usize][nc as usize] == ALIVE_ALIVE
                        || mtx[nr as usize][nc as usize] == ALIVE_DEAD
                    {
                        liv_cnt += 1;
                    }
                }
            }

            // Apply game rules for current cell.
            if mtx[r][c] == 1 {
                if liv_cnt < 2 || liv_cnt > 3 {
                    // Encode live to dead.
                    mtx[r][c] = 2;
                }
            } else if liv_cnt == 3 {
                mtx[r][c] = DEAD_ALIVE;
            }
        }
    }

    // Second pass: finalize states.
    for r in 0..row_len {
        for c in 0..col_len {
            match mtx[r][c] {
                ALIVE_DEAD => mtx[r][c] = 0,
                DEAD_ALIVE => mtx[r][c] = 1,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_example() {
        let mut mtx: Vec<Vec<i32>> =
            vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        game_of_life(&mut mtx); // Pass the matrix by mutable reference
        let expected: Vec<Vec<i32>> =
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]];
        assert_eq!(mtx, expected);
    }

    #[test]
    fn all_dead() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![0, 0], vec![0, 0]];
        game_of_life(&mut mtx); // Pass the matrix by mutable reference
        let expected: Vec<Vec<i32>> = vec![vec![0, 0], vec![0, 0]];
        assert_eq!(mtx, expected);
    }

    #[test]
    fn oscillation() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 0]];
        game_of_life(&mut mtx); // Pass the matrix by mutable reference
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 0]];
        assert_eq!(mtx, expected);
    }
}
