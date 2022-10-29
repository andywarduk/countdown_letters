use std::collections::HashSet;

use crate::dictionary::{LetterNext, LetterVec};

pub fn find_words(letters: &str, dictionary: &Vec<LetterVec>, min_len: u8) -> Vec<String> {
    let mut result = HashSet::new();

    // Dictionary entry element numbers for each letter
    let letter_elems = letters.chars().map(|c| c as u8 - b'A').collect::<Vec<u8>>();

    // Vector of chosen letter elements
    let mut chosen = Vec::with_capacity(letters.len());

    // Been chosen indicators
    let mut is_chosen = vec![false; letters.len()];

    // Start search recursion
    find_words_rec(
        &mut chosen,
        &mut is_chosen,
        &letter_elems,
        dictionary,
        0,
        min_len,
        &mut result,
    );

    // Convert hash set to vector
    result.into_iter().collect()
}

fn find_words_rec(
    chosen: &mut Vec<u8>,
    is_chosen: &mut Vec<bool>,
    letter_elems: &[u8],
    dictionary: &Vec<LetterVec>,
    dict_elem: usize,
    min_len: u8,
    result: &mut HashSet<String>,
) {
    for i in 0..letter_elems.len() {
        if is_chosen[i] {
            // This letter has already been chosen
            continue;
        }

        // Get chosen letter element
        let chosen_letter = letter_elems[i];

        // Add it to the chosen list
        chosen.push(chosen_letter);

        // Walk the dictionary
        let dict_elem = dictionary[dict_elem][chosen_letter as usize];

        // End of a word?
        match dict_elem {
            LetterNext::End | LetterNext::EndNext(_) => {
                if chosen.len() >= min_len as usize {
                    // SAFETY: Guaranteed to be upper case ASCII characters only
                    let string = chosen
                        .iter()
                        .map(|e| (*e + b'A') as char)
                        .collect::<String>();
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
                    chosen,
                    is_chosen,
                    letter_elems,
                    dictionary,
                    e as usize,
                    min_len,
                    result,
                );
                
                is_chosen[i] = false;
            }
            _ => (),
        }

        // SAFETY: length always decreasing and always removing the pushed entry above
        unsafe {
            chosen.set_len(chosen.len() - 1);
        }
    }
}
