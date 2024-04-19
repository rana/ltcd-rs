/// 80. Remove Duplicates from Sorted Array II
///
/// Given an integer array nums sorted in non-decreasing
/// order, remove some duplicates in-place such that each
/// unique element appears at most twice. The relative
///  order of the elements should be kept the same.
///
/// Since it is impossible to change the length of the array
/// in some languages, you must instead have the result be
/// placed in the first part of the array nums. More formally,
/// if there are k elements after removing the duplicates,
/// then the first k elements of nums should hold the final
/// result. It does not matter what you leave beyond the
/// first k elements.
///
/// Return k after placing the final result in the first
/// k slots of nums.
///
/// Do not allocate extra space for another array. You must
/// do this by modifying the input array in-place with O(1)
/// extra memory.
///
/// Constraints:
/// * 1 <= nums.length <= 3 * 10^4
/// * -104 <= nums[i] <= 10^4
/// * nums is sorted in non-decreasing order.

fn remove_duplicates_c(nums: &mut [i32]) -> i32 {
    // Move unique elements, and up to one duplicate left.
    // Remove excess duplicates.
    // More than two duplicate is an excess.

    /// `MAX_DUP` is the maximum number of duplicates.
    // Contributes to O(1) space complexity.
    const MAX_DUP: usize = 2;

    // Check for the nums length
    // minimum edge condition.
    if nums.len() <= MAX_DUP {
        return nums.len() as i32;
    }

    // Initialize left pointer variable.
    // Contributes to O(1) space complexity.
    // `lft` is the slow moving pointer.
    let mut lft: usize = MAX_DUP;

    // Loop until the right pointer is complete.
    // Contributes to O(n) time complexity.
    // `rht` is the fast moving pointer.
    for rht in MAX_DUP..nums.len() {
        // Search for right unique element that can move left.
        // `lft - MAX_DUP` points to a unique element.
        if nums[lft - MAX_DUP] < nums[rht] {
            // Move right element to the left.
            nums[lft] = nums[rht];
            lft += 1;
        }
    }

    // `lft` is the length of the processed array.
    lft as i32
}

fn remove_duplicates_b(nums: &mut [i32]) -> i32 {
    let mut lft = 0;
    for rht in 0..nums.len() {
        if lft < 2 || nums[lft - 2] < nums[rht] {
            nums[lft] = nums[rht];
            lft += 1;
        }
    }
    lft as i32
}

fn remove_duplicates_a(nums: &mut [i32]) -> i32 {
    let mut j: usize = 1;
    let mut cnt: usize = 1;

    for i in 1..nums.len() {
        if nums[i - 1] == nums[i] {
            cnt += 1;
        } else {
            cnt = 1;
        }

        if cnt <= 2 {
            nums[j] = nums[i];
            j += 1;
        }
    }

    j as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_remove_duplicates_c() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let len = remove_duplicates_b(&mut t.nums);
            assert_eq!(len, t.ret, "idx:{} {:?}", idx, t);
            println!("t.nums:{:?} len:{}", t.nums, len);
        }
    }

    #[test]
    fn tst_remove_duplicates_b() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let len = remove_duplicates_b(&mut t.nums);
            assert_eq!(len, t.ret, "idx:{} {:?}", idx, t);
            println!("t.nums:{:?} len:{}", t.nums, len);
        }
    }

    #[test]
    fn tst_remove_duplicates_a() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let len = remove_duplicates_a(&mut t.nums);
            assert_eq!(len, t.ret, "idx:{} {:?}", idx, t);
            println!("t.nums:{:?} act:{}", t.nums, len);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![1, 1, 1, 2, 2, 3],
                ret: 5,
            },
            Tst {
                nums: vec![0, 0, 1, 1, 1, 1, 2, 3, 3],
                ret: 7,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
