// Time Complexity: O(n), where n is the length of the nums array.
// Space Complexity: O(1), constant space used.
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // Given an integer array of nums.
    // Return an array where each element is the product
    // of all elements except self.
    // Run in O(n) without division.
    // Use a two-pass approach.
    // Left pass calculates products left.
    // Right pass calculates products right, also adding to answer.

    // Initialize variables.
    let len = nums.len();
    let mut answer = vec![1; len];

    // Left-pass products.
    let mut left_product = 1;
    for n in 0..len {
        // Assign current left product.
        answer[n] = left_product;

        // Calculate new left product.
        left_product *= nums[n];
    }

    // Right-pass products.
    let mut right_product = 1;
    for n in (0..len).rev() {
        // Assign and combine right product.
        answer[n] *= right_product;

        // Calculate new right product.
        right_product *= nums[n];
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        let nums = vec![1, 2, 3, 4];
        let result = product_except_self(nums);
        assert_eq!(result, vec![24, 12, 8, 6]);
    }

    #[test]
    fn test_with_zero() {
        let nums = vec![0, 4, 5];
        let result = product_except_self(nums);
        assert_eq!(result, vec![20, 0, 0]);
    }

    #[test]
    fn test_multiple_zeros() {
        let nums = vec![0, 4, 0];
        let result = product_except_self(nums);
        assert_eq!(result, vec![0, 0, 0]);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![10];
        let result = product_except_self(nums);
        assert_eq!(result, vec![1]);
    }

    #[test]
    fn test_empty_array() {
        let nums: Vec<i32> = vec![];
        let result = product_except_self(nums);
        assert_eq!(result, vec![]);
    }
}
