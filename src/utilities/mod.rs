use regex::Regex;

pub fn is_number(input: &str) -> bool {
    let re = Regex::new(r"-?[0-9]+(\.[0-9]+)?)").unwrap();
    return re.is_match(input);
}
