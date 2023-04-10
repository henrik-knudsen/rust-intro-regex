#[cfg(test)]
mod tests {

    use regex::{self, Captures, Regex, RegexBuilder};

    use crate::utils::{assert_captured_group, assert_match};

    #[test]
    fn group() {
        const HAYSTACK: &str = r"
file_record_transcript.pdf
file_07241999.pdf
testfile_fake.pdf.tmp
        ";

        // Write a regex which captures the filename (without the extension) of pdf files.
        // The regex should not match testfile_fake.pdf.tmp.
        const PATTERN: &str = r"<INSERT_REGEX_HERE>";

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 2);
        assert_captured_group(&captures[0], 1, "file_record_transcript");
        assert_captured_group(&captures[1], 1, "file_07241999")
    }

    #[test]
    fn groups2() {
        const HAYSTACK: &str = r"
        1280x720
        1920x1600
        1024x768
        ";

        // Haystack contains common display resolutions.
        // Write a regex which captures both the width and height of resolution.
        const PATTERN: &str = r"<INSERT_REGEX_HERE>";

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);

        assert_captured_group(&captures[0], 1, "1280");
        assert_captured_group(&captures[0], 2, "720");

        assert_captured_group(&captures[1], 1, "1920");
        assert_captured_group(&captures[1], 2, "1600");

        assert_captured_group(&captures[2], 1, "1024");
        assert_captured_group(&captures[2], 2, "768");
    }

    #[test]
    fn nested_groups() {
        const HAYSTACK: &str = r"
Jan 1987
May 1969
Aug 2011
        ";

        // Write a regex which captures both the full date (e.g. Jan 1987),
        // and also (only) the year (e.g. 1987).
        const PATTERN: &str = r"<INSERT_REGEX_HERE>";

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);

        assert_captured_group(&captures[0], 1, "Jan 1987");
        assert_captured_group(&captures[0], 2, "1987");

        assert_captured_group(&captures[1], 1, "May 1969");
        assert_captured_group(&captures[1], 2, "1969");

        assert_captured_group(&captures[2], 1, "Aug 2011");
        assert_captured_group(&captures[2], 2, "2011");
    }
}
