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
/// * 2 <= nums.length <= 10^4
/// * -10^9 <= nums[i] <= 10^9
/// * -10^9 <= target <= 10^9
/// * Only one valid answer exists.

fn two_sum(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
    // Create a HashMap of num-to-index.
    // Time complexity: O(n).
    //  - n is the length of the nums array.
    //  - Looks at each number once.
    // Space complexity: O(n).
    //  - n is the length of the nums array.
    //  - Stores each num-to-index pair.
    use std::collections::HashMap;

    // Contributes O(n) space complexity.
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Loop through each element in the nums array.
    // Contributes O(n) time complexity.
    for (idx_num, num) in nums.into_iter().enumerate() {
        // Calculate the complement.
        let complement = tgt - num;
        // Search for the complement in the HashMap.
        if let Some(&idx_map) = map.get(&complement) {
            // Found solution.
            return vec![idx_map as i32, idx_num as i32];
        }
        // Insert current number-index mapping.
        map.insert(num, idx_num);
    }

    vec![]
}

fn two_sum_c(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
    // Map a number to and index with a HashMap.
    // Time complexity: O(n).
    //  - n is the length of the nums array.
    // Space complexity: O(n).
    //  - n is the length of the nums array.
    //  - Up to n-1 numbers are inserted into the HashMap.
    use std::collections::HashMap;

    // Contributes to O(n) space complexity.
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Loop through each element of array nums.
    // Contributes to O(n) time complexity.
    for (idx_num, num) in nums.into_iter().enumerate() {
        // Calculate the complement of the current number.
        let complement = tgt - num;
        // Search for the complement in the HashMap.
        if let Some(&idx_map) = map.get(&complement) {
            // Found solution.
            return vec![idx_map as i32, idx_num as i32];
        }

        // Store current number to index mapping.
        map.insert(num, idx_num);
    }

    vec![]
}

fn two_sum_b(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
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
