/// 189. Rotate Array
///
/// Given an integer array nums, rotate the array
/// to the right by k steps, where k is non-negative.
///
/// Constraints:
/// * 1 <= nums.length <= 10^5
/// * -2^31 <= nums[i] <= 2^31 - 1
/// * 0 <= k <= 10^5

fn rotate_b(nums: &mut [i32], k: i32) {
    fn rev(nums: &mut [i32], mut lft: usize, mut rht: usize) {
        while lft < rht {
            nums.swap(lft, rht);
            lft += 1;
            rht -= 1;
        }
    }

    // Variables contribute to O(1) space complexity.
    let k = k as usize % nums.len();
    let lst = nums.len() - 1;

    // Check for `k` and `nums`
    // minimum edge condition.
    if k == 0 || lst == 0 {
        return;
    }

    // Reverse contributes to O(n) time complexity.
    rev(nums, 0, lst);
    rev(nums, 0, k - 1);
    rev(nums, k, lst);
}

fn rotate_a(nums: &mut [i32], k: i32) {
    let len = nums.len();
    let k = k as usize % len;

    let mut idx_fst: usize = 0;
    let mut cnt: usize = 0;

    while cnt < len {
        let mut idx_cur = idx_fst;
        let mut val_prv = nums[idx_fst];
        loop {
            let idx_nxt = (idx_cur + k) % len;

            std::mem::swap(&mut nums[idx_nxt], &mut val_prv);
            idx_cur = idx_nxt;
            cnt += 1;

            if idx_cur == idx_fst {
                break;
            }
        }
        idx_fst += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_rotate_b() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            rotate_b(&mut t.nums, t.k);
            assert_eq!(t.nums, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_rotate_a() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            rotate_a(&mut t.nums, t.k);
            assert_eq!(t.nums, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![1, 2, 3, 4, 5, 6, 7],
                k: 3,
                ret: vec![5, 6, 7, 1, 2, 3, 4],
            },
            Tst {
                nums: vec![-1, -100, 3, 99],
                k: 2,
                ret: vec![3, 99, -1, -100],
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        k: i32,
        ret: Vec<i32>,
    }
}
