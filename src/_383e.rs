// Time complexity: O(n), n is the length of string note and magazine. We traverse each string once.
// Space complexity: O(1), constant additional space used. We create a fixed-size array with length 26.
pub fn can_construct(rnt: String, mgz: String) -> bool {
    // Ransom Note.
    // Given two string note and magazine.
    // Determine if magazine letters can construct note.
    // Each letter in magazine can be used once in note.
    // Return true if condition met.
    // Use a single frequency counter.
    // Use a fixed-size array for a map due to 26 letter character set.

    // Initialize a fixed-size frequency counter map.
    let mut cnts: [i32; 26] = [0; 26];

    // Iterate through each character of magazine.
    // Start with magazine as the reference letters.
    for chr in mgz.chars() {
        // Calculate map index for character.
        let idx: usize = (chr as u8 - b'a') as usize;
        // Increment counter for source characters (magazine).
        cnts[idx] += 1;
    }

    // Iterate through each character of note.
    // End with note as the success condition check.
    for chr in rnt.chars() {
        // Calculate map index for character.
        let idx: usize = (chr as u8 - b'a') as usize;
        // Decrement counter for target characters (note).
        cnts[idx] -= 1;
        // Check for failure condition.
        // All cnts expected to be zero for success.
        if cnts[idx] < 0 {
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
