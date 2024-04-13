/// 45. Jump Game II
///
/// You are given a 0-indexed array of integers nums
/// of length n. You are initially positioned at nums[0].
///
/// Each element nums[i] represents the maximum length of
/// a forward jump from index i. In other words, if you
/// are at nums[i], you can jump to any nums[i + j] where:
/// * 0 <= j <= nums[i] and
/// * i + j < n
///
/// Return the minimum number of jumps to reach nums[n - 1].
/// The test cases are generated such that you can reach
/// nums[n - 1].
///
/// Constraints:
/// * 1 <= nums.length <= 104
/// * 0 <= nums[i] <= 1000
/// * It's guaranteed that you can reach nums[n - 1].

fn jump_b(nums: Vec<i32>) -> i32 {
    // Variables contribute to O(1) space complexity.
    let mut jmp_cnt: i32 = 0;
    let mut idx_max: usize = 0;
    let mut idx_max_cur: usize = 0;

    // Loop contributes to O(n) time complexity.
    for idx in 0..nums.len() - 1 {
        // Search for the max possible jump
        // from the current index.
        idx_max = idx_max.max(idx + nums[idx] as usize);

        // Check whether we've reached the recent max jump.
        if idx == idx_max_cur {
            // Each time we reach a previous maximum jump, increment.
            jmp_cnt += 1;
            idx_max_cur = idx_max;
        }
    }

    jmp_cnt
}

fn jump_a(nums: Vec<i32>) -> i32 {
    let mut jmp_cnt: i32 = 0;

    let idx_lst = nums.len() - 1; // Last index.
    let mut idx_cur: usize = 0; // Current index.
    let mut idx_max: usize = 0; // Highest index explored.

    while idx_max < idx_lst {
        let idx_lim = idx_lst.min(idx_cur + nums[idx_cur] as usize);
        idx_cur = (idx_max + 1..=idx_lim)
            .max_by(|&j, &k| (j + nums[j] as usize).cmp(&(k + nums[k] as usize)))
            .unwrap();
        idx_max = idx_lim;
        jmp_cnt += 1;
    }

    jmp_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_jump_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = jump_b(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_jump_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = jump_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![2, 3, 1, 1, 4],
                ret: 2,
            },
            Tst {
                nums: vec![2, 3, 0, 1, 4],
                ret: 2,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
