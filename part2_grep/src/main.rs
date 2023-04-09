use std::{fs::read_to_string, path::Path};

use anyhow::{anyhow, Result};
use clap::Parser;
use owo_colors::OwoColorize;
use regex::{self, Match, Regex};

fn main() -> Result<()> {
    let args = Args::parse();

    let re = Regex::new(&args.pattern)?;

    for path in args.path {
        search_path(&re, path)?;
    }

    Ok(())
}

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

fn search_file(re: &Regex, path: &Path) -> Result<()> {
    let mut has_printed_file_path = false;

    let content = read_to_string(&path)?;

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

fn format_matches(matches: &Vec<Match>, index: usize, line: &str) -> String {
    let mut result = String::new();
    let mut last_index = 0;

    // green line number
    result.push_str(format!("{}:", (index + 1).green()).as_str());

    for matched in matches {
        // non matching part of line
        result.push_str(&line[last_index..matched.start()]);

        // matching part of line
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// A regular expression used for searching.
    pattern: String,

    /// A file or directory to search.
    path: Vec<String>,
}
