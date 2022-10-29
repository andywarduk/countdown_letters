use std::collections::HashSet;

use crate::dictionary::{LetterNext, LetterVec};

pub fn find_words(letters: &str, dictionary: &Vec<LetterVec>, min_len: u8) -> Vec<String> {
    let mut result = HashSet::new();

    let letter_chars = letters.as_bytes();
    let mut is_chosen = vec![false; letters.len()];

    find_words_rec(
        &mut Vec::with_capacity(letters.len()),
        0,
        letter_chars,
        &mut is_chosen,
        dictionary,
        min_len,
        &mut result,
    );

    result.into_iter().collect()
}

fn find_words_rec(
    chosen: &mut Vec<u8>,
    elem: usize,
    letters: &[u8],
    is_chosen: &mut Vec<bool>,
    dictionary: &Vec<LetterVec>,
    min_len: u8,
    result: &mut HashSet<String>,
) {
    for i in 0..letters.len() {
        if is_chosen[i] {
            // This letter has already been chosen
            continue;
        }

        // Get chosen character
        let chosen_char = letters[i];

        // Add it to the chosen list
        chosen.push(chosen_char);

        // Walk the dictionary
        let dict_elem = dictionary[elem][uchar_to_elem(chosen_char)];

        // End of a word?
        match dict_elem {
            LetterNext::End | LetterNext::EndNext(_) => {
                if chosen.len() >= min_len as usize {
                    // SAFETY: Guaranteed to be upper case ASCII characters only
                    let string = unsafe { String::from_utf8_unchecked(chosen.to_vec()) };
                    result.insert(string);
                }
            }
            _ => (),
        }

        // Recurse to next letter
        match dict_elem {
            LetterNext::Next(e) | LetterNext::EndNext(e) => {
                is_chosen[i] = true;
                find_words_rec(
                    chosen, e as usize, letters, is_chosen, dictionary, min_len, result,
                );
                is_chosen[i] = false;
            }
            _ => (),
        }

        unsafe {
            chosen.set_len(chosen.len() - 1);
        }
    }
}

#[inline]
fn uchar_to_elem(c: u8) -> usize {
    (c as u8 - b'A') as usize
}
