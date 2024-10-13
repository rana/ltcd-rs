// Time complexity: O(m * n), m is the number of rows. n is the number of colunms. The matrix is traversed twice. Once to evaluate zero states. A second time to update the zero states.

pub fn set_zeroes(mtx: &mut Vec<Vec<i32>>) {
    // Set matrix zeros.
    // Given an MxN integer matrix.
    // If an element is zero, set corresponding row and cell to all zeros.
    // Do it in-place.
    // Use the first row to mark columns to be set to zero.
    // Use the first column to mark rows to be set to zero.

    // Initialize variables.
    let mut fst_row_zro: bool = false;
    let mut fst_col_zro: bool = false;
    let row_len = mtx.len();
    let col_len = mtx[0].len();

    // Check whether first row has any zeros.
    for c in 0..col_len {
        if mtx[0][c] == 0 {
            fst_row_zro = true;
            break;
        }
    }

    // Check whether first column has any zeros.
    for r in 0..row_len {
        if mtx[r][0] == 0 {
            fst_col_zro = true;
            break;
        }
    }

    // Check zero states of remaining matrix.
    for r in 1..row_len {
        for c in 1..col_len {
            if mtx[r][c] == 0 {
                // Track zero state.
                mtx[r][0] = 0;
                mtx[0][c] = 0;
            }
        }
    }

    // Set zero states for matrix.
    for r in 1..row_len {
        for c in 1..col_len {
            if mtx[r][0] == 0 || mtx[0][c] == 0 {
                mtx[r][c] = 0;
            }
        }
    }

    // Set first row zero if needed.
    if fst_row_zro {
        for c in 0..col_len {
            mtx[0][c] = 0;
        }
    }

    // Set first column zero if needed.
    if fst_col_zro {
        for r in 0..row_len {
            mtx[r][0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1]];
        set_zeroes(&mut mtx);
        assert_eq!(mtx, vec![vec![1]]);
    }

    #[test]
    fn no_zeroes() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        set_zeroes(&mut mtx);
        assert_eq!(mtx, vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn with_zeroes() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        set_zeroes(&mut mtx);
        assert_eq!(mtx, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn first_row_zero() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        set_zeroes(&mut mtx);
        assert_eq!(mtx, vec![vec![0, 0, 0], vec![0, 4, 5], vec![0, 7, 8]]);
    }

    #[test]
    fn first_column_zero() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![0, 4, 5], vec![6, 7, 8]];
        set_zeroes(&mut mtx);
        assert_eq!(mtx, vec![vec![0, 2, 3], vec![0, 0, 0], vec![0, 7, 8]]);
    }
}
