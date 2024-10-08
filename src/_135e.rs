// Time Complexity: O(n), where n is the length of the ratings array.
// Space Complexity: O(n), where n is the length of the ratings array.
pub fn candy(ratings: Vec<i32>) -> i32 {
    // Given an integer array of ratings.
    // Give candies to children.
    // * Minimum of 1 candy.
    // * Children with higher rating receive more candy than neighbor.
    // Return the min number of candies.
    // Use a local optimization "greedy" approach.
    // Use a two-pass approach.
    // Left pass evaluate left neighbor relation.
    // Right pass evaluates right neighbor relation.

    // Check min edge condition of input.
    let len = ratings.len();
    if len <= 1 {
        return len as i32;
    }

    // Initialize variables.
    // Assign each 1 by challenge definition.
    let mut candies = vec![1; len];

    // Left pass. Evaluate left neighbor relations.
    for n in 1..len {
        if ratings[n] > ratings[n - 1] {
            // Use the neighbor candy count.
            candies[n] = candies[n - 1] + 1;
        }
    }

    // Right pass. Evaluate right neighbor relations.
    for n in (0..len - 1).rev() {
        if ratings[n] > ratings[n + 1] && candies[n] <= candies[n + 1] {
            // Use the neighbor candy count.
            candies[n] = candies[n + 1] + 1;
        }
    }

    // Return the min candy count
    candies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_case() {
        let ratings = vec![1, 0, 2];
        assert_eq!(candy(ratings), 5);
    }

    #[test]
    fn test_uniform_ratings() {
        let ratings = vec![1, 1, 1];
        assert_eq!(candy(ratings), 3);
    }

    #[test]
    fn test_increasing_ratings() {
        let ratings = vec![1, 2, 3, 4, 5];
        assert_eq!(candy(ratings), 15);
    }

    #[test]
    fn test_decreasing_ratings() {
        let ratings = vec![5, 4, 3, 2, 1];
        assert_eq!(candy(ratings), 15);
    }

    #[test]
    fn test_peak_ratings() {
        let ratings = vec![1, 3, 2, 2, 1];
        assert_eq!(candy(ratings), 7);
    }

    #[test]
    fn test_single_child() {
        let ratings = vec![5];
        assert_eq!(candy(ratings), 1);
    }

    #[test]
    fn test_empty_ratings() {
        let ratings = vec![];
        assert_eq!(candy(ratings), 0);
    }
}
