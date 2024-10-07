pub fn max_profit2(prices: Vec<i32>) -> i32 {
    // Given an array of stock prices.
    // You can buy and sell on different days.
    // Multiple transacations are possible.
    // Find a maximum profit possible.
    // Use a local optimization "greedy" algorithm.
    // Key obsevation is that we can attempt to
    // buy one day, and sell the next day, if profitable.
    // Thus allowing accumulation.

    let mut max_prf: i32 = 0;
    for n in 1..prices.len() {
        if prices[n - 1] < prices[n] {
            max_prf += prices[n] - prices[n - 1];
        }
    }

    max_prf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_valid() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit2(prices), 7); // Buy at 1, sell at 5; buy at 3, sell at 6

        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit2(prices), 4); // Buy at 1, sell at 5 (increasing prices)

        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit2(prices), 0); // No transactions, since prices are decreasing
    }

    #[test]
    fn test_max_profit_single_day() {
        let prices = vec![5];
        assert_eq!(max_profit2(prices), 0); // Only one day, no transactions possible
    }

    #[test]
    fn test_max_profit_empty() {
        let prices: Vec<i32> = vec![];
        assert_eq!(max_profit2(prices), 0); // Empty array, no transactions
    }
}
