// Time complexity: O(m + n), m is the length of magazine. n is the length of note.
// Space complexity: O(1) constant additional space used.
// https://chatgpt.com/c/67095482-ebc4-8002-a97a-f0b1f1a13ee2
pub fn can_construct(rnt: String, mgz: String) -> bool {
    // Given two strings.
    // Return true if note can be constructed using
    // characters from magazine.
    // Use character frequency counting.
    // The not contains only lowercase letters.

    // Initialize a map array.
    // We can use char as bytes and a 16 length array.
    let mut cnt: [i32; 26] = [0; 26];

    // Count magazine characters.
    // Iterate through each magazine character.
    for ch in mgz.chars() {
        let idx: usize = (ch as u8 - b'a') as usize;
        cnt[idx] += 1;
    }

    // Check characters in the ransom note.
    // Iterate through each note character.
    for ch in rnt.chars() {
        let idx: usize = (ch as u8 - b'a') as usize;
        cnt[idx] -= 1;
        // Count below zero means condition not met.
        if cnt[idx] < 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_simple() {
        let rnt = String::from("a");
        let mgz = String::from("b");
        assert!(!can_construct(rnt, mgz));
    }

    #[test]
    fn valid_exact() {
        let rnt = String::from("aa");
        let mgz = String::from("aab");
        assert!(can_construct(rnt, mgz));
    }

    #[test]
    fn valid_insufficient() {
        let rnt = String::from("aa");
        let mgz = String::from("ab");
        assert!(!can_construct(rnt, mgz));
    }

    #[test]
    fn valid_empty_note() {
        let rnt = String::from("");
        let mgz = String::from("any");
        assert!(can_construct(rnt, mgz));
    }

    #[test]
    fn valid_empty_magazine() {
        let rnt = String::from("a");
        let mgz = String::from("");
        assert!(!can_construct(rnt, mgz));
    }
}
