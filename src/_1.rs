/// 1. Two Sum
///
/// Given an array of integers nums and an integer target,
/// return indices of the two numbers such that they add up
/// to target.
///
/// You may assume that each input would have exactly one
/// solution, and you may not use the same element twice.
///
/// You can return the answer in any order.
///
/// Constraints:
/// * 2 <= nums.length <= 104
/// * -10^9 <= nums[i] <= 10^9
/// * -10^9 <= target <= 10^9
/// * Only one valid answer exists.

fn two_sum(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
    use std::collections::HashMap;

    // Map each number to it's index.
    // Contributes to O(n) space complexity.
    let mut map = HashMap::new();

    // Loop through each `nums` element.
    // Contributes to O(n) time complexity.
    for (idx_cur, &num) in nums.iter().enumerate() {
        // Check whether the complement was seen.
        let complement = tgt - num;
        if let Some(&idx_map) = map.get(&complement) {
            return vec![idx_cur as i32, idx_map as i32];
        }
        // Store the current number to index mapping.
        map.insert(num, idx_cur);
    }

    vec![]
}

fn two_sum_a(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for (index, &num) in nums.iter().enumerate() {
        if let Some(&i) = map.get(&(target - num)) {
            return vec![i as i32, index as i32];
        }
        map.insert(num, index);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_common() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn test_two_sum_no_result() {
        assert_eq!(two_sum(vec![1, 2, 3], 7), vec![]);
    }

    #[test]
    fn test_two_sum_single_element() {
        assert_eq!(two_sum(vec![1], 1), vec![]);
    }

    #[test]
    fn test_two_sum_negative_numbers() {
        assert_eq!(two_sum(vec![-1, -2, -3, -4], -6), vec![1, 3]);
    }

    #[test]
    fn test_two_sum_zero_target() {
        assert_eq!(two_sum(vec![0, 4, 3, 0], 0), vec![0, 3]);
    }

    #[test]
    fn test_two_sum_large_numbers() {
        assert_eq!(two_sum(vec![100000000, 50000000], 150000000), vec![0, 1]);
    }
}
