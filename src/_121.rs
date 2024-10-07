// Time Complexity: O(n), where n is the length of the array.
// Space Complexity: O(1), a constant amount of space is used.
// https://chatgpt.com/g/g-pWbYqrykB-rust/c/6703091f-d8b4-8007-b068-539dc9c95027
pub fn max_profit(prices: Vec<i32>) -> i32 {
    // Given an array of stock prices.
    // Maximize profit.
    // Buy and sell with one transation.
    // Return maximum profit; or, zero.

    // Use local optimization "greedy" algorithm.
    // Track the minimum price encountered so far.
    // Calculate the profit with the min price so far.
    // Track the max profit so far.

    // Check for minimum edge condition.
    if prices.is_empty() {
      return 0;
    }

    let mut min_prc = i32::MAX;
    let mut max_prf = 0;

    for prc in  prices {
      if prc < min_prc {
        min_prc = prc;
      } else if prc - min_prc > max_prf {
        max_prf = prc - min_prc;
      }
    }

    max_prf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_single_transaction() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_no_profit_possible() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_profit_on_last_day() {
        let prices = vec![2, 4, 1, 5, 3, 6];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_constant_prices() {
        let prices = vec![3, 3, 3, 3, 3];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_increasing_prices() {
        let prices = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn test_empty_array() {
        let prices: Vec<i32> = vec![];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_single_price() {
        let prices = vec![10];
        assert_eq!(max_profit(prices), 0);
    }

    #[test]
    fn test_two_prices_profit() {
        let prices = vec![1, 10];
        assert_eq!(max_profit(prices), 9);
    }

    #[test]
    fn test_two_prices_loss() {
        let prices = vec![10, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
