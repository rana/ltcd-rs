pub fn rotate(nums: &mut [i32], k: i32) {
    // Modular rotation of the array.
    // Input: nums = [1,2,3,4,5,6,7], k = 3
    // Output: [5,6,7,1,2,3,4]
    // Use a three steps with a reverse function.
    fn rev(nums: &mut [i32], mut lft: usize, mut rht: usize) {
        while lft < rht {
            nums.swap(lft, rht);
            lft += 1;
            rht -= 1;
        }
    }

    // Initialize supporting variables.
    let k = k as usize % nums.len();
    let lst = nums.len() - 1;

    // Check for min edge condition for k and nums.
    if k == 0 || lst == 0 {
        return;
    }

    // Do the three reverses.
    rev(nums, 0, lst);
    rev(nums, 0, k - 1);
    rev(nums, k, lst)
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
