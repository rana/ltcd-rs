pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
    // Given an array of duplicates sorted ascending.
    // Remove duplicates in-place.
    // Allow one instance of any integer.
    // Return the number of unique integers.
    // Use a two-pointer technique.

    // Define a maximum number of unique integers.
    const MAX_UNQ: usize = 1;

    // Check for a minimum edge condition.
    if nums.len() < MAX_UNQ {
        return nums.len() as i32;
    }

    // Initialize a left pointer which tracks the unique element.
    let mut lft: usize = MAX_UNQ;

    // Loop through each element of nums.
    for rht in MAX_UNQ..nums.len() {
        // Check for unique condition.
        if nums[lft - MAX_UNQ] != nums[rht] {
            nums[lft] = nums[rht];
            lft += 1;
        }
    }

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
