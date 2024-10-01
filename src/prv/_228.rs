/// 228. Summary Ranges
///
/// You are given a sorted unique integer array nums.
///
/// A range [a,b] is the set of all integers from a to b (inclusive).
///
/// Return the smallest sorted list of ranges that cover
/// all the numbers in the array exactly. That is, each
/// element of nums is covered by exactly one of the ranges,
/// and there is no integer x such that x is in one of the
/// ranges but not in nums.
///
/// Each range [a,b] in the list should be output as:
/// * "a->b" if a != b
/// * "a" if a == b
///
/// Constraints:
/// * 0 <= nums.length <= 20
/// * -2^31 <= nums[i] <= 2^31 - 1
/// * All the values of nums are unique.
/// * nums is sorted in ascending order.

fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    // `ret` contributes O(n) space complexity.
    let mut ret = Vec::new();
    let mut rht: usize = 0;
    let len = nums.len();

    // Loop through each element in the `nums` array.
    // Contributes O(n) time complexity.
    while rht < len {
        let lft = rht;

        // Search for a contiguous sequence.
        while rht + 1 < len && nums[rht + 1] == nums[rht] + 1 {
            rht += 1;
        }

        // Store the result.
        if lft == rht {
            ret.push(nums[lft].to_string());
        } else {
            ret.push(format!("{}->{}", nums[lft], nums[rht]));
        }

        rht += 1;
    }

    ret
}

fn summary_ranges_a(nums: Vec<i32>) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;

    while i < nums.len() {
        let start = i;

        while i + 1 < nums.len() && nums[i + 1] == nums[i] + 1 {
            i += 1;
        }

        if start == i {
            result.push(nums[start].to_string());
        } else {
            result.push(format!("{}->{}", nums[start], nums[i]));
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(summary_ranges(vec![]), vec![] as Vec<String>);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(summary_ranges(vec![5]), vec!["5"]);
    }

    #[test]
    fn test_continuous_range() {
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }

    #[test]
    fn test_disjoint_elements() {
        assert_eq!(summary_ranges(vec![1, 3, 5, 7]), vec!["1", "3", "5", "7"]);
    }

    #[test]
    fn test_single_range() {
        assert_eq!(summary_ranges(vec![1, 2, 3, 4, 5]), vec!["1->5"]);
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
    }
}
