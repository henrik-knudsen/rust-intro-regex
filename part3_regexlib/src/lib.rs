use compile::{compile, Program};
use execute::execute;
use parse::parse;

mod compile;
mod execute;
mod parse;

pub struct Match<'t> {
    pub text: &'t str,
    pub start: usize,
    pub end: usize,
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

    pub fn find(&self, text: &str) -> Option<Match> {
        self.find_at(text, 0)
    }

    // TODO: Implement extracting matches
    // NB: Have to extend execute function
    pub fn find_at(&self, _text: &str, _start: usize) -> Option<Match> {
        todo!()
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

        assert!(!re.is_match(&"acbde"));
        assert!(!re.is_match(&"aacbde"));
        assert!(!re.is_match(&"abcedeeeee"));
        assert!(!re.is_match(&"aaaaacbdeeeeee"));
    }

    #[test]
    fn is_match_at() {
        const PATTERN: &str = "abcde";

        let re = Regex::new(PATTERN).unwrap();

        assert!(re.is_match_at(&"abcdexabcde", 0));
        assert!(re.is_match_at(&"abcdexabcde", 5));
    }
}
