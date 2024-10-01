/// 274. H-Index
///
/// Given an array of integers citations where citations[i]
/// is the number of citations a researcher received for
/// their ith paper, return the researcher's h-index.
///
/// According to the definition of h-index on Wikipedia:
/// The h-index is defined as the maximum value of h such
/// that the given researcher has published at least h
/// papers that have each been cited at least h times.
///
/// Constraints:
/// * n == citations.length
/// * 1 <= n <= 5000
/// * 0 <= citations[i] <= 1000

fn h_index_b(mut citations: Vec<i32>) -> i32 {
    // Sort the citations.
    // The definition of an H-Index relies on sorting.
    // Contributes to the O(nlogn) time complexity.
    citations.sort_unstable();

    // Variables contribute to O(1) space complexity.
    let mut h_idx: i32 = 0;
    let len = citations.len();

    // Loop through each citation.
    // Contributes to the O(n) time complexity.
    for idx in 0..len {
        // Calculate the current potential H-Index.
        let h_idx_cur = (len - idx) as i32;
        if citations[idx] >= h_idx_cur {
            h_idx = h_idx.max(h_idx_cur);
        }
    }

    h_idx
}

fn h_index_a(mut citations: Vec<i32>) -> i32 {
    // Sort in ascending order.
    citations.sort_unstable();
    let len = citations.len() as i32;
    let mut i: i32 = 0;
    while i < len && citations[(len - 1 - i) as usize] > i {
        i += 1;
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_h_index_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = h_index_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![3, 0, 6, 1, 5],
                ret: 3,
            },
            Tst {
                nums: vec![1, 3, 1],
                ret: 1,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
