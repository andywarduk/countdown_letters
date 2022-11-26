#![warn(missing_docs)]

//! Countdown letters game solver

mod results;

use std::io;
use std::path::Path;
use std::time::Instant;

use clap::Parser;
use dictionary::{Dictionary, WordSizeConstraint};
use numformat::NumFormat;
use solver::{find_words, SolverArgs};

use crate::results::print_results;

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
        default_value_t = default_dict().into(),
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

    // Check we have a dictionary
    if args.dictionary_file.is_empty() {
        eprintln!("No dictionary file given and none of the default dictionaries could be found.");
        eprintln!("Default dictionaries are:");

        for d in DICTS {
            eprintln!("  {}", d);
        }

        std::process::exit(1);
    }

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
    let mut size = WordSizeConstraint::default();

    size.set_min(args.min_len as usize);

    if !args.reuse_letters {
        size.set_max(args.letters.len());
    }

    let dictionary = Dictionary::new_from_file(&args.dictionary_file, size, args.verbose)?;

    // Find words
    let start_time = Instant::now();

    let words = find_words(SolverArgs {
        letters: &args.letters,
        dictionary: &dictionary,
        reuse_letters: args.reuse_letters,
        debug: args.debug,
    });

    if args.verbose {
        println!(
            "Search took {} seconds",
            start_time.elapsed().as_secs_f64().num_format_sigdig(2)
        );
    }

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

const DICTS: [&str; 3] = [
    "words.txt",
    "words.txt.gz",
    "/etc/dictionaries-common/words",
];

fn default_dict() -> &'static str {
    DICTS
        .iter()
        .find(|d| dict_valid(d).is_some())
        .unwrap_or(&"")
}

fn dict_valid(dict: &str) -> Option<String> {
    if Path::new(dict).is_file() {
        Some(dict.into())
    } else {
        None
    }
}
