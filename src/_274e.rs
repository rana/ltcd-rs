pub fn h_index(mut citations: Vec<i32>) -> i32 {
    // Given an array of citations where 
    // citations[i] is the number of citations
    // received for the ith paper, return the h-index.
    
    // Sort descending.
    // Contributes time complexity O(logn)
    citations.sort_by(|a, b| b.cmp(a));

    // Initialize variables.
    let mut h_idx: i32 = 0;

    // Loop through each element.
    // Contributes time complexity O(n).
    for (n, &citation) in citations.iter().enumerate() {
        // Check for next largest  h-index.
        if citation > n as i32 {
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
