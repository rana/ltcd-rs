/// 68. Text Justification
///
/// Given an array of strings words and a width maxWidth, format the
/// text such that each line has exactly maxWidth characters and is
/// fully (left and right) justified.
///
/// You should pack your words in a greedy approach; that is, pack
/// as many words as you can in each line. Pad extra spaces ' '
/// when necessary so that each line has exactly maxWidth characters.
///
/// Extra spaces between words should be distributed as evenly as
/// possible. If the number of spaces on a line does not divide
/// evenly between words, the empty slots on the left will be
/// assigned more spaces than the slots on the right.
///
/// For the last line of text, it should be left-justified,
/// and no extra space is inserted between words.
///
/// Note:
/// * A word is defined as a character sequence consisting of
/// non-space characters only.
/// * Each word's length is guaranteed to be greater than 0
/// and not exceed maxWidth.
/// * The input array words contains at least one word.
///
/// Constraints:
/// * 1 <= words.length <= 300
/// * 1 <= words[i].length <= 20
/// * words[i] consists of only English letters and symbols.
/// * 1 <= maxWidth <= 100
/// * words[i].length <= maxWidth

fn full_justify(mut wrds: Vec<String>, max_width: i32) -> Vec<String> {
    use std::cmp::Ordering;
    use std::iter::repeat;

    let mut ret: Vec<String> = Vec::new();

    // Measure words per line, and space per line.
    // `prv_chr_cnt` is the number of word characters without spaces.
    let lne_max = max_width as usize;
    let len = wrds.len();
    let mut idx: usize = 0;
    let mut prv_chr_cnt: usize = 0;
    let mut idx_fst: usize = 0;
    while idx < len {
        // `gap_cnt` is the number of single spaces between words.
        let wrd_len = wrds[idx].len();
        let gap_cnt = idx - idx_fst;
        let lne_cnt = wrd_len + prv_chr_cnt + gap_cnt;

        match lne_cnt.cmp(&lne_max) {
            Ordering::Equal => {
                // Line complete.
                // Exactly full justified.
                // Use current word.
                // Single spacing.
                // Build line.
                let mut lne = String::with_capacity(lne_max);
                lne.push_str(&wrds[idx_fst]);
                for i in idx_fst + 1..=idx {
                    lne.push(' ');
                    lne.push_str(&wrds[i]);
                }
                ret.push(lne);

                // Setup next line.
                idx += 1;
                idx_fst = idx + 1;
                prv_chr_cnt = 0;
            }
            Ordering::Greater => {
                // Line complete.
                // Do not use current word.
                // Irregular spacing.

                // Determine spacing.
                // Add spacing to existing words.
                let mut spc_cnt = lne_max - prv_chr_cnt;
                if idx - idx_fst == 1 {
                    wrds[idx_fst].extend(repeat(' ').take(spc_cnt));
                } else {
                    while spc_cnt != 0 {
                        for i in idx_fst..idx - 1 {
                            wrds[i].push(' ');
                            spc_cnt -= 1;
                            if spc_cnt == 0 {
                                break;
                            }
                        }
                    }
                }

                // Build line.
                let mut lne = String::with_capacity(lne_max);
                for i in idx_fst..idx {
                    lne.push_str(&wrds[i]);
                }
                ret.push(lne);

                // Setup next line.
                // Do not increment `idx`.
                idx_fst = idx;
                prv_chr_cnt = 0;
            }
            Ordering::Less => {
                // Check for last line.
                if idx == len - 1 {
                    // Build line.
                    let mut lne = String::with_capacity(lne_max);
                    lne.push_str(&wrds[idx_fst]);
                    for i in idx_fst + 1..=idx {
                        lne.push(' ');
                        lne.push_str(&wrds[i]);
                    }
                    lne.extend(repeat(' ').take(lne_max - lne.len()));
                    ret.push(lne);
                    break;
                }

                // Continue accumulating words for line.
                idx += 1;
                prv_chr_cnt += wrd_len;
            }
        }
    }

    ret
}

fn full_justify_b(words: Vec<String>, max_width: i32) -> Vec<String> {
    use std::iter::repeat;
    let mut result = Vec::new();
    let mut current_line: Vec<String> = Vec::new();
    let mut num_of_letters = 0;

    for word in words {
        if num_of_letters + word.len() + current_line.len() > max_width as usize {
            let mut line = String::new();
            let spaces = max_width as usize - num_of_letters;
            if current_line.len() == 1 {
                line.push_str(&current_line[0]);
                line.extend(repeat(' ').take(spaces));
            } else {
                let space_between_words = spaces / (current_line.len() - 1);
                let extra_spaces = spaces % (current_line.len() - 1);
                for i in 0..current_line.len() - 1 {
                    line.push_str(&current_line[i]);
                    let space_len = if i < extra_spaces {
                        space_between_words + 1
                    } else {
                        space_between_words
                    };
                    line.extend(repeat(' ').take(space_len));
                }
                line.push_str(current_line.last().unwrap());
            }
            result.push(line);
            current_line.clear();
            num_of_letters = 0;
        }
        current_line.push(word.clone());
        num_of_letters += word.len();
    }

    let mut last_line = current_line.join(" ");
    last_line.extend(repeat(' ').take((max_width as usize).saturating_sub(last_line.len())));
    result.push(last_line);

    result
}

fn full_justify_a(mut wrds: Vec<String>, max_width: i32) -> Vec<String> {
    use std::ops::Range;

    #[derive(Debug)]
    struct LneRng {
        /// `rng` is the index range of words in a line.
        rng: Range<u16>,
        /// `spc_cnt` is the number of spaces in a line.
        spc_cnt: u16,
    }

    let lne_chr_max = max_width as u16;
    let len_wrds = wrds.len() as u16;
    let mut lne_chr_cnt: u16 = 0;

    // Evaluate words per line.
    let mut lne_rngs: Vec<LneRng> = Vec::new();
    let mut idx_fst: u16 = 0;
    let mut idx: u16 = 0;
    while idx < len_wrds {
        let wrd_chr_cnt = wrds[idx as usize].len() as u16;
        let gap_cnt = idx - idx_fst;
        let min_chr_cnt = lne_chr_cnt + wrd_chr_cnt + gap_cnt;
        // println!(
        //     "idx:{idx} lne_chr_cnt:{lne_chr_cnt} wrd_chr_cnt:{wrd_chr_cnt} gap_cnt:{gap_cnt} min_chr_cnt:{min_chr_cnt}"
        // );

        // Check whether word extends beyond the line's end.
        if min_chr_cnt > lne_chr_max {
            // Line is complete.
            // Store line word range.
            lne_rngs.push(LneRng {
                rng: idx_fst..idx,
                spc_cnt: lne_chr_max - lne_chr_cnt,
            });
            idx_fst = idx;
            lne_chr_cnt = 0;
        } else {
            lne_chr_cnt += wrd_chr_cnt;
            idx += 1;
            if idx == len_wrds {
                lne_rngs.push(LneRng {
                    rng: idx_fst..idx,
                    spc_cnt: lne_chr_max - lne_chr_cnt,
                });
            }
        }
    }
    println!("{:?}", lne_rngs);

    // Create string lines with full justification.
    // Skip the last line, which is left-justified.
    // Append space to words which are not the last line.
    let mut lst_lne_rng = lne_rngs.pop().unwrap();
    for lne_rng in lne_rngs.iter_mut() {
        // println!("lne_rng:{:?}", lne_rng);
        // Don't append space to last word.
        let idx_fst = lne_rng.rng.start;
        let mut lim = lne_rng.rng.end;
        if lim - idx_fst > 1 {
            lim -= 1;
        }
        // Cycle through words multiple times as needed.
        while lne_rng.spc_cnt > 0 {
            'inr: for idx in idx_fst..lim {
                // println!("idx:{idx} spc_cnt:{}", lne_rng.spc_cnt);
                if lne_rng.spc_cnt == 0 {
                    break 'inr;
                }
                wrds[idx as usize].push(' ');
                lne_rng.spc_cnt -= 1;
            }
        }
    }

    // Append space to last line words.
    // Append one space to all but last word.
    let idx_fst = lst_lne_rng.rng.start;
    let lim = lst_lne_rng.rng.end - 1;
    for idx in idx_fst..lim {
        // println!("lst lne: idx:{idx} spc_cnt:{}", lst_lne_rng.spc_cnt);
        wrds[idx as usize].push(' ');
        lst_lne_rng.spc_cnt -= 1;
    }
    // Append remaining space to last word.
    while lst_lne_rng.spc_cnt > 0 {
        // println!("lst wrd: spc_cnt:{}", lst_lne_rng.spc_cnt);
        wrds[lim as usize].push(' ');
        lst_lne_rng.spc_cnt -= 1;
    }

    // Build each line.
    // Spacing already appended to all words.
    let mut lnes: Vec<String> = Vec::with_capacity(lne_rngs.len());
    for lne_rng in lne_rngs {
        let mut lne = String::with_capacity(lne_chr_max as usize);
        for idx in lne_rng.rng {
            lne.push_str(wrds[idx as usize].as_str());
        }
        lnes.push(lne);
    }
    // Last line
    let mut lne = String::with_capacity(lne_chr_max as usize);
    for idx in lst_lne_rng.rng {
        lne.push_str(wrds[idx as usize].as_str());
    }
    lnes.push(lne);

    lnes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_word() {
        let words = vec!["hello".to_string()];
        let max_width = 10;
        let expected = vec!["hello     ".to_string()];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_words_fit_exactly() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let max_width = 11;
        let expected = vec!["hello world".to_string()];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_words_fit_almost() {
        let words = vec!["hello".to_string(), "world".to_string()];
        let max_width = 10;
        let expected = vec!["hello     ".to_string(), "world     ".to_string()];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_multiple_lines() {
        let words = vec![
            "This".to_string(),
            "is".to_string(),
            "an".to_string(),
            "example".to_string(),
            "of".to_string(),
            "text".to_string(),
            "justification.".to_string(),
        ];
        let max_width = 16;
        let expected = vec![
            "This    is    an".to_string(),
            "example  of text".to_string(),
            "justification.  ".to_string(),
        ];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_last_line_left_justified() {
        let words = vec![
            "What".to_string(),
            "must".to_string(),
            "be".to_string(),
            "acknowledgment".to_string(),
            "shall".to_string(),
        ];
        let max_width = 16;
        let expected = vec![
            "What   must   be".to_string(),
            "acknowledgment  ".to_string(),
            "shall           ".to_string(),
        ];
        assert_eq!(full_justify(words, max_width), expected);
    }

    #[test]
    fn test_long() {
        // ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"]
        let words = vec![
            "Science".to_string(),
            "is".to_string(),
            "what".to_string(),
            "we".to_string(),
            "understand".to_string(),
            "well".to_string(),
            "enough".to_string(),
            "to".to_string(),
            "explain".to_string(),
            "to".to_string(),
            "a".to_string(),
            "computer.".to_string(),
            "Art".to_string(),
            "is".to_string(),
            "everything".to_string(),
            "else".to_string(),
            "we".to_string(),
            "do".to_string(),
        ];
        // ["Science  is  what we","understand      well","enough to explain to","a  computer.  Art is","everything  else  we","do                  "]
        let max_width = 20;
        let expected = vec![
            "Science  is  what we".to_string(),
            "understand      well".to_string(),
            "enough to explain to".to_string(),
            "a  computer.  Art is".to_string(),
            "everything  else  we".to_string(),
            "do                  ".to_string(),
        ];
        assert_eq!(full_justify(words, max_width), expected);
    }
}
