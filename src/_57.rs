// Time complexity: O(n), n is the length of the `invls` array. We traverse the array once.
// Space complexity: O(n), constant auxiliary space used. 
// https://chatgpt.com/c/670d6236-7f44-8002-869a-8ebcc88fb276
pub fn insert_intervals(invls: Vec<Vec<i32>>, mut invl: Vec<i32>) -> Vec<Vec<i32>> {
    // Insert Interval
    // Given array `invls` of non-overlapping intervals.
    // The array is ascending sorted by the interval start.
    // Also given an interval `invl`.
    // Insert `inrvl` to an array:
    //  * Preserve the ascending sort.
    //  * Preserve non-overlapping.
    //  * May return a new array.
    // Return an array meeting criteria.

    // Initialize variables.
    let len: usize = invls.len();
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(len + 1);
    let mut idx: usize = 0;

    // Add intervals which come before the new interval.
    // Check existing end before new beginning.
    while idx < len && invls[idx][1] < invl[0] {
        res.push(invls[idx].clone());
        idx += 1;
    }

    // Merge existing intervals with new interval.
    // Check existing beginning before or equal to new end.
    // Select min start, and max end of comparison pair.
    // Existing start is great or equal to new start
    // due to previous loop check.
    while idx < len && invls[idx][0] <= invl[1] {
        invl[0] = invl[0].min(invls[idx][0]);
        invl[1] = invl[1].max(invls[idx][1]);
        idx += 1;
    }
    // Add the merged interval.
    res.push(invl);

    // Add intervals which come after the new interval.
    while idx < len {
        res.push(invls[idx].clone());
        idx += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_insert_no_merge() {
        let intervals = vec![vec![1, 2], vec![3, 5], vec![6, 7]];
        let new_interval = vec![8, 10];
        let res = insert_intervals(intervals, new_interval);
        assert_eq!(res, vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10]]);
    }

    #[test]
    fn valid_insert_merge_overlap() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let res = insert_intervals(intervals, new_interval);
        assert_eq!(res, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn valid_insert_merge_multiple() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 9];
        let res = insert_intervals(intervals, new_interval);
        assert_eq!(res, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }

    #[test]
    fn valid_insert_empty_intervals() {
        let intervals: Vec<Vec<i32>> = Vec::new();
        let new_interval = vec![5, 7];
        let res = insert_intervals(intervals, new_interval);
        assert_eq!(res, vec![vec![5, 7]]);
    }

    #[test]
    fn valid_insert_at_start() {
        let intervals = vec![vec![3, 5], vec![7, 9]];
        let new_interval = vec![1, 2];
        let res = insert_intervals(intervals, new_interval);
        assert_eq!(res, vec![vec![1, 2], vec![3, 5], vec![7, 9]]);
    }
}
