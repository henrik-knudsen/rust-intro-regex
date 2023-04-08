#[cfg(test)]
mod tests {

    use regex::{self, Captures, Regex, RegexBuilder};

    use crate::utils::assert_match;

    #[test]
    fn exact_match() {
        const HAYSTACK: &str = r"
        hello
        hello
        hello
        ";

        // Write a regex which matches each 'hello' in the haystack
        const PATTERN: &str = r"hello";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);
    }

    #[test]
    fn wildcard() {
        const HAYSTACK: &str = r"
        hallo
        hello
        hi
        ";

        // Write a regex which matches 'hello' and 'hallo', but not hi
        const PATTERN: &str = r"h.llo";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 2);
        assert_match(&captures[0], "hallo");
        assert_match(&captures[1], "hello");
    }

    #[test]
    fn sets() {
        const HAYSTACK: &str = r"
        man 
        fan 
        can 
        dan 
        ran 
        pan
        ";

        // Write a regex which matches 'man', 'fan', and 'can, but none of the others words
        const PATTERN: &str = r"[mfc]an";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);
        assert_match(&captures[0], "man");
        assert_match(&captures[1], "fan");
        assert_match(&captures[2], "can");
    }

    #[test]
    fn sets_negative() {
        const HAYSTACK: &str = r"
        dog 
        hog 
        fog 
        bog
        ";

        // Write a regex which matches all the words not starting with 'b' (e.g. 'dog', 'hog', 'bog')
        const PATTERN: &str = r"[^b]og";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);
        assert_match(&captures[0], "dog");
        assert_match(&captures[1], "hog");
        assert_match(&captures[2], "fog");
    }

    #[test]
    fn ranges() {
        const HAYSTACK: &str = r"
        Ann 
        Bob 
        ann 
        bob
        ";

        // Write a regex which matches all the words with capital letters (e.g. 'Ann', 'Bob')
        const PATTERN: &str = r"[A-C]..";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 2);
        assert_match(&captures[0], "Ann");
        assert_match(&captures[1], "Bob");
    }

    #[test]
    fn quantifier1() {
        const HAYSTACK: &str = r"
        bar 
        baz 
        baaz 
        baaaz 
        baaaaz
        ";

        // Write a regex which matches all variants of 'baz' ('baz', 'baaz', ...), but not 'bar'
        const PATTERN: &str = r"ba*z";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 4);
        assert_match(&captures[0], "baz");
        assert_match(&captures[1], "baaz");
        assert_match(&captures[2], "baaaz");
        assert_match(&captures[3], "baaaaz");
    }

    #[test]
    fn quantifier2() {
        const HAYSTACK: &str = r"
        bar 
        baz 
        baaz 
        baaaz 
        baaaaz
        ";

        // Write a regex which matches all variants of 'baz' with
        // at least 2 a's ('baaz', 'baaaz', ...)
        const PATTERN: &str = r"ba{2,}z";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);
        assert_match(&captures[0], "baaz");
        assert_match(&captures[1], "baaaz");
        assert_match(&captures[2], "baaaaz");
    }

    #[test]
    fn quantifier3() {
        const HAYSTACK: &str = r"
        wazup 
        wazzzup 
        wazzzzzup 
        wazzzzzzzzzzzzzzup
        ";

        // Write a regex which matches all variants of wazup, with 3 to 5 z's.
        const PATTERN: &str = r"waz{3,5}up";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 2);
        assert_match(&captures[0], "wazzzup");
        assert_match(&captures[1], "wazzzzzup");
    }

    #[test]
    fn phone_number() {
        const HAYSTACK: &str = "800-222-5555";

        // Write a regex which matches the phone number (hint: \d matches a digit)
        const PATTERN: &str = r"\d{3}-\d{3}-\d{4}";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 1);
        assert_match(&captures[0], "800-222-5555");
    }

    #[test]
    fn optional() {
        const HAYSTACK: &str = r"
        1 file was found?
        2 files was found?
        24 files was found?
        No files was found.
        ";

        // Write a regex which matches all lines with at least one file found.
        const PATTERN: &str = r"\d* files? was found\?";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);
        assert_match(&captures[0], "1 file was found?");
        assert_match(&captures[1], "2 files was found?");
        assert_match(&captures[2], "24 files was found?");
    }

    #[test]
    fn conditional() {
        const HAYSTACK: &str = r"
        I love cats
        I love dogs
        I love logs
        I love cogs
        ";

        // Write a regex which the lines with cats and dogs, but not with logs or cogs.
        const PATTERN: &str = r"I love (cats|dogs)";

        let re = Regex::new(PATTERN).unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 2);
        assert_match(&captures[0], "I love cats");
        assert_match(&captures[1], "I love dogs");
    }

    #[test]
    fn start_and_end() {
        const HAYSTACK: &str = "
Mission: successful
Last Mission: succesfully completed
Next Mission: successful upon capture of target
        ";

        // Write a regex which matches only first line
        const PATTERN: &str = r"^Mission: successful";

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 1);
        assert_match(&captures[0], "Mission: successful");
    }
}
