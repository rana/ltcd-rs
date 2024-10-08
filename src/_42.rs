// Time Complexity: O(n), where n is the length of the array.
// Space Complexity: O(1), constant space used.
// https://chatgpt.com/c/670442c0-84c8-8002-b980-bbae64596ba6
pub fn trap(height: Vec<i32>) -> i32 {
    // Given array of positive integers.
    // Calculate trapped water from elevation map.
    // Use a local optimization "greedy" approach.
    // Use a two-pointer technique.

    // Check for a minimum edge condition.
    if height.is_empty() {
        return 0;
    }

    let mut lft = 0;
    let mut rht = height.len() - 1;
    let mut max_lft = 0;
    let mut max_rht = 0;
    let mut water_trapped = 0;

    while lft < rht {
        if height[lft] < height[rht] {
            if height[lft] > max_lft {
                // Store new max left elevation.
                max_lft = height[lft];
            } else {
                // Calculate trapped water at left position.
                water_trapped += max_lft - height[lft];
            }
            lft += 1;
        } else {
            if height[rht] > max_rht {
                // Store new max right elevation.
                max_rht = height[rht];
            } else {
                // Calculate trapped water at right position.
                water_trapped += max_rht - height[rht];
            }
            rht -= 1;
        }
    }

    water_trapped
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
