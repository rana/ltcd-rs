pub fn remove_duplicates2(nums: &mut [i32]) -> i32 {
    // Remove duplicates from a sorted array.
    // Allow two duplicates.
    // Leave relative order in-place.
    // Given a sorted array.
    // Return the length of the processed array.
    // Use a two-pointer technique.

    // Move unique element to the left.
    // Remove duplicate element from the front on nums array.

    const MAX_DUP: usize = 2;

    // Check for a `nums` minimum edge condition.
    if nums.len() <= MAX_DUP {
        return nums.len() as i32;
    }

    // Initialize lft pointer.
    // lft is the slow moving pointer.
    // This tracks to the unique elements.
    let mut lft: usize = MAX_DUP;

    // Loop until the rht pointer is complete.
    // rht is the fast moving pointer.
    for rht in MAX_DUP..nums.len() {
        // Search for a right element that can move left.
        // Allow two duplicates.
        // `lft - MAX_DUP` points to a unique element.
        if nums[lft - MAX_DUP] != nums[rht] {
            // Found a right element which isn't equal to the left element.
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

    // Edge Case: Empty array
    #[test]
    fn test_empty_array() {
        let mut nums = vec![];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 0);
        assert_eq!(nums, vec![]);
    }

    // Edge Case: Array with one element
    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 1);
        assert_eq!(nums[..len as usize], [1]);
    }

    // Edge Case: Array with two elements
    #[test]
    fn test_two_elements() {
        let mut nums = vec![1, 1];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(nums[..len as usize], [1, 1]);
    }

    // Common Case: Array with duplicates allowed twice
    #[test]
    fn test_allowed_duplicates() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(nums[..len as usize], [1, 1, 2, 2, 3]);
    }

    // Common Case: Array with no duplicates
    #[test]
    fn test_no_duplicates() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 5);
        assert_eq!(nums[..len as usize], [1, 2, 3, 4, 5]);
    }

    // Common Case: Array with all elements the same
    #[test]
    fn test_all_same() {
        let mut nums = vec![1, 1, 1, 1, 1, 1];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 2);
        assert_eq!(nums[..len as usize], [1, 1]);
    }

    // Edge Case: Larger array with multiple duplicate groups
    #[test]
    fn test_large_array() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let len = remove_duplicates2(&mut nums);
        assert_eq!(len, 9);
        assert_eq!(nums[..len as usize], [0, 0, 1, 1, 2, 2, 3, 3, 4]);
    }
}
