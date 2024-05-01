/// 73. Set Matrix Zeroes
///
/// Given an m x n integer matrix matrix,
/// if an element is 0, set its entire row and column to 0's.
///
/// You must do it in place.
///
/// Constraints:
/// * m == matrix.length
/// * n == matrix[0].length
/// * 1 <= m, n <= 200
/// * -231 <= matrix[i][j] <= 231 - 1

fn set_zeroes(mat: &mut Vec<Vec<i32>>) {
    // Scan matrix for state.
    // Store state in first row and first column.
    // Update matrix zeros based on determined state.
    // Time complexity: O(m), m is the number of matrix elements.
    // Space complexity: O(1), supporting variables.
    let rows = mat.len();
    let cols = mat[0].len();
    let mut fst_row_zro = false;
    let mut fst_col_zro = false;

    // Check whether the first row is zero.
    for x in 0..cols {
        if mat[0][x] == 0 {
            fst_row_zro = true;
            break;
        }
    }
    // Check whether the first column is zero.
    for y in 0..rows {
        if mat[y][0] == 0 {
            fst_col_zro = true;
            break;
        }
    }
    // Scan matrix for remaining state.
    for y in 1..rows {
        for x in 1..cols {
            if mat[y][x] == 0 {
                mat[0][x] = 0;
                mat[y][0] = 0;
            }
        }
    }
    // Set matrix zeros for rows.
    for y in 1..rows {
        if mat[y][0] == 0 {
            for x in 1..cols {
                mat[y][x] = 0;
            }
        }
    }
    // Set matrix zeros for columns.
    for x in 1..cols {
        if mat[0][x] == 0 {
            for y in 1..rows {
                mat[y][x] = 0;
            }
        }
    }
    // Set first row zeros.
    if fst_row_zro {
        for x in 0..cols {
            mat[0][x] = 0;
        }
    }
    // Set first column zeros.
    if fst_col_zro {
        for y in 0..rows {
            mat[y][0] = 0;
        }
    }
}

fn set_zeroes_b(mat: &mut Vec<Vec<i32>>) {
    let rows = mat.len();
    let cols = mat[0].len();
    let mut fst_row_zro = false;
    let mut fst_col_zro = false;

    // Check first row for zeros.
    for x in 0..cols {
        if mat[0][x] == 0 {
            fst_row_zro = true;
            break;
        }
    }
    // Check first column for zeros.
    for y in 0..rows {
        if mat[y][0] == 0 {
            fst_col_zro = true;
            break;
        }
    }
    // Check remaining matrix for zeros.
    for y in 1..rows {
        for x in 1..cols {
            if mat[y][x] == 0 {
                mat[0][x] = 0;
                mat[y][0] = 0;
            }
        }
    }
    // Zero rows.
    for y in 1..rows {
        if mat[y][0] == 0 {
            for x in 1..cols {
                mat[y][x] = 0;
            }
        }
    }
    // Zero columns.
    for x in 1..cols {
        if mat[0][x] == 0 {
            for y in 1..rows {
                mat[y][x] = 0;
            }
        }
    }
    // Zero first row.
    if fst_row_zro {
        for x in 0..cols {
            mat[0][x] = 0;
        }
    }
    // Zero first column.
    if fst_col_zro {
        for y in 0..rows {
            mat[y][0] = 0;
        }
    }
}

fn set_zeroes_a(matrix: &mut Vec<Vec<i32>>) {
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut first_row_has_zero = false;
    let mut first_col_has_zero = false;

    // Check if first row has any zeroes
    for j in 0..cols {
        if matrix[0][j] == 0 {
            first_row_has_zero = true;
            break;
        }
    }

    // Check if first column has any zeroes
    for i in 0..rows {
        if matrix[i][0] == 0 {
            first_col_has_zero = true;
            break;
        }
    }

    // Traverse the rest of the matrix and mark rows and columns
    for i in 1..rows {
        for j in 1..cols {
            if matrix[i][j] == 0 {
                matrix[i][0] = 0;
                matrix[0][j] = 0;
            }
        }
    }

    // Zero out marked rows
    for i in 1..rows {
        if matrix[i][0] == 0 {
            for j in 1..cols {
                matrix[i][j] = 0;
            }
        }
    }

    // Zero out marked columns
    for j in 1..cols {
        if matrix[0][j] == 0 {
            for i in 1..rows {
                matrix[i][j] = 0;
            }
        }
    }

    // Zero out the first row
    if first_row_has_zero {
        for j in 0..cols {
            matrix[0][j] = 0;
        }
    }

    // Zero out the first column
    if first_col_has_zero {
        for i in 0..rows {
            matrix[i][0] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::set_zeroes; // Import the function to test

    // Helper function to compare two matrices
    fn compare_matrices(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
        a == b
    }

    #[test]
    fn test_basic_case() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];

        let expected = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }

    #[test]
    fn test_multiple_zeroes() {
        let mut matrix = vec![
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 1],
        ];

        let expected = vec![
            vec![0, 1, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 0, 1],
        ];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }

    #[test]
    fn test_all_zeroes() {
        let mut matrix = vec![vec![0, 0], vec![0, 0]];

        let expected = vec![vec![0, 0], vec![0, 0]];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }

    #[test]
    fn test_single_row() {
        let mut matrix = vec![vec![1, 0, 1]];

        let expected = vec![vec![0, 0, 0]];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }

    #[test]
    fn test_single_column() {
        let mut matrix = vec![vec![1], vec![0], vec![1]];

        let expected = vec![vec![0], vec![0], vec![0]];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }

    #[test]
    fn test_no_zeroes() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }

    #[test]
    fn test_large_case() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 0, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 0, 16],
        ];

        let expected = vec![
            vec![1, 0, 0, 4],
            vec![0, 0, 0, 0],
            vec![9, 0, 0, 12],
            vec![0, 0, 0, 0],
        ];

        set_zeroes(&mut matrix);
        assert!(compare_matrices(&matrix, &expected));
    }
}
