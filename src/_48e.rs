// Time complexity: O(n^2), n is the width and height of the matrix. We traverse the matrix twice.
// Space complexity: O(1), constant additional space used.
pub fn rotate(mtx: &mut Vec<Vec<i32>>) {
    // Rotate image.
    // Given an NxN matrix.
    // Rotate matrix by 90 degrees in-place.
    // Transpose then reverse rows.

    let row_len: usize = mtx.len();
    let col_len: usize = mtx[0].len();

    // Transpose.
    for r in 0..row_len {
        for c in (r + 1)..col_len {
            let tmp = mtx[r][c];
            mtx[r][c] = mtx[c][r];
            mtx[c][r] = tmp;
        }
    }

    // Reverse rows.
    for r in 0..row_len {
        mtx[r].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_1x1_mtx() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1]];
        let expected: Vec<Vec<i32>> = vec![vec![1]];
        rotate(&mut mtx);
        assert_eq!(mtx, expected);
    }

    #[test]
    fn valid_2x2_mtx() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let expected: Vec<Vec<i32>> = vec![vec![3, 1], vec![4, 2]];
        rotate(&mut mtx);
        assert_eq!(mtx, expected);
    }

    #[test]
    fn valid_3x3_mtx() {
        let mut mtx: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected: Vec<Vec<i32>> = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        rotate(&mut mtx);
        assert_eq!(mtx, expected);
    }

    #[test]
    fn valid_4x4_mtx() {
        let mut mtx: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        let expected: Vec<Vec<i32>> = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ];
        rotate(&mut mtx);
        assert_eq!(mtx, expected);
    }
}
