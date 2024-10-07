pub fn can_jump(nums: Vec<i32>) -> bool {
    // You are given an integer array.
    // You are positioned at the first index.
    // Each element represents a number of jumps
    // from that index.
    // Determine if you can reach the last index.
    // Use a local optimization "greedy" approach.

    // Check for minimum edge condition.
    if nums.len() == 1 {
        return true;
    }

    let idx_lst = nums.len() - 1;
    let mut farthest: usize = 0;

    // Loop through each element.
    // No need to evaluate the last element.
    for n in 0..idx_lst {
        // Check for minimum edge condition.
        if n > farthest {
            return false;
        }

        farthest = farthest.max(n + nums[n] as usize);

        // Check for success condition.
        if farthest >= idx_lst {
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
