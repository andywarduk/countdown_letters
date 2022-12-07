use std::cmp::{max, Ordering};

use numformat::NumFormat;
#[cfg(any(unix, windows))]
use terminal_size::{terminal_size, Width};

pub fn print_results(mut words: Vec<String>) {
    // Sort words by longest first then alphabetical
    words.sort_by(|a, b| {
        let mut result = b.len().cmp(&a.len());

        if result == Ordering::Equal {
            result = a.cmp(b);
        }

        result
    });

    println!(
        "{} {} found",
        words.len().num_format(),
        if words.len() == 1 { "word" } else { "words" }
    );

    // Group words by length
    let mut last_len = 0;
    let mut last_start = 0;
    let mut groups = Vec::new();

    for (i, word) in words.iter().enumerate() {
        if word.len() != last_len {
            if last_len != 0 {
                groups.push((last_len, last_start, i));
            }

            last_len = word.len();
            last_start = i;
        }
    }

    if last_len != 0 {
        groups.push((last_len, last_start, words.len()));
    }

    // Get terminal size
    let term_width = terminal_width();

    for (wordlen, start, end) in groups {
        println!("== {} letter words ({}) ==", wordlen, end - start);

        let cols = if term_width > 0 {
            max(1, (term_width as usize - 1) / (wordlen + 2))
        } else {
            1
        };

        for line in words[start..end].chunks(cols) {
            println!("  {}", line.join("  "))
        }
    }
}

#[cfg(any(unix, windows))]
fn terminal_width() -> u16 {
    if let Some((Width(w), _)) = terminal_size() {
        w
    } else {
        0
    }
}

#[cfg(not(any(unix, windows)))]
fn terminal_width() -> u16 {
    0
}
