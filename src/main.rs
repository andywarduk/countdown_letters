mod dictionary;
mod results;
mod solver;

use std::io;

use clap::Parser;

use dictionary::load_words_from_file;
use results::print_results;
use solver::find_words;

/// Countdown letters game solver
#[derive(Parser, Default)]
#[clap(author, version, about)]
struct Args {
    /// Letters to use
    #[clap(value_parser = validate_letters)]
    letters: String,

    /// Dictionary file
    #[clap(
        short = 'd',
        long = "dictionary",
        default_value = "/etc/dictionaries-common/words"
    )]
    dictionary_file: String,

    /// Minimum word length to find
    #[clap(short = 'm', long = "min-len", default_value_t = 3)]
    min_len: u8,
}

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Print details
    println!(
        "{} letters: {}",
        args.letters.len(),
        args.letters
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );

    // Load words
    let dictionary = load_words_from_file(&args.dictionary_file, args.letters.len())?;

    // Find words
    let words = find_words(&args.letters, &dictionary, args.min_len);

    // Print results
    print_results(words);

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

#[cfg(test)]
mod tests {
    use super::*;
    use dictionary::{load_words_from_bufreader, LetterNext};
    use io::BufReader;

    #[test]
    fn size_checks() {
        assert_eq!(8, std::mem::size_of::<LetterNext>());
    }

    #[test]
    fn side() {
        // Create dictionary with one word in it "side"
        let bufreader = BufReader::new("side".as_bytes());
        let word_list = load_words_from_bufreader(bufreader, 4).unwrap();

        // Find words
        let words = find_words("ESID", &word_list, 1);

        // Should be one found
        assert_eq!(words, vec!["SIDE".to_string()]);
    }
}
