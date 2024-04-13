/// 55. Jump Game
///
/// You are given an integer array nums. You are
/// initially positioned at the array's first index,
/// and each element in the array represents your
/// maximum jump length at that position.
///
/// 1 <= nums.length <= 10^4
/// 0 <= nums[i] <= 10^5

fn can_jump_b(nums: Vec<i32>) -> bool {
    // Variables contribute to O(1) space complexity.
    let idx_lst = nums.len() - 1;
    let mut idx_max: usize = 0;

    // Loop contributes to O(n) time complexity.
    for idx in 0..nums.len() {
        // Can we jump no further?
        if idx > idx_max {
            return false;
        }

        // Search for the max possible jump
        // from the current index.
        idx_max = idx_max.max(idx + nums[idx] as usize);
        if idx_max >= idx_lst {
            return true;
        }
    }

    true
}

fn can_jump_a(nums: Vec<i32>) -> bool {
    let mut prv = nums[0]; // base
    let mut cur = 0;
    for i in 1..(nums.len() as i32) {
        // order: left -> right
        cur = if prv >= i {
            prv.max(nums[i as usize] + i)
        } else {
            return false; // can't move further
        };
        prv = cur;
    }
    cur >= (nums.len() - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_can_jump_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = can_jump_b(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_can_jump_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = can_jump_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![2, 3, 1, 1, 4],
                ret: true,
            },
            Tst {
                nums: vec![3, 2, 1, 0, 4],
                ret: false,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: bool,
    }
}
