#![warn(missing_docs)]

//! Countdown letters game solver

use std::collections::HashSet;

use dictionary::{Dictionary, LetterNext};

/// Arguments for the countdown letters solver
pub struct SolverArgs<'a> {
    /// String of letters to use (must be upper case A-Z)
    pub letters: &'a str,
    /// Dictionary to use
    pub dictionary: &'a Dictionary,
    /// Letters can be reused flag
    pub reuse_letters: bool,
    /// Debug output
    pub debug: bool,
}

/// Find words in the provides dictionary using the provided letters
pub fn find_words(args: SolverArgs) -> Vec<String> {
    let mut result = HashSet::new();

    // Dictionary entry element numbers for each letter
    let mut letter_elems = args
        .letters
        .chars()
        .map(|c| c as u8 - b'A')
        .collect::<Vec<u8>>();

    if args.reuse_letters {
        // Remove duplicate entries if allowed to reuse letters
        letter_elems.sort();
        letter_elems.dedup();
    }

    // Vector of chosen letter elements
    let mut chosen = Vec::with_capacity(letter_elems.len());

    // Been chosen indicators
    let mut is_chosen = vec![false; letter_elems.len()];

    // Start search recursion
    find_words_rec(
        &args,
        &mut chosen,
        &mut is_chosen,
        &letter_elems,
        0,
        &mut result,
    );

    // Convert hash set to vector
    result.into_iter().collect()
}

fn find_words_rec(
    args: &SolverArgs,
    chosen: &mut Vec<u8>,
    is_chosen: &mut Vec<bool>,
    letter_elems: &[u8],
    dict_elem: usize,
    result: &mut HashSet<String>,
) {
    for i in 0..letter_elems.len() {
        if is_chosen[i] && !args.reuse_letters {
            // This letter has already been chosen
            continue;
        }

        // Get chosen letter element
        let chosen_letter = letter_elems[i];

        // Add it to the chosen list
        chosen.push(chosen_letter);

        // Walk the dictionary
        let dict_elem = args
            .dictionary
            .lookup_elem_letter_num(dict_elem, chosen_letter);

        if args.debug {
            debug_lookup(chosen, &dict_elem);
        }

        // End of a word?
        match dict_elem {
            LetterNext::End | LetterNext::EndNext(_) => {
                result.insert(chosen_string(chosen));
            }
            _ => (),
        }

        // Recurse to next letter
        match dict_elem {
            LetterNext::Next(e) | LetterNext::EndNext(e) => {
                is_chosen[i] = true;

                find_words_rec(args, chosen, is_chosen, letter_elems, e as usize, result);

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

#[inline]
fn chosen_string(chosen: &[u8]) -> String {
    chosen
        .iter()
        .map(|e| (*e + b'A') as char)
        .collect::<String>()
}

#[cold]
fn debug_lookup(chosen: &[u8], dict_elem: &LetterNext) {
    let string = chosen_string(chosen);
    let indent = string.len() - 1;

    println!("{:indent$}{} ({:?})", "", string, dict_elem);
}

#[cfg(test)]
mod tests {
    use dictionary::{Dictionary, LetterNext};

    use super::*;

    #[test]
    fn size_checks() {
        assert_eq!(8, std::mem::size_of::<LetterNext>());
    }

    #[test]
    fn rust() {
        // Create dictionary with one word in it "rust"
        let dictionary = Dictionary::new_from_string("rust", Default::default(), false).unwrap();

        // Find words
        let words = find_words(SolverArgs {
            letters: "TRUS",
            dictionary: &dictionary,
            reuse_letters: false,
            debug: true,
        });

        // Should be one found
        assert_eq!(words, vec!["RUST"]);
    }

    #[test]
    fn rusty() {
        // Create dictionary with some rusty words in it
        let dict = "\
            aaa\n\
            rut\n\
            ruts\n\
            rust\n\
            rusty\n\
            xxx\n\
            ";
        let dictionary = Dictionary::new_from_string(dict, Default::default(), false).unwrap();

        // Find words
        let mut words = find_words(SolverArgs {
            letters: "TRUS",
            dictionary: &dictionary,
            reuse_letters: false,
            debug: true,
        });

        words.sort();

        assert_eq!(words, vec!["RUST", "RUT", "RUTS"]);
    }
}
