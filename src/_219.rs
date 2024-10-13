// Time complexity: O(n), n is the length of nums. We traverse nums once.
// Space complexity: O(min(n, k)), up to n numbers stored in a HashSet.
// https://chatgpt.com/c/67098e96-319c-8002-8a2d-b3b07c84a953
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    // Contains Duplicate II
    // Given an integer array `nums`.
    // Given an integer `k`.
    // Determine conditions:
    //  * nums[i] == nums[j]
    //  * abs(i - j) <= k
    // Return the two indexes.
    // Notice that the array is not sorted.
    // Use a sliding window strategy.
    // Use a set to track unique elements in the window.

    use std::collections::HashSet;

    // Use HashSet to determine success condition.
    let mut win_unq: HashSet<i32> = HashSet::new();
    // Cast k to usize for indexing.
    let k_usz = k as usize;

    // Iterate through each number.
    for (idx, num) in nums.iter().enumerate() {
        // Check for equal number in window.
        if win_unq.contains(num) {
            // Found success condition.
            return true;
        }

        // Insert new unique number into window.
        win_unq.insert(*num);

        // Remove number from left-side of window.
        if win_unq.len() > k_usz {
            let prv_num: i32 = nums[idx - k_usz];
            win_unq.remove(&prv_num);
        }
    }

    // No duplicate found in window.
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_within_k() {
        let wrds: Vec<i32> = vec![1, 2, 3, 1];
        let k: i32 = 3;
        assert!(contains_nearby_duplicate(wrds, k));
    }

    #[test]
    fn duplicate_at_k() {
        let wrds: Vec<i32> = vec![1, 0, 1, 1];
        let k: i32 = 1;
        assert!(contains_nearby_duplicate(wrds, k));
    }

    #[test]
    fn no_duplicate_within_k() {
        let wrds: Vec<i32> = vec![1, 2, 3, 4];
        let k: i32 = 2;
        assert!(!contains_nearby_duplicate(wrds, k));
    }
}
