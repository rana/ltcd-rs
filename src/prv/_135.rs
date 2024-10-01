/// 135. Candy
///
/// There are n children standing in a line. Each
/// child is assigned a rating value given in the
/// integer array ratings.
///
/// You are giving candies to these children subjected
/// to the following requirements:
///
/// * Each child must have at least one candy.
///
/// * Children with a higher rating get more candies
/// than their neighbors.
///
/// Return the minimum number of candies you need to
/// have to distribute the candies to the children.
///
/// Constraints:
/// * n == ratings.length
/// * 1 <= n <= 2 * 10^4
/// * 0 <= ratings[i] <= 2 * 10^4

fn candy_c(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    let mut candies = vec![1i32; len];

    // Left comparisons.
    for idx in 1..len {
        if ratings[idx - 1] < ratings[idx] {
            candies[idx] = candies[idx - 1] + 1;
        }
    }

    // Right comparisons.
    let mut sum = candies[len - 1];
    for idx in (0..len - 1).rev() {
        if ratings[idx] > ratings[idx + 1] {
            candies[idx] = candies[idx].max(candies[idx + 1] + 1);
        }
        sum += candies[idx];
    }

    sum
}

fn candy_b(ratings: Vec<i32>) -> i32 {
    // Variables contribute to O(1) space complexity.
    let len = ratings.len();
    let mut candies: Vec<i32> = vec![1; len];

    // Evaluate left-side comparisons.
    // Loop from left-to-right through `ratings` array.
    // Loop contributes to O(n) time complexity.
    for i in 1..len {
        if ratings[i - 1] < ratings[i] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    // Evaluate right-side comparisons.
    // Loop from right-to-left through `ratings` array.
    // Loop contributes to O(n) time complexity.
    let mut sum = candies[len - 1];
    for i in (0..len - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
        sum += candies[i];
    }

    // Return the sum of the candies array.
    sum
}

fn candy_a(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    let mut candies: Vec<i32> = vec![1; len];

    // Traverse left-to-right for left neighbor relation.
    for i in 1..len {
        if ratings[i - 1] < ratings[i] {
            candies[i] = candies[i - 1] + 1;
        }
    }

    // Traverse right-to-left for right neighbor relation.
    let mut sum = candies[len - 1];
    for i in (0..len - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
        sum += candies[i];
    }

    println!("candies:{:?}", candies);

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_candy_c() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = candy_c(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_candy_b() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = candy_b(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    #[test]
    fn tst_candy_a() {
        for (idx, t) in tsts().iter().enumerate() {
            let act = candy_a(t.nums.clone());
            assert_eq!(act, t.ret, "idx:{} {:?}", idx, t);
        }
    }

    fn tsts() -> Vec<Tst> {
        vec![
            Tst {
                nums: vec![1, 0, 2],
                ret: 5,
            },
            Tst {
                nums: vec![1, 2, 2],
                ret: 4,
            },
        ]
    }

    #[derive(Clone, Debug)]
    struct Tst {
        nums: Vec<i32>,
        ret: i32,
    }
}
