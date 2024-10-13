// Time complexity: O(nlogn), where n is the length of the inrvls array. We sort the inrvls array, and we iterate through the array once.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/670af1b3-1fe4-8002-ad83-d4ae82fd897a
pub fn merge_intervals(mut inrvls: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // Merge Intervals
    // Given an array of intervals.
    // inrvls[i] = [start, end]
    // Merge overlapping intervals.
    // Return non-overlapping intervals.
    // Notice that the array is unsorted.
    // Sort the array by the starting value.

    // Check input minimum edge case.
    if inrvls.is_empty() {
        return vec![];
    }

    // Sort intervals by first pair value.
    inrvls.sort_by(|a: &Vec<i32>, b: &Vec<i32>| a[0].cmp(&b[0]));

    let mut res: Vec<Vec<i32>> = Vec::new();
    res.push(inrvls[0].clone());

    // Iterate through each interval.
    for cur in inrvls.iter().skip(1) {
        let prv: Vec<i32> = res.last().cloned().unwrap();

        // Check whether current interval overlaps previous interval.
        if cur[0] <= prv[1] {
            // Intervals overlap.
            // Merge intervals.
            res.pop();
            let mrg: Vec<i32> = vec![prv[0], prv[1].max(cur[1])];
            res.push(mrg);
        } else {
            // No overlap.
            // Add the current interval.
            res.push(cur.clone());
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_overlapping_intervals() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let res: Vec<Vec<i32>> = merge_intervals(mtx);
        let expected: Vec<Vec<i32>> = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        assert_eq!(res, expected);
    }

    #[test]
    fn merges_touching_intervals() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 4], vec![4, 5]];
        let res: Vec<Vec<i32>> = merge_intervals(mtx);
        let expected: Vec<Vec<i32>> = vec![vec![1, 5]];
        assert_eq!(res, expected);
    }

    #[test]
    fn handles_empty_input() {
        let mtx: Vec<Vec<i32>> = vec![];
        let res: Vec<Vec<i32>> = merge_intervals(mtx);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(res, expected);
    }

    #[test]
    fn handles_single_interval() {
        let mtx: Vec<Vec<i32>> = vec![vec![5, 7]];
        let res: Vec<Vec<i32>> = merge_intervals(mtx);
        let expected: Vec<Vec<i32>> = vec![vec![5, 7]];
        assert_eq!(res, expected);
    }

    #[test]
    fn merges_multiple_overlaps() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        let res: Vec<Vec<i32>> = merge_intervals(mtx);
        let expected: Vec<Vec<i32>> = vec![vec![1, 6]];
        assert_eq!(res, expected);
    }

    #[test]
    fn no_overlaps() {
        let mtx: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let res: Vec<Vec<i32>> = merge_intervals(mtx);
        let expected: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(res, expected);
    }
}
