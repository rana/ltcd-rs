// Time complexity: O(n), n is the length of array height.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/6705bfc9-0998-8002-b1e5-087c26d639b6
pub fn max_area(heights: Vec<i32>) -> i32 {
    // Use a two-pointer approach.
    let mut lft: usize = 0;
    let mut rht: usize = heights.len() - 1;
    let mut max_area = 0;

    // Loop until pointers meet.
    while lft < rht {
        // Calculate current area.
        let width = (rht - lft) as i32;
        let height = heights[lft].min(heights[rht]);
        let cur_area = width * height;

        // Update  max_area if current area is larger.
        if cur_area > max_area {
            max_area = cur_area;
        }

        // Move the pointer at the shorter line inward
        // in hopes of finding a longer line.
        if heights[lft] < heights[rht] {
            lft += 1;
        } else {
            rht -= 1;
        }
    }

    max_area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(height), 49);
    }

    #[test]
    fn example2() {
        let height = vec![1, 1];
        assert_eq!(max_area(height), 1);
    }

    #[test]
    fn increasing_heights() {
        let height = vec![1, 2, 3, 4, 5];
        assert_eq!(max_area(height), 6);
    }

    #[test]
    fn same_heights() {
        let height = vec![5, 5, 5, 5, 5];
        assert_eq!(max_area(height), 20);
    }

    #[test]
    fn invalid_input() {
        let height = vec![0, 0];
        assert_eq!(max_area(height), 0);
    }
}
