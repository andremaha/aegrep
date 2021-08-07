use clap::{clap_app, crate_version};
use regex::Regex;
use failure::Error:

use std::path::Path;

#[derive(Debug)]
struct Record {
    line: usize,
    txt: String
}

/// Searches for the matches to regular expression in any given file.
///
/// # Example
///
/// Basic usage:
/// ```
///     let cp = clap_app!(
///        aegrep =>
///        (version: crate_version!())
///        (about: "A Grep like program: Andrey's Rust Tutorial")
///        (author: "Andrey I. Esaulov")
///        (@arg file: -f -- file +takes_value "The file to test")
///        (@arg pattern: +required "The regex pattern to search for")
///    )
///        .get_matches();
///
///     let re = Regex::new(cp.value_of("pattern").unwrap()).map_err(|_| "bad regex")?;
///     let p = process_file(cp.value_of("file").ok_or("No file chosen")?, re); // or_or - easiest way to get String out of Option
/// ```
fn process_file<P: AsRef<Path>>(p: P, re: Regex) -> Result<Vec<Record>, String> {
    let mut res = Vec::new();

    // Let's grab the bytes from the file, fail with an error, if no file provided
    let bytes  = std::fs::read(p).map_err(|_| "could not read string".to_string())?;

    // Only read files that are convertible to strings
    if let Ok(ss) = String::from_utf8(bytes) {
        for (i, l) in ss.lines().enumerate() {
            if re.is_match(l) {
                res.push(Record {
                    line: i,
                    txt: l.to_string()
                });
            }
        }
    }

    Ok(res)
}

fn main() -> Result<(), String> {
   let cp = clap_app!(
       aegrep =>
       (version: crate_version!())
       (about: "A Grep like program: Andrey's Rust Tutorial")
       (author: "Andrey I. Esaulov")
       (@arg file: -f -- file +takes_value "The file to test")
       (@arg pattern: +required "The regex pattern to search for")
   )
       .get_matches();

    let re = Regex::new(cp.value_of("pattern").unwrap()).map_err(|_| "bad regex")?;

    let p = process_file(cp.value_of("file").ok_or("No file chosen")?, re); // or_or - easiest way to get String out of Option

    println!("{:?}", p);
    Ok(())
}
