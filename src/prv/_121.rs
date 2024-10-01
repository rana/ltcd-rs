/// 121. Best Time to Buy and Sell S
///
/// You are given an array prices where prices[i]
/// is the price of a given stock on the ith day.
///
/// You want to maximize your profit by choosing
/// a single day to buy one stock and choosing a
/// different day in the future to sell that stock.
///
/// Return the maximum profit you can achieve from
/// this transaction. If you cannot achieve any
/// profit, return 0.
///
/// Constraints:
/// * 1 <= prices.length <= 10^5
/// * 0 <= prices[i] <= 10^4

fn max_profit_b(prices: Vec<i32>) -> i32 {
    // Variables contribute to O(1) space complexity.
    let mut min_prc = i32::MAX;
    let mut max_prf: i32 = 0;

    // Loop contributes to O(n) time complexity.
    for prc in prices {
        if prc < min_prc {
            min_prc = prc;
        } else if prc - min_prc > max_prf {
            max_prf = prc - min_prc;
        }
    }

    max_prf
}

fn max_profit_a(prices: Vec<i32>) -> i32 {
    let mut prf_max = 0i32;
    let mut prc_min = i32::MAX;

    for prc in prices {
        if prc < prc_min {
            prc_min = prc;
        } else if prc - prc_min > prf_max {
            prf_max = prc - prc_min;
        }
    }

    prf_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_max_profit_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = max_profit_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![7, 1, 5, 3, 6, 4],
                ret: 5,
            },
            Tst {
                nums: vec![7, 6, 4, 3, 1],
                ret: 0,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
