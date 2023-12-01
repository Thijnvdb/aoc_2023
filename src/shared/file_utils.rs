use std::fs;

pub fn get_lines(file: &str) -> Result<Vec<String>, std::io::Error> {
    fs::read_to_string(file)?.lines().map(|s| Ok(s.to_string())).collect()
}
