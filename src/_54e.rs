// Time complexity: O(m * n), m is the number of rows. n is the number of columns. We traverse the matrix once.
// Space complexity: O(1), constant additional space used.
pub fn spiral_order(mtx: Vec<Vec<i32>>) -> Vec<i32> {
    // Spiral matrix
    // Given an MxN matrix.
    // Return all elements in spiral order.

    // Initialize variables.
    // Use isize to support decrementing.
    let mut res: Vec<i32> = Vec::new();
    let mut top: isize = 0;
    let mut btm: isize = mtx.len() as isize - 1;
    let mut lft: isize = 0;
    let mut rht: isize = mtx[0].len() as isize - 1;

    // Traverse spiral: top-rht-btm-lft
    while top <= btm && btm >= 0 && lft <= rht && lft >= 0 {
        // top: traverse from left to right.
        for idx in lft..=rht {
            res.push(mtx[top as usize][idx as usize]);
        }
        top += 1; // Move towards bottom.

        // rht: traverse from top to bottom.
        for idx in top..=btm {
            res.push(mtx[idx as usize][rht as usize]);
        }
        rht -= 1; // Move towards left.

        // btm: traverse from right to left.
        // Check lower bound constraint.
        if top <= btm {
            for idx in (lft..=rht).rev() {
                res.push(mtx[btm as usize][idx as usize]);
            }
            btm -= 1; // Move towards top.
        }

        // lft: traverse from bottom to top.
        // Check for lower bound constraint.
        if lft <= rht {
            for idx in (top..=btm).rev() {
                res.push(mtx[idx as usize][lft as usize]);
            }
            lft += 1; // Move towards right.
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let mtx: Vec<Vec<i32>> = vec![vec![1]];
        let res: Vec<i32> = spiral_order(mtx);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn single_row() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4]];
        let res: Vec<i32> = spiral_order(mtx);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn single_column() {
        let mtx: Vec<Vec<i32>> = vec![vec![1], vec![2], vec![3], vec![4]];
        let res: Vec<i32> = spiral_order(mtx);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn square_matrix() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let res: Vec<i32> = spiral_order(mtx);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn rectangle_matrix() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 2, 3, 4], vec![10, 11, 12, 5], vec![9, 8, 7, 6]];
        let res: Vec<i32> = spiral_order(mtx);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }
}
