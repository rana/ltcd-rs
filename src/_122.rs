// Time Complexity: O(n), where n is the length of the array.
// Space Complexity: O(1), a constant amount of space is used.
// https://chatgpt.com/c/67030c24-80ac-8007-934c-5e25853daa4f
pub fn max_profit2(prices: Vec<i32>) -> i32 {
    // Buy and sell stock with multiple transactions.
    // Given an array of stock prices.
    // Determine a max profit.
    // Return the max profit; or, zero.
    // A key observation is to sell after bought on
    // the next day if there is any profit. Then re-buy
    // immediately. This amounts to summing any positive
    // difference.
    let mut max_prf: i32 = 0;

    // Loop through each element.
    for n in 1..prices.len() {
        if prices[n] > prices[n - 1] {
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
