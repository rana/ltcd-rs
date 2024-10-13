// Time complexity: O(n^2), where n is the width and height of the matrix. We traverse once to transpose. We traverse once to reverse rows.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/67081b92-17c0-8002-a232-776466f610c4
pub fn rotate(mtx: &mut Vec<Vec<i32>>) {
    // Rotate image.
    // Given an NxN matrix.
    // Rotate by 90 degrees clockwise in-place.
    // Transpose and reverse to achieve.

    let len = mtx.len();

    // Transpose.
    for n in 0..len {
        for m in (n + 1)..len {
            let tmp: i32 = mtx[n][m];
            mtx[n][m] = mtx[m][n];
            mtx[m][n] = tmp;
        }
    }

    // Reverse rows.
    for row in mtx.iter_mut() {
        row.reverse();
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
