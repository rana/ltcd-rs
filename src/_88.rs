/// 88. Merge Sorted Array
///
/// You are given two integer arrays nums1 and nums2,
/// sorted in non-decreasing order, and two integers m and n,
/// representing the number of elements in nums1 and nums2
/// respectively.
///
/// Merge nums1 and nums2 into a single array sorted in
/// non-decreasing order.
///
/// The final sorted array should not be returned by the
/// function, but instead be stored inside the array nums1.
/// To accommodate this, nums1 has a length of m + n, where
/// the first m elements denote the elements that should be
/// merged, and the last n elements are set to 0 and should
/// be ignored. nums2 has a length of n.
///
/// Constraints:
/// * nums1.length == m + n
/// * nums2.length == n
/// * 0 <= m, n <= 200
/// * 1 <= m + n <= 200
/// * -10^9 <= nums1[i], nums2[j] <= 10^9
pub fn merge_d(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    // Cast m and n to usize for use in indexes.
    let (m, n) = (m as usize, n as usize);

    // Use iterators to handle index boundry conditions.

    // Create read iterators of indexes for nums1 and nums2.
    // Iterate from back to front. Vectors are given in ascending order.
    let mut itr1 = (0..m).rev(); // A time complexity O(m) portion.
    let mut itr2 = (0..n).rev(); // A time complexity O(n) portion.

    // Initialize first indexes.
    let mut opt1 = itr1.next();
    let mut opt2 = itr2.next();

    // Create a merge iterator of indexes for nums1.
    // Insert from back to front.
    let itr_mrg = (0..m + n).rev(); // A time complexity O(m + n) portion.

    // Loop until merge iterator is complete.
    for idx_mrg in itr_mrg {
        match (opt1, opt2) {
            // Merge the larger element from nums1 or nums2.
            (Some(idx1), Some(idx2)) => {
                nums1[idx_mrg] = if nums1[idx1] > nums2[idx2] {
                    opt1 = itr1.next();
                    nums1[idx1]
                } else {
                    opt2 = itr2.next();
                    nums2[idx2]
                };
            }
            // If nums2 has remaining elements, copy them to nums1.
            (None, Some(idx2)) => {
                nums1[idx_mrg] = nums2[idx2];
                opt2 = itr2.next();
            }
            _ => {}
        }
    }
}

pub fn merge_c(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    // Cast m and n to uszie for use in indexes.
    let (m, n) = (m as usize, n as usize);

    // Create index iterators for nums1 and nums2.
    // Use iterators to address boundry conditions.
    // Iterate from back to front. Vectors are in ascending order.
    let mut itr1 = (0..m).rev(); // A time complexity O(m) portion.
    let mut itr2 = (0..n).rev(); // The time complexity O(n) portion.

    // Create a merge index.
    // Insert from back to front of entire nums1.
    let mut idx_mrg: usize = m + n - 1;

    // Initialize first read indexes.
    let mut opt1 = itr1.next();
    let mut opt2 = itr2.next();

    // Loop until one or both iterators are complete.
    while let (Some(idx1), Some(idx2)) = (opt1, opt2) {
        nums1[idx_mrg] = if nums1[idx1] > nums2[idx2] {
            opt1 = itr1.next();
            nums1[idx1]
        } else {
            opt2 = itr2.next();
            nums2[idx2]
        };

        idx_mrg = idx_mrg.wrapping_sub(1);
    }

    // If nums2 has remaining elements, copy them to nums1.
    while let Some(idx2) = opt2 {
        nums1[idx_mrg] = nums2[idx2];
        opt2 = itr2.next();
        idx_mrg = idx_mrg.wrapping_sub(1);
    }
}

fn merge_b(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    // Merge arrays.
    // Arrays are sorted in ascending order.
    // Start from the end of each array.
    // End elements will be the largest.

    let mut idx1 = (m as usize).wrapping_sub(1);
    let mut idx2 = (n as usize).wrapping_sub(1);
    let mut idx_wrt = nums1.len().wrapping_sub(1); // The write index.

    // Insert the larger of the two elements first.
    // Insert the larger of the two elements at the end of the target array.
    // It's possible to complete one array before another.
    // Check for the beyond bounds conditions.
    // Use `wrapping_sub` function to support usize indexing.
    // Function `0.wrapping_sub(1)` will return `usize::MAX`.
    while idx2 != usize::MAX {
        if idx1 != usize::MAX && nums1[idx1] > nums2[idx2] {
            nums1[idx_wrt] = nums1[idx1];
            idx1 = idx1.wrapping_sub(1);
        } else {
            nums1[idx_wrt] = nums2[idx2];
            idx2 = idx2.wrapping_sub(1);
        }

        idx_wrt = idx_wrt.wrapping_sub(1);
    }
}

fn merge_a(nums1: &mut [i32], m: i32, nums2: &mut [i32], n: i32) {
    // Merge arrays which are ascending order sorted.
    // Start from the end of each array.
    // End elements will be the largest.

    let mut idx1 = (m as usize).wrapping_sub(1);
    let mut idx2 = (n as usize).wrapping_sub(1);
    let mut idx_wrt = nums1.len().wrapping_sub(1); // The write index.

    // Insert the larger of the two elements first.
    // Insert the larger of the two elements at the end of the target array.
    // Be careful of one list completing before another.
    while let Some(&num2) = nums2.get(idx2) {
        let num1_o = nums1.get(idx1);
        if num1_o.is_some() && nums1[idx1] > num2 {
            nums1[idx_wrt] = nums1[idx1];
            idx1 = idx1.wrapping_sub(1);
        } else {
            nums1[idx_wrt] = num2;
            idx2 = idx2.wrapping_sub(1);
        }
        idx_wrt = idx_wrt.wrapping_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{bail, Result};
    use ben::*;
    use std::fmt;
    use Lbl::*;

    #[test]
    fn tst_merge_d() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            merge_d(&mut t.nums1, t.m, &mut t.nums2, t.n);
            assert!(t.nums1.is_sorted(), "idx:{} {:?}", idx, t);
            assert_eq!(t.nums1, t.ret, "idx:{} {:?}", idx, t)
        }
    }

    #[test]
    fn tst_merge_c() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            merge_c(&mut t.nums1, t.m, &mut t.nums2, t.n);
            assert!(t.nums1.is_sorted(), "idx:{} {:?}", idx, t);
            assert_eq!(t.nums1, t.ret, "idx:{} {:?}", idx, t)
        }
    }

    #[test]
    fn tst_merge_b() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            merge_b(&mut t.nums1, t.m, &mut t.nums2, t.n);
            assert!(t.nums1.is_sorted(), "idx:{} {:?}", idx, t);
            assert_eq!(t.nums1, t.ret, "idx:{} {:?}", idx, t)
        }
    }

    #[test]
    fn tst_merge_a() {
        for (idx, t) in tsts().iter_mut().enumerate() {
            merge_a(&mut t.nums1, t.m, &mut t.nums2, t.n);
            assert!(t.nums1.is_sorted(), "idx:{} {:?}", idx, t);
            assert_eq!(t.nums1, t.ret, "idx:{} {:?}", idx, t)
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums1: vec![1, 2, 3, 0, 0, 0],
                m: 3,
                nums2: vec![2, 5, 6],
                n: 3,
                ret: vec![1, 2, 2, 3, 5, 6],
            },
            Tst {
                nums1: vec![1],
                m: 1,
                nums2: vec![],
                n: 0,
                ret: vec![1],
            },
            Tst {
                nums1: vec![0],
                m: 0,
                nums2: vec![1],
                n: 1,
                ret: vec![1],
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums1: Vec<i32>,
        m: i32,
        nums2: Vec<i32>,
        n: i32,
        ret: Vec<i32>,
    }

    #[test]
    fn mtr() {
        let mut stdy = Stdy::new();
        let itr: u16 = 64;

        // Register metric functions.

        stdy.reg_bld(&[A], |x| {
            x.ins_prm(Len(1), |tme| {
                let mut t = tsts()[0].clone();
                tme.borrow_mut().start();
                merge_a(&mut t.nums1, t.m, &mut t.nums2, t.n);
                tme.borrow_mut().stop();
                t.nums1
            });
        });
        stdy.reg_bld(&[B], |x| {
            x.ins_prm(Len(1), |tme| {
                let mut t = tsts()[0].clone();
                tme.borrow_mut().start();
                merge_b(&mut t.nums1, t.m, &mut t.nums2, t.n);
                tme.borrow_mut().stop();
                t.nums1
            });
        });
        stdy.reg_bld(&[D], |x| {
            x.ins_prm(Len(1), |tme| {
                let mut t = tsts()[0].clone();
                tme.borrow_mut().start();
                merge_d(&mut t.nums1, t.m, &mut t.nums2, t.n);
                tme.borrow_mut().stop();
                t.nums1
            });
        });

        // Define function queries.
        let mut qry = QryBld::new();
        let a_id = qry.sel(&[A]);
        let b_id = qry.sel(&[B]);
        let d_id = qry.sel(&[D]);

        qry.cmp(a_id, b_id);
        qry.cmp(d_id, b_id);

        // Run metric functions.
        stdy.run(qry, itr).expect("err");
    }

    /// Benchmark labels.
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub enum Lbl {
        A,
        B,
        D,
        Len(u32),
    }
    impl fmt::Display for Lbl {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                A => write!(f, "a"),
                B => write!(f, "b"),
                D => write!(f, "d"),
                Len(x) => {
                    if f.alternate() {
                        write!(f, "len")
                    } else {
                        write!(f, "len({})", x)
                    }
                }
            }
        }
    }
    impl EnumStructVal for Lbl {
        fn val(&self) -> Result<u32> {
            match *self {
                Len(x) => Ok(x),
                _ => bail!("label '{}' isn't a struct enum", self),
            }
        }
    }
    impl Label for Lbl {}
}
