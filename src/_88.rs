pub fn merge(nums1: &mut [i32], m: i32, nums2: &[i32], n: i32) {
    // Arrays are sorted in ascending order.
    // Merge to end of the destination array.

    // Create indexes.
    let mut idx1 = m as isize - 1;
    let mut idx2 = n as isize - 1;
    let mut idx_dst = nums1.len() - 1;

    // Check for exit condition.
    // When `nums2` is  complete, values in `nums1` are merged.
    while idx2 >= 0 {
        if idx1 >= 0 && nums1[idx1 as usize] > nums2[idx2 as usize] {
            nums1[idx_dst] = nums1[idx1 as usize];
            idx1 -= 1;
        } else {
            nums1[idx_dst] = nums2[idx2 as usize];
            idx2 -= 1;
        }
        idx_dst -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_common_case() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let nums2 = vec![2, 5, 6];
        let m = 3;
        let n = 3;
        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_merge_empty_nums2() {
        let mut nums1 = vec![1, 2, 3];
        let nums2 = vec![];
        let m = 3;
        let n = 0;
        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 3]);
    }

    #[test]
    fn test_merge_single_element_in_each() {
        let mut nums1 = vec![1, 0];
        let nums2 = vec![2];
        let m = 1;
        let n = 1;
        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2]);
    }

    #[test]
    fn test_merge_large_case() {
        let mut nums1 = vec![1, 3, 5, 7, 9, 0, 0, 0, 0, 0];
        let nums2 = vec![2, 4, 6, 8, 10];
        let m = 5;
        let n = 5;
        merge(&mut nums1, m, &nums2, n);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
