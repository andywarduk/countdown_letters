mod dictionary;
mod numformat;
mod results;
mod solver;

use std::io;
use std::path::Path;

use clap::Parser;

use crate::dictionary::load_words_from_file;
use crate::results::print_results;
use crate::solver::{find_words, SolverArgs};

/// Countdown letters game solver
#[derive(Parser, Default)]
#[clap(author, version, about)]
struct Args {
    /// Letters to use
    #[clap(value_parser = validate_letters)]
    letters: String,

    /// Word list file
    #[clap(
        short = 'd',
        long = "dictionary",
        default_value_t = default_dict(),
    )]
    dictionary_file: String,

    /// Minimum word length to find
    #[clap(short = 'm', long = "min-len", default_value_t = 3)]
    min_len: u8,

    /// Allow letters to be used more than once
    #[clap(short = 'r', long = "reuse")]
    reuse_letters: bool,

    /// Verbose output
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    /// Debug output
    #[clap(long = "debug")]
    debug: bool,
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    if args.dictionary_file.is_empty() {
        eprintln!("No dictionary file given and none of the default dictionaries could be found.");
    } else {
        // Print details
        if args.verbose {
            println!(
                "{} letters: {}",
                args.letters.len(),
                args.letters
                    .chars()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }

        // Load words
        let dictionary =
            load_words_from_file(&args.dictionary_file, args.letters.len(), args.verbose)?;

        // Find words
        let words = find_words(SolverArgs {
            letters: &args.letters,
            dictionary: &dictionary,
            min_len: args.min_len,
            reuse_letters: args.reuse_letters,
            debug: args.debug,
        });

        // Print results
        print_results(words);
    }

    Ok(())
}

fn validate_letters(s: &str) -> Result<String, String> {
    // Check minimum length
    if s.len() < 2 {
        Err("At least 2 letters must be provided")?;
    }

    // Convert all letters to upper case
    let ustring = s
        .chars()
        .map(|c| c.to_ascii_uppercase())
        .collect::<String>();

    // Check we only have upper case ascii characters
    if !ustring.chars().all(|c| c.is_ascii_uppercase()) {
        Err("Letters must be A-Z only".to_string())?;
    }

    Ok(ustring)
}

fn default_dict() -> String {
    dict_valid("words.txt.gz").unwrap_or_else(|| {
        dict_valid("/etc/dictionaries-common/words").unwrap_or_else(|| "".into())
    })
}

fn dict_valid(dict: &str) -> Option<String> {
    if Path::new(dict).is_file() {
        Some(dict.into())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use dictionary::{load_words_from_bufread, LetterNext};
    use io::BufReader;

    use super::*;

    #[test]
    fn size_checks() {
        assert_eq!(8, std::mem::size_of::<LetterNext>());
    }

    #[test]
    fn rust() {
        // Create dictionary with one word in it "rust"
        let bufreader = BufReader::new("rust".as_bytes());
        let dictionary = load_words_from_bufread(Box::new(bufreader), 4, false).unwrap();

        // Find words
        let words = find_words(SolverArgs {
            letters: "TRUS",
            dictionary: &dictionary,
            min_len: 1,
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
        let bufreader = BufReader::new(dict.as_bytes());
        let dictionary = load_words_from_bufread(Box::new(bufreader), 5, false).unwrap();

        // Find words
        let mut words = find_words(SolverArgs {
            letters: "TRUS",
            dictionary: &dictionary,
            min_len: 1,
            reuse_letters: false,
            debug: true,
        });

        words.sort();

        assert_eq!(words, vec!["RUST", "RUT", "RUTS"]);
    }
}
