/// 202. Happy Number
///
/// Write an algorithm to determine if a number n is happy.
///
/// A happy number is a number defined by the following process:
///
/// * Starting with any positive integer,
/// replace the number by the sum of the squares of its digits.
///
/// * Repeat the process until the number equals 1
/// (where it will stay), or it loops endlessly in a cycle
/// which does not include 1.
///
/// * Those numbers for which this process ends in 1 are happy.
///
/// Return true if n is a happy number, and false if not.
///
/// Constraints:
/// * 1 <= n <= 2^31 - 1

fn is_happy(n: i32) -> bool {
    // Calculate the next number with the given rules.
    fn nxt_num(mut n: i32) -> i32 {
        let mut sum: i32 = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }

    // Use Floyd's cycle finding to determine a solution.
    let mut slow = n;
    let mut fast = n;
    loop {
        slow = nxt_num(slow);
        fast = nxt_num(nxt_num(fast));
        if fast == 1 {
            return true;
        }
        if slow == fast {
            return false;
        }
    }
}

fn is_happy_a(n: i32) -> bool {
    fn next_number(n: i32) -> i32 {
        let mut n = n;
        let mut sum = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }

    let mut slow = n;
    let mut fast = n;
    loop {
        slow = next_number(slow);
        fast = next_number(next_number(fast));
        if fast == 1 {
            return true;
        }
        if slow == fast {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_number() {
        assert_eq!(is_happy(19), true);
    }

    #[test]
    fn test_not_happy_number() {
        assert_eq!(is_happy(2), false);
    }

    #[test]
    fn test_small_number() {
        assert_eq!(is_happy(1), true);
    }

    #[test]
    fn test_large_number() {
        assert_eq!(is_happy(9999999), false);
    }
}
