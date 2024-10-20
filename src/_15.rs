// Time complexity: O(n^2), n is the length of the nums array. Nums is traversed twice.
// Space complexity: O(n), the average case of triplets dependent on input
// https://chatgpt.com/c/6705c47b-79c4-8002-af71-9e73c411f45a
pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;

    // Given an integer array nums.
    // Notice nums is not sorted.
    // Notice duplicates may exist.
    // Return all triplets which sum to zero.
    // Triplet indexes are to be different.
    // Sort and use a two-pointer technique.

    // Sort to enable two-pointer approach.
    nums.sort_unstable();

    // Initialize variables.
    let mut res = Vec::new();

    for n in 0..nums.len() {
        // Skip duplicates for the first number.
        if n > 0 && nums[n] == nums[n - 1] {
            continue;
        }

        // Initialize two-pointer variables.
        let mut lft = n + 1;
        let mut rht = nums.len() - 1;

        // Loop until the two pointers meet.
        while lft < rht {
            // Calculate the current triplet sum.
            let cur_sum = nums[n] + nums[lft] + nums[rht];

            // Evaluate current sum.
            match cur_sum.cmp(&0) {
                // Current sum matches target value of zero.
                Ordering::Equal => {
                    // Store the matching triplet.
                    res.push(vec![nums[n], nums[lft], nums[rht]]);

                    // Progress lft pointer to last duplicate.
                    while lft < rht && nums[lft] == nums[lft + 1] {
                        lft += 1;
                    }
                    // Progress rht pointer to last duplicate.
                    while lft < rht && nums[rht] == nums[rht - 1] {
                        rht -= 1;
                    }

                    // Progress two pointers to next unique.
                    lft += 1;
                    rht -= 1;
                }
                // Make the next sum larger.
                Ordering::Less => lft += 1,
                // Make the nex sum smaller.
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
