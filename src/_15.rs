// https://chatgpt.com/c/6705c47b-79c4-8002-af71-9e73c411f45a
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // TODO:
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_triplets() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn no_triplets() {
        assert_eq!(three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn all_zeros() {
        assert_eq!(three_sum(vec![0, 0, 0, 0]), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn empty_input() {
        assert_eq!(three_sum(Vec::new()), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn single_element() {
        assert_eq!(three_sum(vec![1]), Vec::<Vec<i32>>::new());
    }
}
