use std::collections::HashSet;

use crate::dictionary::{LetterNext, LetterVec};

pub fn find_words(letters: &str, dictionary: &Vec<LetterVec>, min_len: u8) -> Vec<String> {
    let mut result = HashSet::new();

    find_words_rec(
        vec![],
        0,
        letters.chars().collect(),
        dictionary,
        min_len,
        &mut result,
    );

    result.into_iter().collect()
}

fn find_words_rec(
    chosen: Vec<char>,
    elem: usize,
    left: Vec<char>,
    dictionary: &Vec<LetterVec>,
    min_len: u8,
    result: &mut HashSet<String>,
) {
    let found_word = |chosen: &Vec<char>, letter, result: &mut HashSet<String>| {
        let s = format!("{}{}", chosen.iter().collect::<String>(), letter);

        if s.len() >= min_len as usize {
            result.insert(s);
        }
    };

    for i in 0..left.len() {
        let chosen_char = left[i];

        let new_elem = match dictionary[elem][uchar_to_elem(chosen_char)] {
            LetterNext::None => None,
            LetterNext::Next(e) => Some(e as usize),
            LetterNext::End => {
                found_word(&chosen, chosen_char, result);
                None
            }
            LetterNext::EndNext(e) => {
                found_word(&chosen, chosen_char, result);
                Some(e as usize)
            }
        };

        match new_elem {
            None => (),
            Some(e) => {
                let mut new_left = left.clone();
                new_left.remove(i);
                let mut new_chosen = chosen.clone();
                new_chosen.push(chosen_char);

                find_words_rec(new_chosen, e, new_left, dictionary, min_len, result);
            }
        }
    }
}

#[inline]
fn uchar_to_elem(c: char) -> usize {
    (c as u8 - b'A') as usize
}
