use std::collections::HashSet;

use crate::dictionary::{LetterNext, LetterVec};

pub fn find_words(letters: &str, dictionary: &Vec<LetterVec>, min_len: u8) -> Vec<String> {
    let mut result = HashSet::new();

    let letter_chars = letters.chars().collect();
    let mut is_chosen = (0..letters.len()).map(|_| false).collect::<Vec<bool>>();

    find_words_rec(
        &mut vec![],
        0,
        &letter_chars,
        &mut is_chosen,
        dictionary,
        min_len,
        &mut result,
    );

    result.into_iter().collect()
}

fn find_words_rec(
    chosen: &mut Vec<char>,
    elem: usize,
    letters: &Vec<char>,
    is_chosen: &mut Vec<bool>,
    dictionary: &Vec<LetterVec>,
    min_len: u8,
    result: &mut HashSet<String>,
) {
    let found_word = |chosen: &Vec<char>, result: &mut HashSet<String>| {
        if chosen.len() >= min_len as usize {
            let s = chosen.iter().collect::<String>();
            result.insert(s);
        }
    };

    for i in 0..letters.len() {
        if is_chosen[i] {
            continue
        }

        let chosen_char = letters[i];

        chosen.push(chosen_char);

        let new_elem = match dictionary[elem][uchar_to_elem(chosen_char)] {
            LetterNext::None => None,
            LetterNext::Next(e) => Some(e as usize),
            LetterNext::End => {
                found_word(chosen, result);
                None
            }
            LetterNext::EndNext(e) => {
                found_word(chosen, result);
                Some(e as usize)
            }
        };

        match new_elem {
            None => (),
            Some(e) => {
                is_chosen[i] = true;
                find_words_rec(chosen, e, letters, is_chosen, dictionary, min_len, result);
                is_chosen[i] = false;
            }
        }

        chosen.pop();
    }
}

#[inline]
fn uchar_to_elem(c: char) -> usize {
    (c as u8 - b'A') as usize
}
