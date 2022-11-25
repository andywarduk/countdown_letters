use std::fs::{read_link, symlink_metadata, File};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;

use flate2::read::GzDecoder;

use crate::numformat::NumFormat;

pub struct Dictionary {
    words: usize,
    tree: Vec<LetterVec>,
}

impl Dictionary {
    pub fn word_count(&self) -> usize {
        self.words
    }

    pub fn len(&self) -> usize {
        self.tree.len()
    }

    pub fn mem_usage(&self) -> usize {
        self.len() * std::mem::size_of::<LetterNext>()
    }

    #[inline]
    pub fn lookup_elem_letter(&self, elem: usize, letter: u8) -> LetterNext {
        self.tree[elem][letter as usize]
    }
}

type LetterVec = [LetterNext; 26];

#[derive(Copy, Clone, Debug)]
pub enum LetterNext {
    None,
    Next(u32),
    End,
    EndNext(u32),
}

pub fn load_words_from_file(file: &str, max_len: usize, verbose: bool) -> io::Result<Dictionary> {
    let path_buf = PathBuf::from(file);

    if verbose {
        println!("Loading words from {}", file_spec(&path_buf)?);
    }

    // Open word file
    let mut word_file = File::open(&path_buf)?;

    // Read the first two bytes
    let mut hdr = [0u8; 2];
    let hdr_read = word_file.read(&mut hdr)?;
    word_file.seek(SeekFrom::Start(0))?;

    // Check for gzip signature
    let bufreader: Box<dyn BufRead> = if hdr_read == 2 && hdr[0] == 0x1f && hdr[1] == 0x8b {
        // gzip compressed file
        Box::new(BufReader::new(GzDecoder::new(word_file)))
    } else {
        // Create buf reader for the file
        Box::new(BufReader::new(word_file))
    };

    load_words_from_bufread(bufreader, max_len, verbose)
}

pub fn load_words_from_bufread(
    bufread: Box<dyn BufRead>,
    max_len: usize,
    verbose: bool,
) -> io::Result<Dictionary> {
    let mut tree = Vec::new();

    let empty = [LetterNext::None; 26];

    let mut lines: usize = 0;
    let mut words: usize = 0;
    let mut too_short: usize = 0;
    let mut wrong_case: usize = 0;

    tree.push(empty);

    // Iterate file lines
    for line in bufread.lines() {
        let line = line?;

        lines += 1;

        // Check length
        let length = line.len();

        if length < 2 || length > max_len {
            too_short += 1;
            continue;
        }

        // Make sure word consists of all lower case ascii characters
        if !is_ascii_lower(&line) {
            wrong_case += 1;
            continue;
        }

        // Add this word to the tree
        words += 1;

        let mut cur_elem = 0;

        for (i, c) in line.chars().enumerate() {
            let letter = lchar_to_elem(c);

            if i == length - 1 {
                // Last character
                tree[cur_elem][letter] = match tree[cur_elem][letter] {
                    LetterNext::None => LetterNext::End,
                    LetterNext::Next(n) => LetterNext::EndNext(n),
                    _ => panic!("Duplicate word {}", line),
                }
            } else {
                // Mid character
                cur_elem = match tree[cur_elem][letter] {
                    LetterNext::None => {
                        tree.push(empty);
                        let e = tree.len() - 1;
                        tree[cur_elem][letter] = LetterNext::Next(e as u32);
                        e
                    }
                    LetterNext::End => {
                        tree.push(empty);
                        let e = tree.len() - 1;
                        tree[cur_elem][letter] = LetterNext::EndNext(e as u32);
                        e
                    }
                    LetterNext::Next(e) | LetterNext::EndNext(e) => e as usize,
                };
            }
        }
    }

    let dictionary = Dictionary { words, tree };

    if verbose {
        println!(
            "{} total lines, ({} too short, {} not all lower case)",
            lines.num_format(),
            too_short.num_format(),
            wrong_case.num_format()
        );

        println!(
            "Dictionary words {}, size {} ({} bytes)",
            dictionary.word_count().num_format(),
            dictionary.len().num_format(),
            dictionary.mem_usage().num_format(),
        );
    }

    Ok(dictionary)
}

fn file_spec(path: &PathBuf) -> io::Result<String> {
    let meta = symlink_metadata(path)?;

    if meta.is_symlink() {
        let target = read_link(path)?;

        Ok(format!(
            "{} -> {}",
            path.to_string_lossy(),
            file_spec(&target)?
        ))
    } else {
        Ok(format!("{}", path.to_string_lossy()))
    }
}

#[inline]
fn lchar_to_elem(c: char) -> usize {
    (c as u8 - b'a') as usize
}

#[inline]
fn is_ascii_lower(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_lowercase())
}
