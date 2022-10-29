use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

#[derive(Copy, Clone, Debug)]
pub enum LetterNext {
    None,
    Next(u32),
    End,
    EndNext(u32),
}

pub type LetterVec = [LetterNext; 26];

pub fn load_words_from_file(file: &str, max_len: usize) -> io::Result<Vec<LetterVec>> {
    // Open word file
    let word_file = File::open(file)?;

    // Create buf reader for the file
    let bufreader = BufReader::new(word_file);

    load_words_from_bufreader(bufreader, max_len)
}

pub fn load_words_from_bufreader<R>(
    bufreader: BufReader<R>,
    max_len: usize,
) -> io::Result<Vec<LetterVec>>
where
    R: Read,
{
    let mut result = Vec::new();

    let empty = [LetterNext::None; 26];

    result.push(empty);

    // Iterate file lines
    for line in bufreader.lines() {
        let line = line?;

        // Check length
        let length = line.len();

        if length < 2 || length > max_len {
            continue;
        }

        // Make sure word consists of all lower case ascii characters
        if !is_ascii_lower(&line) {
            continue;
        }

        let mut cur_elem = 0;
        for (i, c) in line.chars().enumerate() {
            let letter = lchar_to_elem(c);

            if i == length - 1 {
                // Last character
                result[cur_elem][letter] = match result[cur_elem][letter] {
                    LetterNext::None => LetterNext::End,
                    LetterNext::Next(n) => LetterNext::EndNext(n),
                    _ => panic!("Duplicate word {}", line),
                }
            } else {
                // Mid character
                cur_elem = match result[cur_elem][letter] {
                    LetterNext::None => {
                        result.push(empty);
                        let e = result.len() - 1;
                        result[cur_elem][letter] = LetterNext::Next(e as u32);
                        e
                    }
                    LetterNext::End => {
                        result.push(empty);
                        let e = result.len() - 1;
                        result[cur_elem][letter] = LetterNext::EndNext(e as u32);
                        e
                    }
                    LetterNext::Next(e) | LetterNext::EndNext(e) => e as usize,
                };
            }
        }
    }

    Ok(result)
}

#[inline]
fn lchar_to_elem(c: char) -> usize {
    (c as u8 - b'a') as usize
}

#[inline]
fn is_ascii_lower(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}
