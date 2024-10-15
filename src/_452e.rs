// Time complexity: O(nlogn), n is the length of the array invs. We sort the array once. We traverse the array once.
// Space complexity: O(n), n auxilary space used for sorting.
pub fn find_min_arrow_shots(mut invs: Vec<Vec<i32>>) -> i32 {
    // Minimum Number of Arrows to Burst Ballons
    // Given an array of intervals `invs`.
    // Determine the min count of intersections.
    // Return the min count.
    // Notice that the array is unsorted.
    // Sort array by interval end point.
    // Count intersections.

    // Sort intervals by interval end-point.
    invs.sort_unstable_by(|a, b| a[1].cmp(&b[1]));

    // Initialize variable.
    let mut min_cnt: i32 = 1;
    let mut prv_end: i32 = invs[0][1];

    // Loop through remaining intervals.
    for cur_inv in invs.iter().skip(1) {
        // Check for non-overlapping condition.
        // Current start great than previous end?
        if cur_inv[0] > prv_end {
            // Found non-overlapping interval.
            min_cnt += 1;
            prv_end = cur_inv[1];
        }
    }

    min_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let pts = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(find_min_arrow_shots(pts), 2);
    }

    #[test]
    fn overlapping_balloons() {
        let pts = vec![vec![1, 5], vec![2, 6], vec![3, 7]];
        assert_eq!(find_min_arrow_shots(pts), 1);
    }

    #[test]
    fn non_overlapping_balloons() {
        let pts = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(find_min_arrow_shots(pts), 3);
    }

    #[test]
    fn single_balloon() {
        let pts = vec![vec![1, 2]];
        assert_eq!(find_min_arrow_shots(pts), 1);
    }

    #[test]
    fn max_range_balloons() {
        let pts = vec![vec![-2147483648, 2147483647], vec![-2147483648, 2147483647]];
        assert_eq!(find_min_arrow_shots(pts), 1);
    }
}
