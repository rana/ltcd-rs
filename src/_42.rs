/// 42. Trapping Rain Water
///
/// Given n non-negative integers representing an elevation
/// map where the width of each bar is 1, compute how much
/// water it can trap after raining.
///
/// Constraints:
/// * n == height.length
/// * 1 <= n <= 2 * 10^4
/// * 0 <= height[i] <= 105

fn trap_b(height: Vec<i32>) -> i32 {
    // Variables contribute to O(1) space complexity.
    let mut lft: usize = 0;
    let mut rht: usize = height.len() - 1;
    let mut lft_max = height[lft];
    let mut rht_max = height[rht];
    let mut water: i32 = 0;

    // Search for water capacity of valleys.
    // Loop contributes to O(n) time complexity.
    while lft < rht {
        if height[lft] <= height[rht] {
            if height[lft] >= lft_max {
                lft_max = height[lft];
            } else {
                water += lft_max - height[lft];
            }
            lft += 1;
        } else {
            if height[rht] >= rht_max {
                rht_max = height[rht];
            } else {
                water += rht_max - height[rht];
            }
            rht -= 1;
        }
    }

    water
}

fn trap_a(height: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    let mut idx_lft: usize = 0;
    let mut idx_rht: usize = height.len() - 1;
    let mut max_lft: i32 = 0;
    let mut max_rht: i32 = 0;

    while idx_lft < idx_rht {
        if height[idx_lft] < height[idx_rht] {
            // Right side is descending.
            if height[idx_lft] >= max_lft {
                max_lft = height[idx_lft];
            } else {
                sum += max_lft - height[idx_lft];
            }
            idx_lft += 1;
        } else {
            // Left side is descending; or, flat.
            if height[idx_rht] >= max_rht {
                max_rht = height[idx_rht];
            } else {
                sum += max_rht - height[idx_rht];
            }
            idx_rht -= 1;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_trap_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = trap_b(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_trap_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = trap_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
                ret: 6,
            },
            Tst {
                nums: vec![4, 2, 0, 3, 2, 5],
                ret: 9,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
