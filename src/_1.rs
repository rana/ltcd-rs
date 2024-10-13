// Time complexity: O(n), n is the length of array nums. We traverse the array once.
// Space complexity: O(n), we store up to n integers in a HashMap.
// https://chatgpt.com/c/67096f32-3c38-8002-aa1e-f193ef92a391
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Given an array of integers.
    // Given a target integer.
    // Determine two numbers which equal target.
    // Expect exactly one solution.
    // Return the indexes of the two integers.
    // Notice that the array is not sorted.

    use std::collections::HashMap;

    // Initialize a map for the number-index mapping.
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Iterate through each integer.
    for (idx, &num) in nums.iter().enumerate() {
        // Calculate a complement. Use for reaching target.
        let complement: i32 = target - num;

        // Check if the complement is already mapped.
        if let Some(&comp_idx) = map.get(&complement) {
            // Solution found.
            return vec![comp_idx as i32, idx as i32];
        }

        // Map number-index.
        map.insert(num, idx);
    }

    // No solution
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_scenario() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let result: Vec<i32> = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn negative_numbers() {
        let nums: Vec<i32> = vec![-3, 4, 3, 90];
        let target: i32 = 0;
        let result: Vec<i32> = two_sum(nums, target);
        assert_eq!(result, vec![0, 2]);
    }

    #[test]
    fn multiple_pairs() {
        let nums: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;
        let result: Vec<i32> = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn single_element() {
        let nums: Vec<i32> = vec![1];
        let target: i32 = 2;
        let result: Vec<i32> = two_sum(nums, target);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn no_solution() {
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        let target: i32 = 8;
        let result: Vec<i32> = two_sum(nums, target);
        assert_eq!(result, vec![]);
    }
}
