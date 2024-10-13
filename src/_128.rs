/// Time complexity: O(n), where *n* is the length of the array `nums`. We iterate through `nums` twice: first to create a HashSet, and second to search for sequences.
/// Space complexity: O(n), we create a HashSet with all elements of `nums`.
/// https://chatgpt.com/c/670992c3-3128-8002-aeb8-066e9cf5ed45
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // Longest Consecutive Sequence
    // Given an unsorted array of integers `nums`.
    // Determine the max length of a consecutive sequence of integers.
    // Solve with time complexity O(n).
    // Notice that the array is unsorted.
    // Use a set to store integers. Enables solution in O(n) time.
    // Search for sequence starts.

    use std::collections::HashSet;

    let mut max_len: i32 = 0;
    let set: HashSet<i32> = nums.iter().cloned().collect();

    // Iterate through each element in nums.
    // Iterate through set avoids iteration over duplicates.
    for &num in &set {
        // Check for the start of a sequence.
        // If the prior number isn't in the set we have a starting point.
        if !set.contains(&(num - 1)) {
            let mut cur_num: i32 = num;
            let mut cur_len: i32 = 1;

            // Check for consecutive integers.
            while set.contains(&(cur_num + 1)) {
                cur_num += 1;
                cur_len += 1;
            }

            // Evaluate maximum length.
            if cur_len > max_len {
                max_len = cur_len;
            }
        }
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let nums: Vec<i32> = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn example_two() {
        let nums: Vec<i32> = vec![0, -1];
        assert_eq!(longest_consecutive(nums), 2);
    }

    #[test]
    fn empty_input() {
        let nums: Vec<i32> = vec![];
        assert_eq!(longest_consecutive(nums), 0);
    }

    #[test]
    fn single_element() {
        let nums: Vec<i32> = vec![10];
        assert_eq!(longest_consecutive(nums), 1);
    }

    #[test]
    fn no_consecutive() {
        let nums: Vec<i32> = vec![10, 30, 20];
        assert_eq!(longest_consecutive(nums), 1);
    }

    #[test]
    fn all_consecutive() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        assert_eq!(longest_consecutive(nums), 5);
    }

    #[test]
    fn negative_numbers() {
        let nums: Vec<i32> = vec![-2, -3, -1, -5, -4];
        assert_eq!(longest_consecutive(nums), 5);
    }
}
