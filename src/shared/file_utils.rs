use std::fs;

pub fn get_lines(file: &str) -> Vec<String> {
    let lines = fs::read_to_string(file);
    match lines {
        Ok(l) => return l.lines().map(|s| s.to_string()).collect(),
        Err(e) => {
            panic!("error occurred while parsing file: {:?} ", e);
        }
    }
}
