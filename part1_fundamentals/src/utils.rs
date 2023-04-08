use regex::Captures;

pub fn assert_match(captures: &Captures, value: &str) {
    let captured_value = captures.get(0).unwrap().as_str();

    assert_eq!(captured_value, value);
}

pub fn assert_captured_group(captures: &Captures, group_index: usize, value: &str) {
    let captured_value = captures.get(group_index).unwrap().as_str();

    assert_eq!(captured_value, value);
}

pub fn assert_captured_group_is_none(captures: &Captures, group_index: usize) {
    let captured_value = captures.get(group_index);

    assert_eq!(captured_value, None);
}
