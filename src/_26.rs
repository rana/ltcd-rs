pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    // Remove duplicate elements from a sorted array.
    // Return the number of unique sorted elements.
    // Given a sorted array with duplicates.
    // Use a two-pointer technique.

    // Move unique elements to the left.
    // Remove duplicate elements from the front of nums.
    // The array is sorted.

    // `MAX_DUP` is the maximum number of duplicates.
    // `MAX_DUP = 1` means no duplicates.
    const MAX_DUP: usize = 1;

    // Check for a `nums` minum edge condition.
    if nums.len() <= MAX_DUP {
        return nums.len() as i32;
    }

    // Initialize a lft pointer.
    // lft is the slow moving pointer.
    // This tracks the unique elements.
    let mut lft: usize = MAX_DUP;

    // Loop until the rht pointer is complete.
    // rht is the fast moving pointer.
    for rht in MAX_DUP..nums.len() {
        // Search for a right element that can move left.
        // May be multiple duplicate to slide over.
        // `lft - MAX_DUP` points to a unique element.
        if nums[lft - MAX_DUP] != nums[rht] {
            // Found a right element which isn't equal
            // to the left element.
            nums[lft] = nums[rht];
            lft += 1;
        }
    }

    // lft is the processed array length.
    lft as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let mut nums = vec![];
        assert_eq!(remove_duplicates(&mut nums), 0);
        assert_eq!(nums, vec![]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_two_same_elements() {
        let mut nums = vec![1, 1];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(nums[..1], [1]); // Check that the first part of the array is correct
    }

    #[test]
    fn test_two_different_elements() {
        let mut nums = vec![1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2);
        assert_eq!(nums[..2], [1, 2]);
    }

    #[test]
    fn test_multiple_duplicates() {
        let mut nums = vec![1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 4);
        assert_eq!(nums[..4], [1, 2, 3, 4]);
    }

    #[test]
    fn test_all_same_elements() {
        let mut nums = vec![5, 5, 5, 5];
        assert_eq!(remove_duplicates(&mut nums), 1);
        assert_eq!(nums[..1], [5]);
    }

    #[test]
    fn test_large_input() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
        assert_eq!(nums[..5], [0, 1, 2, 3, 4]);
    }
}
