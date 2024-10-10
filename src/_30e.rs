pub fn find_substring(s: String, wrds: Vec<String>) -> Vec<i32> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let expected = vec![0, 9];
        let mut result = find_substring(s, words);
        result.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn example2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec![
            "word".to_string(),
            "good".to_string(),
            "best".to_string(),
            "word".to_string(),
        ];
        let expected: Vec<i32> = vec![];
        let result = find_substring(s, words);
        assert_eq!(result, expected);
    }

    #[test]
    fn example3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let expected = vec![6, 9, 12];
        let mut result = find_substring(s, words);
        result.sort();
        assert_eq!(result, expected);
    }
}
