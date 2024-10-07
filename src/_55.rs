// Time Complexity: O(n), where n is the length of the array.
// Space Complexity: O(1), a constant amount of space is used.
// https://chatgpt.com/c/67030f39-6d14-8007-a45b-4ca06ae179ab
pub fn can_jump(nums: Vec<i32>) -> bool {
    // Given an integer array of jump lengths.
    // Start at the first index.
    // Each element in the array represents the jump length.
    // Return true is the last index can be reached.
    // Use a local optimization "greedy" algorithm.
    let mut max_reach: usize = 0;

    for n in 0..nums.len() {
        // Check for min edge condition.
        if n > max_reach {
            return false;
        }

        // Measure the max reach from the current position.
        max_reach = max_reach.max(n + nums[n] as usize);

        // Check for success condition.
        if max_reach >= nums.len() - 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_valid_2() {
        let nums = vec![0];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_invalid_1() {
        let nums = vec![3, 2, 1, 0, 4];
        assert!(!can_jump(nums));
    }

    #[test]
    fn test_valid_edge_case() {
        let nums = vec![2, 5, 0, 0];
        assert!(can_jump(nums));
    }

    #[test]
    fn test_invalid_edge_case() {
        let nums = vec![1, 0, 0, 0];
        assert!(!can_jump(nums));
    }
}
