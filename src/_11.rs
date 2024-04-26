/// 11. Container With Most Water
///
/// You are given an integer array height of length n.
/// There are n vertical lines drawn such that the two
/// endpoints of the ith line are (i, 0) and (i, height[i]).
///
/// Find two lines that together with the x-axis form a
/// container, such that the container contains the most water.
///
/// Return the maximum amount of water a container can store.
///
/// Notice that you may not slant the container.
///
/// Constraints:
/// * n == height.length
/// * 2 <= n <= 10^5
/// * 0 <= height[i] <= 10^4

fn max_area(heights: Vec<i32>) -> i32 {
    // Two-pointers, Local Optimization
    
    // Variables contribute to O(1) space complexity.
    let mut lft: usize = 0;
    let mut rht: usize = heights.len() - 1;
    let mut max_area: i32 = 0;

    // Loop contributes to O(n) time complexity.
    while lft < rht {
        // The width is based on the indexes themselves.
        let width = (rht - lft) as i32;
        
        // The height is based on the value in the array.
        // Use the minimum side height based on the 
        // concept of a water container.
        let height = heights[lft].min(heights[rht]);
    
        // Compare the current area to previous areas.
        // Select the maximum area.
        max_area = max_area.max(width * height);

        // Continue by looking for larger heights.
        if heights[lft] < heights[rht] {
            lft += 1;
        } else {
            rht -= 1;
        }
    }

    max_area
}

fn max_area_b(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);
    let mut max_area = 0;

    while left < right {
        let width = (right - left) as i32;
        let current_height = height[left].min(height[right]);
        max_area = max_area.max(width * current_height);

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_area
}

fn max_area_a(heights: Vec<i32>) -> i32 {
    let mut lft: usize = 0;
    let mut rht: usize = heights.len() - 1;
    let mut vol_max: i32 = 0;
    while lft < rht {
        let height = heights[lft].min(heights[rht]);
        let vol = height * (rht - lft) as i32;
        vol_max = vol_max.max(vol);
        if heights[lft] < heights[rht] {
            lft += 1;
        } else {
            rht -= 1;
        }
    }

    vol_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_element() {
        let heights = vec![4];
        assert_eq!(max_area(heights), 0);
    }

    #[test]
    fn test_two_elements() {
        let heights = vec![1, 5];
        assert_eq!(max_area(heights), 1);
    }

    #[test]
    fn test_all_same_height() {
        let heights = vec![5, 5, 5, 5];
        assert_eq!(max_area(heights), 15);
    }

    #[test]
    fn test_increasing_height() {
        let heights = vec![1, 2, 3, 4];
        assert_eq!(max_area(heights), 4);
    }

    #[test]
    fn test_decreasing_height() {
        let heights = vec![4, 3, 2, 1];
        assert_eq!(max_area(heights), 4);
    }

    #[test]
    fn test_mixed_heights() {
        let heights = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(max_area(heights), 49);
    }
}
