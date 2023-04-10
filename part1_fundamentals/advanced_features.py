import re

"""
Examples of some more advanced regex features, not available in Rust regex crate.
"""

def print_regex_result(regex: str, haystacks: list[str]):
    for haystack in haystacks:
        matched = regex.match(haystack)
        result = None if not matched else (matched.group(), matched.groups(), matched.groupdict())
        print(f"Regex: '{regex.pattern}' matching against haystack: '{haystack}', returned the match: {result}")
    print()



# Backreferences
print("Backreferences\n")
"""
Matches against an opening of an html tag (e.g. <a>) and matches until it is correctly closed again (e.g. </a>).
Both the tag name "a" and what is inside the tag is captured in groups.
"""
regex_tag = re.compile(r"<(?P<tag>[a-z]+)\b[^>]*>(.*?)</(?P=tag)>")
haystacks_tag = ["<a>hello</a>", "<p>hello</p>", "<p>hello</a>", "<foo>hello</f>", "<div><div>hello</div></div>"]
print_regex_result(regex_tag, haystacks_tag)

# Lookaround
print("Lookaround\n")
"""
Validates that a password contains one uppercase letter, one lowercase letter, one digit, one special character and at least 8 total characters.
If the haystack does not yield a match, it is not a valid password (according to the validation rules).
"""
regex_password= re.compile(r"^(?=.*?[A-Z])(?=.*?[a-z])(?=.*?[0-9])(?=.*?[#?!@$%^&*-]).{8,}$")
haystacks_password = ["password123", "Passord", "P4$$wrd", "P4$$word"]
print_regex_result(regex_password, haystacks_password)


