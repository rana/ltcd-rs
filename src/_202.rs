// Time complexity: O(k), k is the number of cycles to reach 1, or detect a cycle.
// Space complexity: O(1), constant additional space used.
// https://chatgpt.com/c/67098089-a444-8002-b3ca-c43eee30898f
pub fn is_happy(n: i32) -> bool {
    // Happy number
    // Given an integer.
    // Determine whether the number meets a happy condition.
    // Return true if condition is met.
    // Happy condition:
    // * Transform positive integer to sum of the square of digits.
    // * Repeat until equal to one; or, infinite loop detected.
    // Detect infinite loop with slow and faster pointer.
    // Write an inner function for integer transform.
    fn sum_sqr(mut num: i32) -> i32 {
        let mut sum: i32 = 0;
        // Calculate sum of squares of digits.
        while num > 0 {
            let digit: i32 = num % 10;
            sum += digit * digit;
            num /= 10;
        }

        sum
    }

    // Initialize slow and fast pointers.
    let mut slo: i32 = n;
    let mut fst: i32 = sum_sqr(n);

    // Iterate until fast reaches 1, or slow and fast meet.
    while fst != 1 && slo != fst {
        slo = sum_sqr(slo);
        fst = sum_sqr(sum_sqr(fst));
    }

    // Return whether happy condition reached.
    fst == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hpy_true_19() {
        assert!(is_happy(19));
    }

    #[test]
    fn is_hpy_false_2() {
        assert!(!is_happy(2));
    }

    #[test]
    fn is_hpy_true_1() {
        assert!(is_happy(1));
    }

    #[test]
    fn is_hpy_false_4() {
        assert!(!is_happy(4));
    }
}
