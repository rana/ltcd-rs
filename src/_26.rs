/// 26. Remove Duplicates from Sorted Array
///
/// Given an integer array nums sorted in non-decreasing
/// order, remove the duplicates in-place such that each
/// unique element appears only once. The relative order
/// of the elements should be kept the same. Then return
/// the number of unique elements in nums.
///
/// Consider the number of unique elements of nums to be k,
/// to get accepted, you need to do the following things:
/// * Change the array nums such that the first k elements
/// of nums contain the unique elements in the order they
/// were present in nums initially. The remaining elements
/// of nums are not important as well as the size of nums.
/// * Return k.
///
/// Constraints:
/// * 1 <= nums.length <= 3 * 10^4
/// * -100 <= nums[i] <= 100
/// * nums is sorted in non-decreasing order.

fn remove_duplicates_c(nums: &mut [i32]) -> i32 {
    // Move unique elements to the left.
    // Remove duplicate elements from the front of nums.
    // The array is sorted.

    // Initialize left pointer variable.
    // Variable contributes to O(1) space complexity.
    // `lft` points to a unique element.
    // `lft + 1` may be unique or duplicate.
    let mut lft: usize = 0;

    // Loop until the right pointer is complete.
    // Loop contributes to O(n) time complexity.
    for rht in 1..nums.len() {
        // Search for right unique element that can move left.
        if nums[lft] != nums[rht] {
            // Overwrite left element with right element.
            // Either:
            // * Write unique right element to same index.
            // * Write unique right element to left duplicate element.
            lft += 1;
            nums[lft] = nums[rht];
        }
    }

    lft as i32 + 1
}

fn remove_duplicates_b(nums: &mut [i32]) -> i32 {
    // Slide unique elements to the left.
    // Removes duplicate elements from a sorted array in-place.

    // Initialize two pointer variables.
    // `lft` points to a unique element.
    // `lft + 1` may be unique or duplicate.
    // Variables contribute to O(1) space complexity.
    let mut lft: usize = 0;
    let mut rht: usize = 1;
    let len = nums.len();

    // Loop until the right pointer is complete.
    // Loop contributes to O(n) time complexity.
    while rht < len {
        // Search for right unique element that can move left.
        if nums[lft] != nums[rht] {
            // Either:
            // * Write unique right element to same index.
            // * Write unique right element to left duplicate element.
            lft += 1;
            nums[lft] = nums[rht];
        }

        rht += 1;
    }

    lft as i32 + 1
}

fn remove_duplicates_a(nums: &mut [i32]) -> i32 {
    let mut idx_ins: usize = 1;
    for idx in 1..nums.len() {
        if nums[idx - 1] != nums[idx] {
            nums[idx_ins] = nums[idx];
            idx_ins += 1;
        }
    }

    idx_ins as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_remove_duplicates_c() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let act = remove_duplicates_c(&mut t.nums);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
            assert!(t.nums[..t.ret as usize].is_sorted(), "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_remove_duplicates_b() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let act = remove_duplicates_b(&mut t.nums);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
            assert!(t.nums[..t.ret as usize].is_sorted(), "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_remove_duplicates_a() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let act = remove_duplicates_a(&mut t.nums);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
            assert!(t.nums[..t.ret as usize].is_sorted(), "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![1, 1, 2],
                ret: 2,
            },
            Tst {
                nums: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
                ret: 5,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
