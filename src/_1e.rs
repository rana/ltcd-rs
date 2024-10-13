pub fn two_sum(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
    // Given an array of integers.
    // Given a target integer.
    // Determine two numbers which equal target.
    // Return the indexes of the two numbers.
    // One solution exists.
    // Notice that the array is not sorted.
    // Use a HashMap of integer to index.

    use std::collections::HashMap;

    // Initialize an integer-to-index map.
    let mut map: HashMap<i32, usize> = HashMap::new();

    // Iterate through each number.
    for (idx, &num) in nums.iter().enumerate() {
        // Calculate complement for map lookup key.
        let complement = tgt - num;
        // Lookup index of complement.
        if let Some(&comp_idx) = map.get(&complement) {
            // Solution found.
            // Return indexes.
            return vec![comp_idx as i32, idx as i32];
        }

        // Insert number-index mapping.
        map.insert(num, idx);
    }

    // No solution.
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
