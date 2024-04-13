/// 169. Majority Element
///
/// Given an array nums of size n,
/// return the majority element.
///
/// The majority element is the element that appears more
/// than ⌊n / 2⌋ times. You may assume that the majority
/// element always exists in the array.
///
/// Constraints:
/// * n == nums.length
/// * 1 <= n <= 5 * 10^4
/// * -10^9 <= nums[i] <= 10^9

fn majority_element_c(nums: Vec<i32>) -> i32 {
    // Majority element occurs at least
    // half the length of `nums` times.

    // Initialize supporting variables.
    // Contributes to O(1) space complexity.
    let mut candidate: Option<i32> = None;
    let mut cnt: u16 = 0;

    // Loop through each number in `nums`.
    // Contributes to O(n) time complexity.
    for num in nums {
        if cnt == 0 {
            candidate = Some(num);
        }

        if candidate == Some(num) {
            cnt += 1;
        } else {
            cnt -= 1;
        }
    }

    // Candidate is now the majority element.
    candidate.expect("no majority element")
}

fn majority_element_b(nums: Vec<i32>) -> i32 {
    let mut candidate: Option<i32> = None;
    let mut count = 0;

    for &num in nums.iter() {
        if count == 0 {
            candidate = Some(num);
        }

        if candidate == Some(num) {
            count += 1;
        } else {
            count -= 1;
        }
    }

    // Since the problem guarantees the presence
    // of a majority element, no need to verify.
    candidate.expect("No candidate found")
}

fn majority_element_a(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut cnts: HashMap<i32, u16> = HashMap::with_capacity(nums.len());
    let lim = (nums.len() >> 1) as u16;

    for num in nums {
        // Count the frequency
        cnts.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
    }

    // Find the number with the maximum frequency.
    // Due to the limit flooring, possible to have
    // multiple values above the limit, with one
    // frequency larger.
    let mut max_num: i32 = i32::MAX;
    let mut max_cnt: u16 = 0;
    for (num, cnt) in cnts {
        if cnt >= lim && cnt > max_cnt {
            max_num = num;
            max_cnt = cnt;
        }
    }

    max_num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_remove_duplicates_c() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = majority_element_c(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_remove_duplicates_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = majority_element_b(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_remove_duplicates_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = majority_element_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![3, 2, 3],
                ret: 3,
            },
            Tst {
                nums: vec![2, 2, 1, 1, 1, 2, 2],
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
