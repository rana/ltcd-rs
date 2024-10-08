
// Time Complexity: O(n), where two iteration over array.
// Space Complexity: O(1), constant supporting variables used.
// https://chatgpt.com/c/67042de0-c94c-8002-9592-eee40b070b63
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // Products of array except self.
    // No division use.
    // Use a two-pass approach.
    // Left pass: calculate products of all value to the left.
    // Right pass: calculate products of all value to the right.

    let len = nums.len();
    let mut answer = vec![1; len];

    // Left pass: all values to the left of current value.
    let mut left_product = 1;
    for n in 0..len {
        answer[n] = left_product;
        left_product *= nums[n];
    }

    // Right pass: all values to the right of current value,
    //  multiplied by left pass product.
    let mut right_product = 1;
    for n in (0..len).rev() {
        answer[n] *= right_product;
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
