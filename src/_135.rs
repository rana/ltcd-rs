// Time Complexity: O(n), where n is the length of the array.
// Space Complexity: O(n), where n is the length of the array.
// https://chatgpt.com/c/67043a22-6b18-8002-a68f-ca6e75900483
pub fn candy(ratings: Vec<i32>) -> i32 {
    // Given a line of children's ratings.
    // Conditions:
    //  * Each receives at least 1 candy.
    //  * Children with a higher rating than a neighbor
    //      receives more candy than neighbor.
    // Return the minimum number of candies.
    // Use a local optimization "greedy" algorithm.
    // Use a two-pass approach to address left neighbors 
    // and right neighbors.

    // Check minimum edge condition.
    let len = ratings.len();
    if len == 0 {
        return 0;
    }

    // Initialize variables.
    // Initialize each with 1 by challenge definition.
    let mut candies = vec![1; len];

    // Left-to-right pass.
    // Ensure that left-neighbor constaint met.
    // Use the neighbor's candy value.
    for n in 1..len {
        if ratings[n] > ratings[n - 1] {
            candies[n] = candies[n - 1] + 1;
        }
    }

    // Right-to-left pass.
    // Ensure the right-neighbor constraint met.
    // Use the neighbor's candy value.
    for n in (0..len - 1).rev() {
        if ratings[n] > ratings[n + 1] && candies[n] <= candies[n + 1] {
            candies[n] = candies[n + 1] + 1;
        }
    }

    // Sum candies to return the min total.
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
