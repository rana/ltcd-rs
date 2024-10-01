/// 15. 3Sum
///
/// Given an integer array nums, return all the triplets
/// [nums[i], nums[j], nums[k]] such that i != j, i != k,
/// and j != k, and nums[i] + nums[j] + nums[k] == 0.
///
/// Notice that the solution set must not contain
/// duplicate triplets.
///
/// Constraints:
/// * 3 <= nums.length <= 3000
/// * -10^5 <= nums[i] <= 10^5

fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;

    let mut ret: Vec<Vec<i32>> = Vec::new();
    let idx_lst = nums.len() - 1;

    // Sort nums to search for duplicates and use the two-pointer technique.
    // Use `sort_unstable` which is faster than stable sorting.
    // Sorting contributes O(nlogn) time complexity.
    nums.sort_unstable();

    // Outer loop.
    // Loop contributes to O(n^2) time complexity.
    for idx in 0..nums.len() - 2 {
        // Check for minimum value edge condition.
        if nums[idx] > 0 {
            break;
        }

        // Check to skip any duplicate.
        if idx > 0 && nums[idx - 1] == nums[idx] {
            continue;
        }

        let mut lft: usize = idx + 1;
        let mut rht: usize = idx_lst;

        // Search for three values equal to zero.
        // Loop contributes to O(n^2) time complexity.
        while lft < rht {
            match (nums[idx] + nums[lft] + nums[rht]).cmp(&0) {
                Ordering::Equal => {
                    // Found a solution.
                    ret.push(vec![nums[idx], nums[lft], nums[rht]]);

                    // Look for more solutions.

                    // Skip over left duplicate values.
                    while lft < rht && nums[lft] == nums[lft + 1] {
                        lft += 1;
                    }

                    // Skip over right duplicate values.
                    while lft < rht && nums[rht] == nums[rht - 1] {
                        rht -= 1;
                    }

                    // Advance the left and right pointers.
                    lft += 1;
                    rht -= 1;
                }
                Ordering::Less => {
                    // Sum is less than zero.
                    // Make the next value larger.
                    lft += 1;
                },
                Ordering::Greater => {
                    // Sum is greater than zero.
                    // Make the next value smaller.
                    rht -= 1;
                },
            }
        }
    }

    ret
}

fn three_sum_b(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.sort_unstable();
    let mut result = Vec::new();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }
                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

fn three_sum_a(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    use std::cmp::Ordering;

    let mut ret: Vec<Vec<i32>> = Vec::new();

    // Rely on sorted values for two pointer logic.
    nums.sort_unstable();

    let idx_lst = nums.len() - 1;

    // Index goes to the third to last.
    for idx in 0..nums.len() - 2 {
        // Check whether solution is no longer possible.
        // If smallest value is great than zero,
        // summing three value would always be greater than zero.
        if nums[idx] > 0 {
            break;
        }

        // Check whether to skip a duplicate value.
        if idx > 0 && nums[idx - 1] == nums[idx] {
            continue;
        }

        // Search for a solution.
        let mut lft = idx + 1;
        let mut rht = idx_lst;
        while lft < rht {
            match (nums[idx] + nums[lft] + nums[rht]).cmp(&0) {
                Ordering::Less => lft += 1,
                Ordering::Greater => rht -= 1,
                Ordering::Equal => {
                    // Store a solution.
                    ret.push(vec![nums[idx], nums[lft], nums[rht]]);

                    // Continue to look for more solutions.

                    // Skip over left duplicate values.
                    while lft < rht && nums[lft] == nums[lft + 1] {
                        lft += 1;
                    }

                    // Skip over right duplicate values.
                    while lft < rht && nums[rht] == nums[rht - 1] {
                        rht -= 1;
                    }

                    // Advance the left and right pointers.
                    lft += 1;
                    rht -= 1;
                }
            }
        }
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_solution() {
        let nums = vec![1, 2, -2, -1];
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(three_sum(nums), expected);
    }

    #[test]
    fn test_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
        assert_eq!(three_sum(nums), expected);
    }

    #[test]
    fn test_positive_and_negative() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let expected: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(three_sum(nums), expected);
    }

    #[test]
    fn test_large_numbers() {
        let nums = vec![1000, -1000, 0, -999, 999];
        let expected: Vec<Vec<i32>> = vec![vec![-1000, 0, 1000], vec![-999, 0, 999]];
        assert_eq!(three_sum(nums), expected);
    }
}
