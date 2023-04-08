use std::{fs::File, io::Read};

use anyhow::Result;
use regex::{self, Regex};

fn main() -> Result<()> {
    let mut file = File::open("data/log.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("Content: \n{content}");

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$")?;

    let matches = re.is_match("2002-10-30 14:33:22");

    println!("{matches}");

    Ok(())
}
