pub fn insert_intervals(invs: Vec<Vec<i32>>, mut inv: Vec<i32>) -> Vec<Vec<i32>> {
    // Insert Interval
    // Given an array of non-overlapping intervals.
    // Array is sorted ascending.
    // Given a single interval.
    // Insert the interval into an array.
    // Condition:
    //  * Preserve the non-overlapping.
    //  * Preserve the ascending sort.
    //  * In-place not required.
    // Return a sort-ascending array of non-overlapping intervals.
    // Create a new array to simplify operation.

    let len: usize = invs.len();
    let mut idx: usize = 0;
    let mut res: Vec<Vec<i32>> = Vec::with_capacity(len + 1);

    // Insert intervals with end less than new interval start.
    while idx < len && invs[idx][1] < inv[0] {
        res.push(invs[idx].clone());
        idx += 1;
    }

    // Determine any merge value with new interval.
    // Existing starts are equal or greater than new interval start.
    // Search for starts less than or equal to new interval end.
    while idx < len && invs[idx][0] <= inv[1] {
        inv[0] = inv[0].min(invs[idx][0]);
        inv[1] = inv[1].max(invs[idx][1]);
        idx += 1;
    }
    res.push(inv);

    // Add remaining intervals.
    while idx < len {
        res.push(invs[idx].clone());
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
