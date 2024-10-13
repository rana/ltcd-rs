// Time complexity: O(m * n), m and n are the matrix boundries.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/6708150b-b6ec-8002-807c-7e31778dc78a
pub fn spiral_order(mtx: Vec<Vec<i32>>) -> Vec<i32> {
    // Given an m x n matrix.
    // Return all elements in spiral order.
    // Use a boundry tracking technique.

    // Initialize variables.
    // Use isize to allow decrementing boundries.
    let mut res: Vec<i32> = Vec::new();
    let mut top: isize = 0;
    let mut btm: isize = (mtx.len() - 1) as isize;
    let mut lft: isize = 0;
    let mut rht: isize = (mtx[0].len() - 1) as isize;

    // Traverse the matrix in spiral order.
    while top <= btm && lft <= rht {
        // Top: traverse from left to right.
        for idx in lft..=rht {
            res.push(mtx[top as usize][idx as usize]);
        }
        top += 1; // Move top boundry.

        // Rht: traverse top to bottom.
        for idx in top..=btm {
            res.push(mtx[idx as usize][rht as usize]);
        }
        rht -= 1; // Move right boundry.

        if top <= btm {
            // Btm: traverse from right to left.
            for idx in (lft..=rht).rev() {
                res.push(mtx[btm as usize][idx as usize]);
            }
            btm -= 1; // Move bottom boundry.
        }

        if lft <= rht {
            // Lft: traverse from bottom to top.
            for idx in (top..=btm).rev() {
                res.push(mtx[idx as usize][lft as usize]);
            }
            lft += 1; // Move left boundry.
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
