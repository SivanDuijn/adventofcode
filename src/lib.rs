use std::fs;

pub fn read_file(file: &str) -> String {
    return fs::read_to_string(file)
    .expect("Should have been able to read the file");
}

pub fn split(text: String, delimeter: &str) -> Vec<String> {
    return text.split(delimeter).map(|s| s.to_string())
    .collect();
}

pub fn split_lines(lines: Vec<String>, delimeter: &str) -> Vec<Vec<String>> {
    return lines.iter().map(|l| split(l.to_owned(), delimeter)).collect()
}