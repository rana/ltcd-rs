/// 167. Two Sum II - Input Array Is Sorted
///
/// Given a 1-indexed array of integers numbers that is
/// already sorted in non-decreasing order, find two
/// numbers such that they add up to a specific target
/// number. Let these two numbers be numbers[index1]
/// and numbers[index2] where
/// 1 <= index1 < index2 <= numbers.length.
///
/// Return the indices of the two numbers, index1 and
/// index2, added by one as an integer array
/// [index1, index2] of length 2.
///
/// The tests are generated such that there is exactly
/// one solution. You may not use the same element twice.
///
/// Your solution must use only constant extra space.
///
/// Constraints:
/// * 2 <= numbers.length <= 3 * 104
/// * -1000 <= numbers[i] <= 1000
/// * numbers is sorted in non-decreasing order.
/// * -1000 <= target <= 1000
/// * The tests are generated such that there is exactly one solution.

fn two_sum(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
    use std::cmp::Ordering;

    // Variables contribute to O(1) space complexity.
    let mut lft: usize = 0;
    let mut rht: usize = nums.len() - 1;

    // Loop through numbers.
    // Rely on the numbers being sorted to adjust the sum.
    // Loop contributes to O(n) time complexity.
    while lft < rht {
        let sum = nums[lft] + nums[rht];
        match sum.cmp(&tgt) {
            Ordering::Equal => {
                // We've found the solution.
                break;
            }
            Ordering::Less => lft += 1,
            Ordering::Greater => rht -= 1,
        }
    }

    vec![lft as i32 + 1, rht as i32 + 1]
}

fn two_sum_b(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let (mut left, mut right) = (0, numbers.len() - 1);

    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            // The problem requires 1-based indices, hence we return left+1 and right+1
            return vec![(left + 1) as i32, (right + 1) as i32];
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }

    vec![] // This line will never be reached because the problem guarantees exactly one solution
}

fn two_sum_a(nums: Vec<i32>, tgt: i32) -> Vec<i32> {
    use std::cmp::Ordering;

    let mut lft: usize = 0;
    let mut rht: usize = nums.len() - 1;

    while lft < rht {
        match (nums[lft] + nums[rht]).cmp(&tgt) {
            Ordering::Less => lft += 1,
            Ordering::Greater => rht -= 1,
            Ordering::Equal => break,
        }
    }

    vec![(lft + 1) as i32, (rht + 1) as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(two_sum(vec![1, 2, 3, 4, 5], 9), vec![4, 5]);
    }
}
