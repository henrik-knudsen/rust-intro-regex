#[cfg(test)]
mod tests {

    use regex::{self, Captures, RegexBuilder};

    use crate::utils::{assert_captured_group, assert_captured_group_is_none, assert_match};

    #[test]
    fn decimal() {
        const HAYSTACK: &str = r"
3.14529
-255.34
128
1.9e10
123,340.00
720p
        ";

        // Numbers can sometimes come in various formats (e.g. different decimal formats,
        // scientific notation, commas or dots, with or without sign.)
        // Write a regex which the different decimal formats.
        // 720p is not a valid decimal format and should not be matched.
        const PATTERN: &str = r"^-?(\d+,?)+(\.)?\d+(e\d+)?$";

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 5);
        assert_match(&captures[0], "3.14529");
        assert_match(&captures[1], "-255.34");
        assert_match(&captures[2], "128");
        assert_match(&captures[3], "1.9e10");
        assert_match(&captures[4], "123,340.00");
    }

    #[test]
    fn phonenumbers() {
        const HAYSTACK: &str = r"
        555-1234
        415-555-1234
        650-555-2345
        (416)555-3456
        202 555 4567
        4035555678
        1 416 555 9292
        ";

        // (American) phone numbers can come in different kinds of formats, depending on whether it is
        // an international number, has an area code (domestic vs local numbers),
        // and preferences in terms of using dashes (-), space or no space to separate digits.
        // Examples:
        // 555-1234 -- Local phone number
        // 464-555-1234 -- Domestic phone number
        // +1-212-456-7890 -- International phone number
        // 1-212-456-7890 -- International phone number, called from America

        // TODO: Write a regex which matches all the variants of phone numbers shown above
        const PATTERN: &str = r"(\d )?(\d{3}[- ]?|\((\d{3})\))?\d{3}[- ]?\d{4}";

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 7);

        assert_match(&captures[0], "555-1234");
        assert_match(&captures[1], "415-555-1234");
        assert_match(&captures[2], "650-555-2345");
        assert_match(&captures[3], "(416)555-3456");
        assert_match(&captures[4], "202 555 4567");
        assert_match(&captures[5], "4035555678");
        assert_match(&captures[6], "1 416 555 9292");
    }

    #[test]
    fn email() {
        const HAYSTACK: &str = r"
        tom@hogwarts.com
        tom.riddle@hogwarts.com
        tom.riddle+work@hogwarts.com
        tom@hogwarts.eu.com
        potter@hogwarts.com
        harry@hogwarts.com
        hermione+work@hogwarts.com
        ";

        /// Email addresses can be challenging to match against, due to the complexity
        /// of the specification.
        ///
        /// Examples:
        /// tom@hogwarts.com
        /// tom.riddle@hogwarts.com -- Email 'name' with multiple components
        /// tom.riddle+work@hogwarts.com -- Email with a filter (the component after +, e.g. 'work')
        /// tom@hogwarts.eu.com -- Email with domain with multiple components

        /// TODO: Write a regex which matches all the variants of email shown above
        /// AND captures the email name (e.g. tom, tom.riddle etc.)
        const PATTERN: &str = r"((\w+\.)?(\w+))(\+\w+)?@(\w+)(\.(\w+))+";
        /// Group index (starting from 1) of the group capturing the email 'name'
        /// By default set to 1 here, update it if this does not match your pattern.
        const EMAIL_NAME_CAPTURE_GROUP_INDEX: usize = 1;

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 7);

        assert_match(&captures[0], "tom@hogwarts.com");
        assert_captured_group(&captures[0], EMAIL_NAME_CAPTURE_GROUP_INDEX, "tom");

        assert_match(&captures[1], "tom.riddle@hogwarts.com");
        assert_captured_group(&captures[1], EMAIL_NAME_CAPTURE_GROUP_INDEX, "tom.riddle");

        assert_match(&captures[2], "tom.riddle+work@hogwarts.com");
        assert_captured_group(&captures[2], EMAIL_NAME_CAPTURE_GROUP_INDEX, "tom.riddle");

        assert_match(&captures[3], "tom@hogwarts.eu.com");
        assert_captured_group(&captures[3], EMAIL_NAME_CAPTURE_GROUP_INDEX, "tom");

        assert_match(&captures[4], "potter@hogwarts.com");
        assert_captured_group(&captures[4], EMAIL_NAME_CAPTURE_GROUP_INDEX, "potter");

        assert_match(&captures[5], "harry@hogwarts.com");
        assert_captured_group(&captures[5], EMAIL_NAME_CAPTURE_GROUP_INDEX, "harry");

        assert_match(&captures[6], "hermione+work@hogwarts.com");
        assert_captured_group(&captures[6], EMAIL_NAME_CAPTURE_GROUP_INDEX, "hermione");
    }

    #[test]
    fn whitespace() {
        const HAYSTACK: &str = r"
              The quick brown fox...        
  jumps over the lazy dog.           
        ";

        /// Input sometimes may have a mismatch of leading and trailing whitespace.
        /// \s can be used to match against a whitespace character.
        /// In combination with ^ and $, we can exclude leading and trailing whitespace.

        /// TODO: Write a regex which matches and captures the sentence
        /// without including the leading or trailing whitespace.
        const PATTERN: &str = r"^\s*(.*\S)\s*$";
        /// Group index (starting from 1) of the group capturing the sentence
        /// By default set to 1 here, update it if this does not match your pattern.
        const SENTENCE_CAPTURE_GROUP_INDEX: usize = 1;

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 2);

        assert_match(
            &captures[0],
            "\n              The quick brown fox...        ",
        );
        assert_captured_group(
            &captures[0],
            SENTENCE_CAPTURE_GROUP_INDEX,
            "The quick brown fox...",
        );

        assert_match(
            &captures[1],
            "  jumps over the lazy dog.           \n        ",
        );
        assert_captured_group(
            &captures[1],
            SENTENCE_CAPTURE_GROUP_INDEX,
            "jumps over the lazy dog.",
        );
    }

    #[test]
    fn stacktrace() {
        const HAYSTACK: &str = r"
        W/dalvikvm( 1553): threadid=1: uncaught exception      
        E/( 1553): FATAL EXCEPTION: main
        E/( 1553): java.lang.StringIndexOutOfBoundsException
        E/( 1553):   at widget.List.makeView(ListView.java:1727)
        E/( 1553):   at widget.List.fillDown(ListView.java:652)
        E/( 1553):   at widget.List.fillFrom(ListView.java:709)
        ";

        /// The text above is an example stacktrace (Android).  

        /// TODO: Write a regex which matches and captures the the method name,
        /// the filename (with the extension) and the linenumber.
        /// NB: The relevant lines follow the pattern: 'at package.Class.MethodName(File.java:LineNumber)'
        const PATTERN: &str = r"(?:\w+)\.(?:\w+)\.(\w+)\((\w+\.java):(\d{0,})\)";

        const METHOD_NAME_CAPTURE_GROUP_INDEX: usize = 1;
        const FILE_NAME_CAPTURE_GROUP_INDEX: usize = 2;
        const LINE_NUMBER_CAPTURE_GROUP_INDEX: usize = 3;

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 3);

        assert_match(&captures[0], "widget.List.makeView(ListView.java:1727)");
        assert_captured_group(&captures[0], METHOD_NAME_CAPTURE_GROUP_INDEX, "makeView");
        assert_captured_group(&captures[0], FILE_NAME_CAPTURE_GROUP_INDEX, "ListView.java");
        assert_captured_group(&captures[0], LINE_NUMBER_CAPTURE_GROUP_INDEX, "1727");

        assert_match(&captures[1], "widget.List.fillDown(ListView.java:652)");
        assert_captured_group(&captures[1], METHOD_NAME_CAPTURE_GROUP_INDEX, "fillDown");
        assert_captured_group(&captures[1], FILE_NAME_CAPTURE_GROUP_INDEX, "ListView.java");
        assert_captured_group(&captures[1], LINE_NUMBER_CAPTURE_GROUP_INDEX, "652");

        assert_match(&captures[2], "widget.List.fillFrom(ListView.java:709)");
        assert_captured_group(&captures[2], METHOD_NAME_CAPTURE_GROUP_INDEX, "fillFrom");
        assert_captured_group(&captures[2], FILE_NAME_CAPTURE_GROUP_INDEX, "ListView.java");
        assert_captured_group(&captures[2], LINE_NUMBER_CAPTURE_GROUP_INDEX, "709");
    }

    #[test]
    fn uri() {
        const HAYSTACK: &str = r"
        ftp://file_server.com:21/top_secret/life_changing_plans.pdf
        https://regexone.com/lesson/introduction#section
        file://localhost:4040/zip_file
        https://s3cur3-server.com:9999/
        market://search/angry%20birds
        https://vg.no
        ";

        /// When working with files and resources over a network, you will often come across URIs
        /// and URLs which can be parsed and worked with directly. Most standard libraries will
        /// have classes to parse and construct these kind of identifiers, but if you need to
        /// match them in logs or a larger corpus of text, you can use regular expressions to pull
        /// out information from their structured format quite easily.
        ///
        /// URIs, or Uniform Resource Identifiers, are a representation of a resource that is
        /// generally composed of a scheme, host, port (optional), and resource path.
        ///
        /// Example:  http://regexone.com:80/page
        ///
        /// scheme: http
        /// host: regexone.com
        /// port: 80
        /// resource path: /page

        /// TODO: Write a regex which matches and captures the scheme, host, port number (optional)
        /// and resource path (also optional).
        const PATTERN: &str =
            r"(\w+)://([\w-]+(?:\.\w{2,4})?)(?::(\d{0,5}))?((?:/[\w%#]*)+(?:\.\w+)?)?";

        const SCHEME_CAPTURE_GROUP_INDEX: usize = 1;
        const HOST_CAPTURE_GROUP_INDEX: usize = 2;
        const PORT_NUMBER_CAPTURE_GROUP_INDEX: usize = 3;
        const PATH_CAPTURE_GROUP_INDEX: usize = 4;

        let re = RegexBuilder::new(PATTERN).multi_line(true).build().unwrap();
        let captures: Vec<Captures> = re.captures_iter(HAYSTACK).collect();

        assert_eq!(captures.len(), 6);

        assert_match(
            &captures[0],
            "ftp://file_server.com:21/top_secret/life_changing_plans.pdf",
        );
        assert_captured_group(&captures[0], SCHEME_CAPTURE_GROUP_INDEX, "ftp");
        assert_captured_group(&captures[0], HOST_CAPTURE_GROUP_INDEX, "file_server.com");
        assert_captured_group(&captures[0], PORT_NUMBER_CAPTURE_GROUP_INDEX, "21");
        assert_captured_group(
            &captures[0],
            PATH_CAPTURE_GROUP_INDEX,
            "/top_secret/life_changing_plans.pdf",
        );

        assert_match(
            &captures[1],
            "https://regexone.com/lesson/introduction#section",
        );
        assert_captured_group(&captures[1], SCHEME_CAPTURE_GROUP_INDEX, "https");
        assert_captured_group(&captures[1], HOST_CAPTURE_GROUP_INDEX, "regexone.com");
        assert_captured_group_is_none(&captures[1], PORT_NUMBER_CAPTURE_GROUP_INDEX);
        assert_captured_group(
            &captures[1],
            PATH_CAPTURE_GROUP_INDEX,
            "/lesson/introduction#section",
        );

        assert_match(&captures[2], "file://localhost:4040/zip_file");
        assert_captured_group(&captures[2], SCHEME_CAPTURE_GROUP_INDEX, "file");
        assert_captured_group(&captures[2], HOST_CAPTURE_GROUP_INDEX, "localhost");
        assert_captured_group(&captures[2], PORT_NUMBER_CAPTURE_GROUP_INDEX, "4040");
        assert_captured_group(&captures[2], PATH_CAPTURE_GROUP_INDEX, "/zip_file");

        assert_match(&captures[3], "https://s3cur3-server.com:9999/");
        assert_captured_group(&captures[3], SCHEME_CAPTURE_GROUP_INDEX, "https");
        assert_captured_group(&captures[3], HOST_CAPTURE_GROUP_INDEX, "s3cur3-server.com");
        assert_captured_group(&captures[3], PORT_NUMBER_CAPTURE_GROUP_INDEX, "9999");
        assert_captured_group(&captures[3], PATH_CAPTURE_GROUP_INDEX, "/");

        assert_match(&captures[4], "market://search/angry%20birds");
        assert_captured_group(&captures[4], SCHEME_CAPTURE_GROUP_INDEX, "market");
        assert_captured_group(&captures[4], HOST_CAPTURE_GROUP_INDEX, "search");
        assert_captured_group_is_none(&captures[4], PORT_NUMBER_CAPTURE_GROUP_INDEX);
        assert_captured_group(&captures[4], PATH_CAPTURE_GROUP_INDEX, "/angry%20birds");

        assert_match(&captures[5], "https://vg.no");
        assert_captured_group(&captures[5], SCHEME_CAPTURE_GROUP_INDEX, "https");
        assert_captured_group(&captures[5], HOST_CAPTURE_GROUP_INDEX, "vg.no");
        assert_captured_group_is_none(&captures[5], PORT_NUMBER_CAPTURE_GROUP_INDEX);
        assert_captured_group_is_none(&captures[5], PATH_CAPTURE_GROUP_INDEX);
    }
}
