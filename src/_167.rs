pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    use std::cmp::Ordering;

    // Given an ascending sorted array of integers.
    // Find two numbers adding to target number.
    // Return the two indexes in an array with 1-based indexing.
    // Use a two-pointer technique.

    // Initialize variables.
    let mut lft: usize = 0;
    let mut rht: usize = numbers.len() - 1;

    // Loop until the two pointers meet.
    while lft < rht {
        // Calculate a sum.
        let sum = numbers[lft] + numbers[rht];

        match sum.cmp(&target) {
            // Found two numbers.
            Ordering::Equal => return vec![lft as i32 + 1, rht as i32 + 1],
            // Increase the next sum.
            Ordering::Less => lft += 1,
            // Decrease the next sum.
            Ordering::Greater => rht -= 1,
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }

    #[test]
    fn example_case2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        assert_eq!(two_sum(numbers, target), vec![1, 3]);
    }

    #[test]
    fn example_case3() {
        let numbers = vec![-1, 0];
        let target = -1;
        assert_eq!(two_sum(numbers, target), vec![1, 2]);
    }

    #[test]
    fn invalid_case_no_solution() {
        let numbers = vec![1, 2, 3];
        let target = 7;
        assert_eq!(two_sum(numbers, target), vec![]);
    }
}
