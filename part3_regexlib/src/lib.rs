use compile::{compile, Program};
use execute::execute;
use parse::parse;

mod compile;
mod execute;
mod parse;

pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}

#[derive(Debug)]
pub struct Error;

pub struct Regex {
    program: Program,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, Error> {
        let parsed = parse(pattern);
        let program = compile(&parsed);

        Ok(Self { program })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.is_match_at(text, 0)
    }

    pub fn is_match_at(&self, text: &str, start: usize) -> bool {
        execute(&self.program, text, start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_match() {
        const PATTERN: &str = "abcde";

        let re = Regex::new(PATTERN).unwrap();

        assert!(re.is_match(&"abcde"));
        assert!(re.is_match(&"aabcde"));
        assert!(re.is_match(&"abcdeeeeee"));
        assert!(re.is_match(&"aaaaabcdeeeeee"));
    }

    #[test]
    fn is_match_at() {
        const PATTERN: &str = "abcde";

        let re = Regex::new(PATTERN).unwrap();

        assert!(re.is_match_at(&"abcdexabcde", 0));
        assert!(re.is_match_at(&"abcdexabcde", 5));
    }
}
