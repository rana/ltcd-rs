pub fn jump2(nums: Vec<i32>) -> i32 {
    // Given an integer array of jumps.
    // Start at the first index.
    // End at the last index.
    // Determine the minmum number of jumps.
    // There is a solution to the last index.
    // Use a local optimization "greedy" algorithm.

    // Check for minimum edge case.
    if nums.len() <= 1 {
        return 0;
    }

    // Initialize variables.
    let mut jmp_cnt: i32 = 0;
    let mut cur_end: i32 = 0;
    let mut farthest: i32 = 0;
    let idx_lst = nums.len() - 1;

    for idx in 0..idx_lst {
        // Search for farthest jump.
        farthest = farthest.max(idx as i32 + nums[idx]);

        // Check for segment update.
        if idx as i32 == cur_end {
            jmp_cnt += 1;
            cur_end = farthest;

            // Check for success condition.
            if cur_end >= idx_lst as i32 {
                break;
            }
        }
    }

    jmp_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_case_1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(jump2(nums), 2); // Expect 2 jumps
    }

    #[test]
    fn test_valid_case_2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(jump2(nums), 2); // Expect 2 jumps
    }

    #[test]
    fn test_valid_case_3() {
        let nums = vec![0];
        assert_eq!(jump2(nums), 0); // No jumps needed when there's only 1 element
    }

    #[test]
    fn test_valid_case_4() {
        let nums = vec![1, 2, 1, 1, 1];
        assert_eq!(jump2(nums), 3); // 3 jumps are needed
    }

    #[test]
    fn test_valid_case_5() {
        let nums = vec![10, 9, 8, 1, 0, 1, 1];
        assert_eq!(jump2(nums), 1); // The first number is large enough to jump to the end
    }

    #[test]
    fn test_invalid_case_1() {
        let nums = vec![];
        assert_eq!(jump2(nums), 0); // Edge case: empty array, no jumps needed
    }
}
