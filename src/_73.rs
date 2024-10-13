// Time complexity: O(m * n), m is the number of rows, and n is the number of columns. We traverse the matrix twice. Once to scan for zeros, and a second time to set zeros.
// Space complexity: O(1), constant additional space is used.
// https://claude.ai/chat/d5a7cf00-19a3-4c7f-88da-2666e5da7b96
pub fn set_zeroes(mtx: &mut Vec<Vec<i32>>) {
    // Set matrix zeros.
    // Given an MxN ihteger matrix.
    // For each element zero, set corresponding row and column zero.
    // Do it in-place.
    // Use first row and first column to mark rows and columns.

    // Initialize variables.
    let row_len = mtx.len();
    let col_len = mtx[0].len();
    let mut fst_row_zro: bool = false;
    let mut fst_col_zro: bool = false;

    // Check whether first row contains zero.
    for idx in 0..col_len {
        if mtx[0][idx] == 0 {
            fst_row_zro = true;
            break;
        }
    }

    // Check whether first column contains zero.
    for idx in 0..row_len {
        if mtx[idx][0] == 0 {
            fst_col_zro = true;
            break;
        }
    }

    // Use first row and first column to mark presence of zero.
    for row in 1..row_len {
        for col in 1..col_len {
            if mtx[row][col] == 0 {
                // Mark both row and column zero.
                mtx[row][0] = 0;
                mtx[0][col] = 0;
            }
        }
    }

    // Set matrix based on recorded markers.
    for row in 1..row_len {
        for col in 1..col_len {
            if mtx[row][0] == 0 || mtx[0][col] == 0 {
                mtx[row][col] = 0;
            }
        }
    }

    // Set first row zero if needed.
    if fst_row_zro {
        for col in 0..col_len {
            mtx[0][col] = 0;
        }
    }

    // Set first column zero if needed.
    if fst_col_zro {
        for row in 0..row_len {
            mtx[row][0] = 0;
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
