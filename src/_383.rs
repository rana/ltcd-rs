/// 383. Ransom Note
///
/// Given two strings ransomNote and magazine, return true if
/// ransomNote can be constructed by using the letters from
/// magazine and false otherwise.
///
/// Each letter in magazine can only be used once in ransomNote.
///
/// Constraints:
/// * 1 <= ransomNote.length, magazine.length <= 10^5
/// * ransomNote and magazine consist of lowercase English letters.

fn can_construct(ransom_note: &str, magazine: &str) -> bool {
    // Count character frequencies in each string.
    // Compare character frequencies.
    // Return false if magazine has an individual character
    // frequency less than the ransom note.
    // Use an array as a map with a perfect hash.
    // Time complexity: O(n + m)
    //  - n is the length of string ransom_note.
    //  - m is the length of string magazine.
    // Space complexity: O(1)
    //  - Constant space due to use of only 26 lowercase
    //  English letters.
    const CHR_LEN: usize = 26;
    const CHR_FST: u8 = b'a';

    // Count character frequency in ransom note.
    let mut ran_cnts: [u32; CHR_LEN] = [0; CHR_LEN];
    for &byt_chr in ransom_note.as_bytes() {
        ran_cnts[(byt_chr - CHR_FST) as usize] += 1;
    }

    // Count character frequency in magazine.
    let mut mag_cnts: [u32; CHR_LEN] = [0; CHR_LEN];
    for &byt_chr in magazine.as_bytes() {
        mag_cnts[(byt_chr - CHR_FST) as usize] += 1;
    }

    // Compare character frequencies.
    for idx in 0..CHR_LEN {
        if mag_cnts[idx] < ran_cnts[idx] {
            return false;
        }
    }

    true
}

fn can_construct_c(ransom_note: &str, magazine: &str) -> bool {
    // 26 English letters in alphabet.
    // Can create a perfect hash for map.
    const CHR_LEN: usize = 26;
    const CHR_FST: u8 = b'a';

    // Count characters from ransom note.
    let mut ran_cnts: [u32; CHR_LEN] = [0; CHR_LEN];
    for &byt_chr in ransom_note.as_bytes().iter() {
        ran_cnts[(byt_chr - CHR_FST) as usize] += 1;
    }

    // Count characters from magazine.
    let mut mag_cnts: [u32; CHR_LEN] = [0; CHR_LEN];
    for &byt_chr in magazine.as_bytes().iter() {
        mag_cnts[(byt_chr - CHR_FST) as usize] += 1;
    }

    // Compare character frequencies.
    for idx in 0..CHR_LEN {
        if mag_cnts[idx] < ran_cnts[idx] {
            return false;
        }
    }

    true
}

fn can_construct_b(ransom_note: &str, magazine: &str) -> bool {
    use std::collections::HashMap;

    // Count ransom note characters.
    let mut ran_cnts = HashMap::new();
    for chr in ransom_note.chars() {
        *ran_cnts.entry(chr).or_insert(0) += 1;
    }

    // Count magazine characters.
    let mut mag_cnts = HashMap::new();
    for chr in magazine.chars() {
        *mag_cnts.entry(chr).or_insert(0) += 1;
    }

    // Compare character frequencies.
    for (chr, cnt) in ran_cnts.iter() {
        if mag_cnts.get(chr).unwrap_or(&0) < cnt {
            return false;
        }
    }

    true
}

fn can_construct_a(ransom_note: &str, magazine: &str) -> bool {
    use std::collections::HashMap;

    // Count the frequency of each letter in the ransom note.
    let mut ransom_note_counts = HashMap::new();
    for c in ransom_note.chars() {
        *ransom_note_counts.entry(c).or_insert(0) += 1;
    }

    // Count the frequency of each letter in the magazine.
    let mut magazine_counts = HashMap::new();
    for c in magazine.chars() {
        *magazine_counts.entry(c).or_insert(0) += 1;
    }

    // Check if the magazine contains enough letters to construct the ransom note.
    for (letter, &count) in ransom_note_counts.iter() {
        if magazine_counts.get(letter).unwrap_or(&0) < &count {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::can_construct;

    #[test]
    fn test_common_cases() {
        assert_eq!(can_construct("a", "b"), false);
        assert_eq!(can_construct("aa", "ab"), false);
        assert_eq!(can_construct("aa", "aab"), true);
    }

    #[test]
    fn test_edge_cases() {
        // Empty ransom note
        assert_eq!(can_construct("", "aab"), true);

        // Empty magazine
        assert_eq!(can_construct("a", ""), false);

        // Ransom note and magazine with single matching letter
        assert_eq!(can_construct("a", "a"), true);

        // Long ransom note requiring many letters from a magazine
        let long_ransom_note = "a".repeat(100000);
        let long_magazine = "a".repeat(100001);
        assert_eq!(can_construct(&long_ransom_note, &long_magazine), true);

        let long_ransom_note2 = "a".repeat(100000);
        let long_magazine2 = "a".repeat(99999);
        assert_eq!(can_construct(&long_ransom_note2, &long_magazine2), false);
    }
}
