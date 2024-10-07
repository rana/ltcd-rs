// Time Complexity: O(nlogn), where logn is for sorting, and n is for iterating through each array element.
// Space Complexity: O(1), a constant amount of space is used.
// https://chatgpt.com/g/g-pWbYqrykB-rust/c/67031bd5-4914-8007-94b2-d4f6643a62a0
pub fn h_index(mut citations: Vec<i32>) -> i32 {
    // Sorty the citations in descending order.
    citations.sort_by(|a, b| b.cmp(a));

    // Find the h-index by iterating over the sorted list.
    let mut h_idx = 0;

    for (n, &citation) in citations.iter().enumerate() {
        // Add 1 because h-index is 1-based.
        if citation >= (n as i32 + 1) {
            h_idx = n as i32 + 1;
        } else {
            break;
        }
    }

    h_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_paper() {
        let citations = vec![5];
        assert_eq!(h_index(citations), 1);
    }

    #[test]
    fn test_all_zero_citations() {
        let citations = vec![0, 0, 0, 0];
        assert_eq!(h_index(citations), 0);
    }

    #[test]
    fn test_h_index_example() {
        let citations = vec![3, 0, 6, 1, 5];
        assert_eq!(h_index(citations), 3);
    }

    #[test]
    fn test_multiple_high_citations() {
        let citations = vec![10, 8, 5, 4, 3];
        assert_eq!(h_index(citations), 4);
    }

    #[test]
    fn test_no_citations() {
        let citations = vec![];
        assert_eq!(h_index(citations), 0);
    }

    #[test]
    fn test_low_citations() {
        let citations = vec![1, 1, 1, 1, 1];
        assert_eq!(h_index(citations), 1);
    }

    #[test]
    fn test_all_high_citations() {
        let citations = vec![10, 10, 10, 10, 10];
        assert_eq!(h_index(citations), 5);
    }

    #[test]
    fn test_duplicate_citations() {
        let citations = vec![3, 3, 3, 3, 3];
        assert_eq!(h_index(citations), 3);
    }
}
