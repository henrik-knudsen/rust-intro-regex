use std::{fs::read_to_string, path::Path};

use anyhow::{anyhow, Result};
use clap::Parser;
use owo_colors::OwoColorize;
use regex::{self, Match, Regex};

fn main() -> Result<()> {
    // TODO: Add options for replacing text, regex flags (case insensitive ..) and output options
    let args = Args::parse();

    let re = Regex::new(&args.pattern)?;

    for path in args.path {
        search_path(&re, path)?;
    }

    Ok(())
}

/// Searches with the regex pattern in a given path (either directory or file).
fn search_path(re: &Regex, path: String) -> Result<()> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err(anyhow!("Path '{}' does not exist", path.display()));
    }

    if path.is_dir() {
        search_dir(re, path)?;
    } else {
        search_file(re, path)?;
    }

    Ok(())
}

/// Searches with the regex pattern in a directory path.
/// Delegate to search_file for file paths.
/// In the case of sub-directories, we recurse.
fn search_dir(re: &Regex, dir: &Path) -> Result<()> {
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            search_dir(re, &path)?;
        } else {
            search_file(re, &path)?;
        }
    }

    Ok(())
}

/// Searches with the regex pattern in a file path
/// File path and matching lines are printed to standard out
fn search_file(re: &Regex, path: &Path) -> Result<()> {
    let mut has_printed_file_path = false;

    let content = read_to_string(&path)?;

    // TODO: println! does not buffer writes, and can become bottleneck if called frequently in a loop.
    // Use BufWriter<T> (https://doc.rust-lang.org/std/io/struct.BufWriter.html)
    // with std::io::stdout (https://doc.rust-lang.org/std/io/fn.stdout.html)

    // TODO: Current approach allocates a lot of extra memory (Vec for all matches + new string for formatted output)
    // Avoid the allocations.

    for (i, line) in content.lines().enumerate() {
        let mut matches = Vec::new();

        for matched in re.find_iter(line) {
            matches.push(matched);
        }

        if matches.is_empty() {
            continue;
        }

        if !has_printed_file_path {
            println!("{}", &path.display().purple());
            has_printed_file_path = true;
        }

        let formatted = format_matches(&matches, i, line);

        println!("{}", formatted);
    }

    if has_printed_file_path {
        println!();
    }

    Ok(())
}

/// Helper function for generating a String for line with linenumber in green and
/// all matches highlighted in bright red.
fn format_matches(matches: &Vec<Match>, index: usize, line: &str) -> String {
    let mut result = String::new();
    let mut last_index = 0;

    // green line number
    result.push_str(format!("{}:", (index + 1).green()).as_str());

    for matched in matches {
        // non matching part of line
        result.push_str(&line[last_index..matched.start()]);

        // matching part of line, in bright red
        result.push_str(
            &(&line[matched.start()..matched.end()])
                .bright_red()
                .to_string(),
        );

        last_index = matched.end();
    }

    // push remaining part of line
    result.push_str(&line[last_index..]);

    result
}

/// CLI arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// A regular expression used for searching.
    pattern: String,

    /// A file or directory to search.
    path: Vec<String>,
    //
    // Add more options ..
}
