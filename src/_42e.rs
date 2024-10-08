// Time Complexity: O(n), where n is the length of the height array.
// Space Complexity: O(1), constant space used.
pub fn trap(height: Vec<i32>) -> i32 {
    // Given an array of non-negative integers.
    // Array represents an elevation map.
    // Compute how much water it can trap after raining.
    // Use a local optimization "greedy" approach.
    // Use a two-pointer approach.

    // Check for input min edge condition.
    if height.len() <= 1 {
        return 0;
    }

    // Initialize variables.
    let mut trapped_water = 0;
    let mut lft_max = 0;
    let mut rht_max = 0;
    let mut lft = 0;
    let mut rht = height.len() - 1;

    // Loop through each element.
    while lft < rht {
        if height[lft] < height[rht] {
            // Check left max setting.
            if height[lft] > lft_max {
                lft_max = height[lft];
            } else {
                // Accumulate water on left.
                trapped_water += lft_max - height[lft];
            }
            lft += 1;
        } else {
            // Check right max setting.
            if height[rht] > rht_max {
                rht_max = height[rht];
            } else {
                // Accumulate water on right.
                trapped_water += rht_max - height[rht];
            }
            rht -= 1;
        }
    }

    trapped_water
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap_example1() {
        // Test case from the problem description
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(trap(height), 6);
    }

    #[test]
    fn test_trap_example2() {
        // Flat surface, no trapped water
        let height = vec![1, 1, 1, 1];
        assert_eq!(trap(height), 0);
    }

    #[test]
    fn test_trap_increasing() {
        // Increasing elevation, no trapped water
        let height = vec![0, 1, 2, 3, 4, 5];
        assert_eq!(trap(height), 0);
    }

    #[test]
    fn test_trap_decreasing() {
        // Decreasing elevation, no trapped water
        let height = vec![5, 4, 3, 2, 1, 0];
        assert_eq!(trap(height), 0);
    }

    #[test]
    fn test_trap_empty() {
        // Empty elevation map
        let height: Vec<i32> = vec![];
        assert_eq!(trap(height), 0);
    }

    #[test]
    fn test_trap_single_bar() {
        // Single elevation bar
        let height = vec![4];
        assert_eq!(trap(height), 0);
    }

    #[test]
    fn test_trap_invalid_negative_height() {
        // Invalid case with negative elevation
        let height = vec![2, -1, 2];
        assert_eq!(trap(height), 3);
    }
}
