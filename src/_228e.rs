pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    // Summary Ranges
    // Given a sorted unique integer array `nums`.
    // Determine an array of consecutive sequences.
    // Condition:
    //  * Smallest and sorted list
    // Return array of consecutive sequences as strings.
    // Use a sliding window two-pointer technique.

    // Check for input minimum edge case.
    if nums.is_empty() {
        return vec![];
    }

    // Initialize variables.
    let mut res: Vec<String> = Vec::new();
    let mut lft: i32 = nums[0];
    let mut rht: i32 = nums[0];

    // Iterate through remaining integers.
    for num in nums.into_iter().skip(1) {
        // Check for consecutive sequence growth.
        if num == rht + 1 {
            rht += 1;
        } else {
            // Detected end of sequence.
            // Store sequence.
            if lft == rht {
                res.push(lft.to_string());
            } else {
                res.push(format!("{}->{}", lft, rht));
            }

            // Reset sliding window values.
            lft = num;
            rht = num;
        }
    }

    // Store the last consecutive sequence.
    if lft == rht {
        res.push(lft.to_string());
    } else {
        res.push(format!("{}->{}", lft, rht));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        let wrds: Vec<i32> = Vec::new();
        let rslt: Vec<String> = Vec::new();
        assert_eq!(summary_ranges(wrds), rslt);
    }

    #[test]
    fn single_element() {
        let wrds: Vec<i32> = vec![5];
        let rslt: Vec<String> = vec!["5".to_string()];
        assert_eq!(summary_ranges(wrds), rslt);
    }

    #[test]
    fn consecutive_ranges() {
        let wrds: Vec<i32> = vec![0, 1, 2, 4, 5, 7];
        let rslt: Vec<String> = vec!["0->2".to_string(), "4->5".to_string(), "7".to_string()];
        assert_eq!(summary_ranges(wrds), rslt);
    }

    #[test]
    fn non_consecutive_ranges() {
        let wrds: Vec<i32> = vec![0, 2, 3, 4, 6, 8, 9];
        let rslt: Vec<String> = vec![
            "0".to_string(),
            "2->4".to_string(),
            "6".to_string(),
            "8->9".to_string(),
        ];
        assert_eq!(summary_ranges(wrds), rslt);
    }

    #[test]
    fn all_consecutive() {
        let wrds: Vec<i32> = vec![1, 2, 3, 4, 5];
        let rslt: Vec<String> = vec!["1->5".to_string()];
        assert_eq!(summary_ranges(wrds), rslt);
    }

    #[test]
    fn no_consecutive() {
        let wrds: Vec<i32> = vec![1, 3, 5, 7];
        let rslt: Vec<String> = vec![
            "1".to_string(),
            "3".to_string(),
            "5".to_string(),
            "7".to_string(),
        ];
        assert_eq!(summary_ranges(wrds), rslt);
    }
}
