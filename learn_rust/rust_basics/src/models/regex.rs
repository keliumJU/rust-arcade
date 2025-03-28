use regex::Regex;

pub fn show_regex_ex() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2024-01-01"));
}
