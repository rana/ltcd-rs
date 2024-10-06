pub fn remove_duplicates2(nums: &mut [i32]) -> i32 {
    // Given an array sorted ascending.
    // Remove some duplicates. Allow two duplicates/
    // Remove in-place.
    // Place valid elements in front.
    // Return number of valid elements.
    // Use a two-pointer technique.

    // Define the number of valid unique elements.
    const MAX_UNQ: usize = 2;

    // Check for edge condition.
    if nums.len() < MAX_UNQ {
        return nums.len() as i32;
    }

    // Define left, slow pointer which tracks
    // the location of the valid element.
    let mut lft: usize = MAX_UNQ;

    // Loop through each element.
    for rht in MAX_UNQ..nums.len() {
        // Check valid condition.
        if nums[lft - MAX_UNQ] != nums[rht] {
            nums[lft] = nums[rht];
            lft += 1;
        }
    }

    // Return the number of valid elements.
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
