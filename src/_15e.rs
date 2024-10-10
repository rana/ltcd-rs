pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 3Sum
    // Given an integer array of nums.
    // Duplicate integers may exist.
    // Array is unsorted.
    // Determine all sums equal to zero.
    // Return an array of index triplet solutions.
    // Use a two-pointer approach.

    use std::cmp::Ordering;

    // Sort nums array in ascending order.
    // Used to skip duplicates.
    // Time complexity: O(nlogn)
    nums.sort_unstable();

    // Initialize variables.
    let mut res: Vec<Vec<i32>> = Vec::new();

    // Loop through each number.
    // Pins first triplet.
    for idx in 0..nums.len() {
        // Progress through any duplicates.
        if idx > 0 && nums[idx] == nums[idx - 1] {
            continue;
        }

        // Initialize two pointers.
        let mut lft: usize = idx + 1;
        let mut rht: usize = nums.len() - 1;

        // Loop until two pointers meet.
        while lft < rht {
            // Calculate sum of triplets.
            let sum = nums[idx] + nums[lft] + nums[rht];

            // Compare sum to target.
            const TARGET: i32 = 0;
            match sum.cmp(&TARGET) {
                // Success condition found.
                Ordering::Equal => {
                    // Store triplet.
                    res.push(vec![nums[idx], nums[lft], nums[rht]]);

                    // Progress lft to last duplicate.
                    while lft < rht && nums[lft] == nums[lft + 1] {
                        lft += 1;
                    }

                    // Progress rht to last duplicate.
                    while lft < rht && nums[rht] == nums[rht - 1] {
                        rht -= 1;
                    }

                    // Progress two pointers.
                    lft += 1;
                    rht -= 1;
                }
                // Make next sum larger.
                Ordering::Less => lft += 1,
                // Make next sum smaller.
                Ordering::Greater => rht -= 1,
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_and_positive_numbers() {
        let mut result = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        result.sort();
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn valid_triplets() {
        let mut result = three_sum(vec![-2, -1, -1, 0, 1, 2]);
        result.sort();
        let mut expected = vec![vec![-2, 0, 2], vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn no_triplets() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn all_zeros() {
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(three_sum(Vec::new()), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn single_element() {
        assert_eq!(three_sum(vec![1]), Vec::<Vec<i32>>::new());
    }
}
