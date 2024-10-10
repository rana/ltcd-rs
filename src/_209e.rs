// Time complexity: O(n), n is the length of the nums array. We traverse the array once.
// Space complexity: O(1), constant additional space used.
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    // Minimum size subarray sum
    // Given an array of positive integers.
    // Given a target positive integer.
    // Determine the min window length
    // of a subarray whose sum >= target.
    // Return the min window length; or, zero.
    // Use a sliding window two pointer technique.

    let mut min_len: Option<usize> = None;
    let mut sum: i32 = 0;
    let mut lft: usize = 0;

    // Loop through each element.
    for rht in 0..nums.len() {
        // Calculate sum of current window.
        sum += nums[rht];

        // Check whether window condition met.
        while sum >= target {
            // Calculate the current window length.
            let cur_len = rht - lft + 1;

            // Store min window length if current is shorter.
            min_len = match min_len {
                Some(len) => Some(len.min(cur_len)),
                None => Some(cur_len),
            };

            // Shrink window on left.
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
