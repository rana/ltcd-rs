pub fn rotate(nums: &mut [i32], k: i32) {
    // Given integer array, rotate to the right
    // by `k` steps.
    // Use an inner rotate function with 3 calls.
    fn rot(mut lft: usize, mut rht: usize, nums: &mut [i32]) {
        while lft < rht {
            nums.swap(lft, rht);
            lft += 1;
            rht -= 1;
        }
    }

    // Initialize variables.
    let k = k as usize % nums.len();
    let lst = nums.len() - 1;

    rot(0, lst, nums);
    rot(0, k - 1, nums);
    rot(k, lst, nums);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_array() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        rotate(&mut nums, 3);
        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);

        let mut nums = vec![0, 1, 2];
        rotate(&mut nums, 4);
        assert_eq!(nums, vec![2, 0, 1]);

        let mut nums = vec![1, 2];
        rotate(&mut nums, 1);
        assert_eq!(nums, vec![2, 1]);
    }
}
