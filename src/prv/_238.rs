/// 238. Product of Array Except Self
///
/// Given an integer array nums, return an array answer
/// such that answer[i] is equal to the product of all
/// the elements of nums except nums[i].
///
/// The product of any prefix or suffix of nums is
/// guaranteed to fit in a 32-bit integer.
///
/// You must write an algorithm that runs in O(n)
/// time and without using the division operation.
///
/// Constraints:
/// * 2 <= nums.length <= 10^5
/// * -30 <= nums[i] <= 30
/// * The product of any prefix or suffix of nums
/// is guaranteed to fit in a 32-bit integer.

fn product_except_self_b(nums: Vec<i32>) -> Vec<i32> {
    // Variables contribute to O(1) space complexity.
    let len = nums.len();
    let mut ret: Vec<i32> = vec![0i32; len];

    // Create left products.
    // Loop through `nums` array.
    // Loop contributes to O(n) time complexity.
    let mut lft_prd: i32 = 1;
    for idx in 0..len {
        ret[idx] = lft_prd;
        // Multiplication after assignment skips
        // storing current num.
        lft_prd *= nums[idx];
    }

    // Create right products. Start from end.
    // Loop through `nums` array.
    // Loop contributes to O(n) time complexity.
    let mut rht_prd: i32 = 1;
    for idx in (0..len).rev() {
        // Multiply existing elements in `ret`.
        // Existing elements have left products.
        ret[idx] *= rht_prd;
        // Multiplication after assignment skips
        // storing current num.
        rht_prd *= nums[idx];
    }

    ret
}

fn product_except_self_a(nums: Vec<i32>) -> Vec<i32> {
    let len = nums.len();
    let mut ret: Vec<i32> = vec![0; len];

    // Build left prefix products.
    ret[0] = 1;
    for i in 1..len {
        ret[i] = ret[i - 1] * nums[i - 1];
    }

    // Build right suffix products.
    let mut rht: i32 = 1;
    for i in (0..len).rev() {
        ret[i] *= rht;
        rht *= nums[i];
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_product_except_self_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = product_except_self_b(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_product_except_self_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = product_except_self_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![1, 2, 3, 4],
                ret: vec![24, 12, 8, 6],
            },
            Tst {
                nums: vec![-1, 1, 0, -3, 3],
                ret: vec![0, 0, 9, 0, 0],
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: Vec<i32>,
    }
}
