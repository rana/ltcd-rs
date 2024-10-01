pub fn majority_element(nums: Vec<i32>) -> i32 {
    // Return the majority element.
    // A majority element occurs at least half the time.
    // Use the Boyer-Moore Voting technique.

    // Initialize supporting variables.
    // u16 is smallest type representing max nums length of 50,000.
    let mut candidate: i32 = 0;
    let mut cnt: u16 = 0;

    // loop through nums array.
    for num in nums {
        if cnt == 0 {
            candidate = num;
        }

        if candidate == num {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    // Candidate is the majority element.
    candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element_common_case() {
        let nums = vec![3, 2, 3];
        assert_eq!(majority_element(nums), 3);
    }

    #[test]
    fn test_majority_element_edge_case_single_element() {
        let nums = vec![1];
        assert_eq!(majority_element(nums), 1);
    }

    #[test]
    fn test_majority_element_multiple_majority_elements() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }

    #[test]
    fn test_majority_element_all_same_elements() {
        let nums = vec![5, 5, 5, 5];
        assert_eq!(majority_element(nums), 5);
    }

    #[test]
    fn test_majority_element_two_majority_candidates() {
        let nums = vec![1, 1, 2, 2, 1];
        assert_eq!(majority_element(nums), 1);
    }
}
