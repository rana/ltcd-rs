/// 209. Minimum Size Subarray Sum
/// 
/// Given an array of positive integers nums and a positive 
/// integer target, return the minimal length of a subarray 
/// whose sum is greater than or equal to target. If there 
/// is no such subarray, return 0 instead.
/// 
/// Constraints:
/// * 1 <= target <= 10^9
/// * 1 <= nums.length <= 10^5
/// * 1 <= nums[i] <= 10^4

fn min_sub_array_len(tgt: i32, nums: Vec<i32>) -> i32 {
    // Variables contribute to the O(1) space complexity.
    let mut lft: usize = 0;
    let mut min_len = usize::MAX;
    let mut sum: i32 = 0;

    // Loop contributes to the O(n) time complexity.
    for (rht, &val) in nums.iter().enumerate() {
        // Accumulate the sum.
        sum += val;

        // Check whether the target is reached.
        while sum >= tgt {
            // Store the smallest window length.
            min_len = min_len.min(rht - lft + 1);

            // Shrink the window from the left side.
            sum -= nums[lft];
            lft += 1;
        }
    }

    // Return the minimum window.
    // Check whether we found any window.
    if min_len == usize::MAX {
        0
    } else {
        min_len as i32
    }
}

fn min_sub_array_len_b(target: i32, nums: Vec<i32>) -> i32 {
    let (mut left, mut sum, mut result) = (0, 0, usize::MAX);
    for (right, &value) in nums.iter().enumerate() {
        sum += value;
        while sum >= target {
            result = result.min(right - left + 1);
            sum -= nums[left];
            left += 1;
        }
    }
    if result == usize::MAX {
        0
    } else {
        result as i32
    }
}

fn min_sub_array_len_a(tgt: i32, nums: Vec<i32>) -> i32 {
    let mut min_arr_len: usize = usize::MAX;

    let mut sum: i32 = 0;
    let mut lft: usize = 0;

    for rht in 0..nums.len() {
        sum += nums[rht];
        while sum >= tgt {
            let arr_len = rht - lft + 1;
            min_arr_len = min_arr_len.min(arr_len);
            sum -= nums[lft];
            lft += 1;
        }
    }

    if min_arr_len == usize::MAX {
        0
    } else {
        min_arr_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let nums = vec![];
        let target = 7;
        assert_eq!(min_sub_array_len(target, nums), 0);
    }

    #[test]
    fn test_no_valid_subarray() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 6;
        assert_eq!(min_sub_array_len(target, nums), 0);
    }

    #[test]
    fn test_single_element_valid() {
        let nums = vec![10];
        let target = 10;
        assert_eq!(min_sub_array_len(target, nums), 1);
    }

    #[test]
    fn test_single_element_invalid() {
        let nums = vec![5];
        let target = 10;
        assert_eq!(min_sub_array_len(target, nums), 0);
    }

    #[test]
    fn test_all_elements_valid() {
        let nums = vec![2, 3, 1, 2, 4, 3];
        let target = 7;
        assert_eq!(min_sub_array_len(target, nums), 2);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![100, 200, 300, 400];
        let target = 500;
        assert_eq!(min_sub_array_len(target, nums), 2);
    }
}
