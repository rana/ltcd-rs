/// 128. Longest Consecutive Sequence
///
/// Given an unsorted array of integers nums,
/// return the length of the longest consecutive elements sequence.
///
/// You must write an algorithm that runs in O(n) time.
///
/// Constraints:
/// * 0 <= nums.length <= 10^5
/// -10^9 <= nums[i] <= 10^9

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    // Store each number in a HashSet.
    // Search for sequence starts by n-1 not in the HashSet.
    // Measure each sequence length.
    // Store the maximum length of a sequence.
    // Time complexity: O(n).
    //  - n is the length of the nums array.
    //  - Loop through nums twice: once to populate the HashSet,
    //      once to search for sequences.
    // Space complexity: O(n).
    //  - n is the length of the nums array.
    //  - Store each number in a HashSet.

    use std::collections::HashSet;

    // Create a HashSet with each number.
    // Contributes to O(n) space complexity.
    // Contributes to O(n) time complexity.
    let set: HashSet<i32> = nums.iter().cloned().collect();

    let mut max_len: i32 = 0;

    // Loop through each number.
    // Contributes to O(n) time complexity.
    for &num in &set {
        // Search for sequence start.
        // Start of a sequence is when n-1 isn't in the set.
        if !set.contains(&(num - 1)) {
            // Found the start of a possible sequence.
            let mut cur_len: i32 = 1;
            let mut cur_num: i32 = num;

            // Measure the length of the sequence.
            while set.contains(&(cur_num + 1)) {
                cur_num += 1;
                cur_len += 1;
            }

            // Store the maximum sequence length.
            max_len = max_len.max(cur_len);
        }
    }

    max_len
}

fn longest_consecutive_b(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    // Create a set of all elements in `nums`.
    // Contributes to O(n) space complexity.
    let num_set: HashSet<i32> = nums.iter().cloned().collect();
    let mut max_len: i32 = 0;

    // Loop through each element in `nums`.
    // Contributes to O(n) time complexity.
    for &num in &num_set {
        // Check for sequence start.
        // Start indicated by one before current number not in set.
        if !num_set.contains(&(num - 1)) {
            let mut cur_len = 1;
            let mut cur_num = num;

            // Count the sequence length.
            while num_set.contains(&(cur_num + 1)) {
                cur_num += 1;
                cur_len += 1;
            }

            // Check whether the current sequence is the longest.
            max_len = max_len.max(cur_len);
        }
    }

    max_len
}

fn longest_consecutive_a(nums: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let num_set: HashSet<i32> = nums.iter().cloned().collect();
    let mut max_length = 0;

    for &num in &num_set {
        // Only start counting if `num - 1` is not in the set (ensures starting at the beginning of a sequence)
        if !num_set.contains(&(num - 1)) {
            let mut current_length = 1;
            let mut current_num = num;

            // Increment the sequence
            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_length += 1;
            }

            max_length = max_length.max(current_length);
        }
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(longest_consecutive(vec![]), 0);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(longest_consecutive(vec![1, 1, 1, 1]), 1);
    }

    #[test]
    fn test_consecutive_start() {
        assert_eq!(longest_consecutive(vec![1, 2, 3, 10, 11, 15]), 3);
    }

    #[test]
    fn test_consecutive_end() {
        assert_eq!(longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(longest_consecutive(vec![-1, -2, -3, 1]), 3);
    }
}
