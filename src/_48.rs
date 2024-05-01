/// 48. Rotate Image
///
/// You are given an n x n 2D matrix representing an image,
/// rotate the image by 90 degrees (clockwise).
///
/// You have to rotate the image in-place, which means you
/// have to modify the input 2D matrix directly. DO NOT
/// allocate another 2D matrix and do the rotation.
///
/// Constraints:
/// * n == matrix.length == matrix[i].length
/// * 1 <= n <= 20
/// * -1000 <= matrix[i][j] <= 1000

fn rotate(mat: &mut Vec<Vec<i32>>) {
    // Time complexity: O(m). m is the number of matrix elements.
    // Space complexity: O(1). A constant number of supporting variables.
    let rows = mat.len();
    let cols = mat[0].len();

    // Transpose matrix.
    for y in 0..rows {
        for x in y..cols {
            let tmp = mat[y][x];
            mat[y][x] = mat[x][y];
            mat[x][y] = tmp;
        }
    }

    // Reverse rows.
    for y in 0..rows {
        mat[y].reverse();
    }
}

fn rotate_b(mat: &mut Vec<Vec<i32>>) {
    let len = mat.len();

    // Rotates matrix to a partial position.
    // Contributes to O(m) time complexity.
    for y in 0..len {
        for x in y..len {
            let tmp = mat[y][x];
            mat[y][x] = mat[x][y];
            mat[x][y] = tmp;
        }
    }

    // Reverse to bring to 90 degree rotation.
    // Contributes to O(m) time complexity.
    for row in mat.iter_mut() {
        row.reverse();
    }
}

fn rotate_a(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();

    // Transpose the matrix
    for i in 0..n {
        for j in i..n {
            let temp = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = temp;
        }
    }

    // Reverse each row
    for row in matrix.iter_mut() {
        row.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to compare two 2D vectors
    fn matrices_are_equal(m1: &Vec<Vec<i32>>, m2: &Vec<Vec<i32>>) -> bool {
        m1.len() == m2.len() && m1.iter().zip(m2.iter()).all(|(r1, r2)| r1 == r2)
    }

    #[test]
    fn test_common_case() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];

        rotate(&mut matrix);
        assert!(matrices_are_equal(&matrix, &expected));
    }

    #[test]
    fn test_common_case_4x4() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];

        let expected = vec![
            vec![13, 9, 5, 1],
            vec![14, 10, 6, 2],
            vec![15, 11, 7, 3],
            vec![16, 12, 8, 4],
        ];

        rotate(&mut matrix);
        assert!(matrices_are_equal(&matrix, &expected));
    }

    #[test]
    fn test_single_element() {
        let mut matrix = vec![vec![1]];
        let expected = vec![vec![1]];

        rotate(&mut matrix);
        assert!(matrices_are_equal(&matrix, &expected));
    }

    #[test]
    fn test_two_by_two() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];

        let expected = vec![vec![3, 1], vec![4, 2]];

        rotate(&mut matrix);
        assert!(matrices_are_equal(&matrix, &expected));
    }

    #[test]
    fn test_symmetric_matrix() {
        let mut matrix = vec![vec![1, 2, 3], vec![2, 4, 5], vec![3, 5, 6]];

        let expected = vec![vec![3, 2, 1], vec![5, 4, 2], vec![6, 5, 3]];

        rotate(&mut matrix);
        assert!(matrices_are_equal(&matrix, &expected));
    }

    #[test]
    fn test_identical_elements() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        let expected = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];

        rotate(&mut matrix);
        assert!(matrices_are_equal(&matrix, &expected));
    }
}
