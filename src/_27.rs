/// 27. Remove Element
///
/// Given an integer array nums and an integer val,
/// remove all occurrences of val in nums in-place.
/// The order of the elements may be changed.
/// Then return the number of elements in nums which
/// are not equal to val.
///
/// Consider the number of elements in nums which are
/// not equal to val be k, to get accepted, you need
/// to do the following things:
/// * Change the array nums such that the first k elements
/// of nums contain the elements which are not equal to val.
/// The remaining elements of nums are not important as well
/// as the size of nums.
/// * Return k.
///
/// Constraints:
/// * 0 <= nums.length <= 100
/// * 0 <= nums[i] <= 50
/// * 0 <= val <= 100

fn remove_element_c(nums: &mut [i32], val: i32) -> i32 {
    // Slide all non-val elements to the left.
    // Removes `val` from front of nums.

    // Initialize two pointer variables.
    // Variables contribute to O(1) space complexity.
    let mut lft: usize = 0;
    let mut rht: usize = 0;
    let len = nums.len();

    // Loop until the right pointer is complete.
    // Loop contributes to O(n) time complexity.
    while rht < len {
        // Check whether the current left value is equal to `val`.

        // Search for a right value that can move left.
        if nums[rht] != val {
            // Overwrite left value with right value.
            nums[lft] = nums[rht];
            lft += 1;
        }

        rht += 1;
    }

    // Left is now the length array without `val`.
    lft as i32
}

fn remove_element_b(nums: &mut [i32], val: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;

    while j < nums.len() {
        if nums[j] != val {
            nums[i] = nums[j];
            i += 1;
        }
        j += 1;
    }

    i as i32
}
fn remove_element_a(nums: &mut [i32], val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut idx: usize = 0;
    let mut len = nums.len();

    while idx < len {
        if nums[idx] == val {
            // Swap value from right to left.
            // Overwrite left value and forget left value.
            nums[idx] = nums[len - 1];
            // Decrement the length and right index.
            len -= 1;
        } else {
            idx += 1;
        }
    }

    // Return the count of values not equal to `val`.
    len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_remove_element_c() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let act = remove_element_c(&mut t.nums, t.val);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_remove_element_b() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let act = remove_element_b(&mut t.nums, t.val);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_remove_element_a() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            let act = remove_element_a(&mut t.nums, t.val);
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![3, 2, 2, 3],
                val: 3,
                ret: 2,
            },
            Tst {
                nums: vec![0, 1, 2, 2, 3, 0, 4, 2],
                val: 2,
                ret: 5,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        val: i32,
        ret: i32,
    }
}
