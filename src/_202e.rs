pub fn is_happy(n: i32) -> bool {
    // Happy Number
    // Given an integer n.
    // Reduce integer n:
    //  * Start with a positive integer.
    //  * Split the number into single digits.
    //  * Square the digits.
    //  * Sum the square.
    //  * Repeat until number is zero.
    // Determine whether n equals 1; or, infinite loop.
    // Return true if n equals 1.
    // Use an inner function to reduce.
    // Use Floyd's cycle-detecting algorithm.

    // Reduce n according to criteria.
    fn reduce(mut num: i32) -> i32 {
        let mut sum: i32 = 0;

        while num > 0 {
            let digit: i32 = num % 10;
            sum += digit * digit;
            num /= 10;
        }

        sum
    }

    // Check for infinite loop.
    let mut slo: i32 = n;
    let mut fst: i32 = reduce(n);
    while fst != 1 && slo != fst {
        slo = reduce(slo);
        fst = reduce(reduce(fst));
    }

    // Determine whether success condition met.
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
