// Time complexity: O(n), n is the length of the nums array. We traverse the array once.
// Space complexity: O(1), constant additional space is used.
// https://chatgpt.com/c/6706cb78-8bf8-8002-8c97-cb0b6f9665c3
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // Given an array of positive integers.
    // Given a positive target integer.
    // Determine a subarray sum >= target.
    // Determine the minimum subarray length.
    // Return minimum subarray length; or, zero.
    // Use a sliding window, two-pointer technique.
    // Expand the window to the right to meet the criteria.
    // Attempt to shrink the window from the left to optimize criteria.

    let mut lft: usize = 0;
    let mut sum: i32 = 0;
    let mut min_len: Option<usize> = None;

    // Loop through each element.
    for rht in 0..nums.len() {
        // Calculate current sum.
        sum += nums[rht];

        // Expand the window to the right.
        while sum >= target {
            let cur_len = rht - lft + 1;

            // Update min length if current window is smaller.
            min_len = match min_len {
                Some(len) => Some(len.min(cur_len)),
                None => Some(cur_len),
            };

            // Shrink window from left.
            sum -= nums[lft];
            lft += 1;
        }
    }

    match min_len {
        Some(len) => len as i32,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_case_small_array() {
        let target = 7;
        let nums = vec![2, 3, 1, 2, 4, 3];
        assert_eq!(min_sub_array_len(target, nums), 2);
    }

    #[test]
    fn valid_case_entire_array() {
        let target = 15;
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(min_sub_array_len(target, nums), 5);
    }

    #[test]
    fn valid_case_no_subarray() {
        let target = 100;
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(min_sub_array_len(target, nums), 0);
    }

    #[test]
    fn invalid_case_empty_array() {
        let target = 5;
        let nums: Vec<i32> = vec![];
        assert_eq!(min_sub_array_len(target, nums), 0);
    }

    #[test]
    fn valid_case_single_element() {
        let target = 4;
        let nums = vec![4];
        assert_eq!(min_sub_array_len(target, nums), 1);
    }
}
