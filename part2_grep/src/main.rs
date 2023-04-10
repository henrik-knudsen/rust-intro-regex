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
    // std::path::Path is a useful type for working with file/directory paths
    let path = Path::new(&path);

    /*
       Implement logic for:
           Traversing file/directory paths
           Read content (line for line)
           Output lines with matches with correct color (green line numbers, bright red for matching parts)
    */
    todo!()
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
