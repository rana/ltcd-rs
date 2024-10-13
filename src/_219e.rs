pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    // Contains Duplicate II
    // Given an integer array `nums`.
    // Given an integer `k`.
    // Determine condition:
    //  * nums[i] == nums[j]
    //  * abs(i - j) <= k
    // Return true if indexes i and j exist.
    // Notice that the array is not sorted.
    // Use a sliding window technique.
    // Use a set to store unique integers for the window.

    use std::collections::HashSet;

    // Store unique integers for the window.
    // Use to detect duplicate in the window.
    let mut win_unq: HashSet<i32> = HashSet::new();
    // Cast k to usize for index use.
    let k_usz = k as usize;

    // Iterate through each integer in nums array.
    for (idx, num) in nums.iter().enumerate() {
        // Check whether duplicate exists in window.
        if win_unq.contains(num) {
            // Found a duplicate in the window.
            return true;
        }

        // Store the current integer ni the set for the window.
        win_unq.insert(*num);

        // Check whether to shrink the window set.
        if win_unq.len() > k_usz {
            // Shrink the left-side of the window.
            let prv_num: i32 = nums[idx - k_usz];
            // Remove the left-side number.
            win_unq.remove(&prv_num);
        }
    }

    // No duplicate found.
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
