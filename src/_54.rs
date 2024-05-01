/// 54. Spiral Matrix
///
/// Given an m x n matrix, return all elements of
/// the matrix in spiral order.
///
/// Constraints:
/// * m == matrix.length
/// * n == matrix[i].length
/// * 1 <= m, n <= 10
/// * -100 <= matrix[i][j] <= 100

fn spiral_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::with_capacity(mat.len() * mat[0].len());
    let mut lft: usize = 0;
    let mut rht: usize = mat[0].len();
    let mut top: usize = 0;
    let mut btm: usize = mat.len();

    // Loop through matrix: top-rht-btm-lft.
    while lft < rht && top < btm {
        // Traverse top row.
        for x in lft..rht {
            ret.push(mat[top][x]);
        }
        top += 1;

        // Traverse right column.
        for y in top..btm {
            ret.push(mat[y][rht - 1]);
        }
        rht -= 1;

        // Traverse bottom row.
        if top < btm {
            for x in (lft..rht).rev() {
                ret.push(mat[btm - 1][x]);
            }
            btm -= 1;
        }

        // Traverse left column.
        if lft < rht {
            for y in (top..btm).rev() {
                ret.push(mat[y][lft]);
            }
            lft += 1;
        }
    }

    ret
}

fn spiral_order_b(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let cell_cnt = mat[0].len() * mat.len();
    let mut ret = Vec::with_capacity(cell_cnt);
    let mut lft: usize = 0;
    let mut rht: usize = mat[0].len();
    let mut top: usize = 0;
    let mut btm: usize = mat.len();

    // Traversal: top-rht-btm-lft
    while lft < rht && top < btm {
        // Traverse top row.
        for col in lft..rht {
            ret.push(mat[top][col]);
        }
        top += 1;

        // Traverse right column.
        for row in top..btm {
            ret.push(mat[row][rht - 1]);
        }
        rht -= 1;

        // Traverse bottom row.
        if top < btm {
            for col in (lft..rht).rev() {
                ret.push(mat[btm - 1][col]);
            }
            btm -= 1;
        }

        // Traverse left column.
        if lft < rht {
            for row in (top..btm).rev() {
                ret.push(mat[row][lft]);
            }
            lft += 1;
        }
    }

    ret
}

fn spiral_order_a(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (mut top, mut bottom) = (0, matrix.len());
    let (mut left, mut right) = (0, matrix[0].len());
    let mut result = Vec::new();

    while top < bottom && left < right {
        for i in left..right {
            result.push(matrix[top][i]);
        }
        top += 1;

        for i in top..bottom {
            result.push(matrix[i][right - 1]);
        }
        right -= 1;

        if top < bottom {
            for i in (left..right).rev() {
                result.push(matrix[bottom - 1][i]);
            }
            bottom -= 1;
        }

        if left < right {
            for i in (top..bottom).rev() {
                result.push(matrix[i][left]);
            }
            left += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_element() {
        let matrix = vec![vec![42]];
        assert_eq!(spiral_order(matrix), vec![42]);
    }

    #[test]
    fn test_single_row() {
        let matrix = vec![vec![1, 2, 3, 4, 5]];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_single_column() {
        let matrix = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_rectangular_matrix() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        assert_eq!(
            spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
    }

    #[test]
    fn test_square_matrix() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_increasing_numbers() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(spiral_order(matrix), vec![1, 2, 3, 6, 9, 8, 7, 4, 5]);
    }

    #[test]
    fn test_large_square_matrix() {
        let matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        assert_eq!(
            spiral_order(matrix),
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
        );
    }
}
