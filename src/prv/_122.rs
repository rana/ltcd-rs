/// 122. Best Time to Buy and Sell Stock II
///
/// You are given an integer array prices where
/// prices[i] is the price of a given stock on
/// the ith day.
///
/// On each day, you may decide to buy and/or sell
/// the stock. You can only hold at most one share
/// of the stock at any time. However, you can buy
/// it then immediately sell it on the same day.
///
/// Constraints:
/// * 1 <= prices.length <= 3 * 10^4
/// * 0 <= prices[i] <= 104

fn max_profit_b(prices: Vec<i32>) -> i32 {
    // Variable contributes to O(1) space complexity.
    let mut prf: i32 = 0;

    // Loop contributes to O(n) time complexity.
    for idx in 1..prices.len() {
        // Is the current prices great than the previous price?
        if prices[idx - 1] < prices[idx] {
            prf += prices[idx] - prices[idx - 1];
        }
    }

    prf
}

fn max_profit_a(prices: Vec<i32>) -> i32 {
    let mut prf_max = 0i32;

    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            prf_max += prices[i] - prices[i - 1];
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
            let act = max_profit_a(t.prices.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                prices: vec![7, 1, 5, 3, 6, 4],
                ret: 7,
            },
            Tst {
                prices: vec![1, 2, 3, 4, 5],
                ret: 4,
            },
            Tst {
                prices: vec![7, 6, 4, 3, 1],
                ret: 0,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        prices: Vec<i32>,
        ret: i32,
    }
}
