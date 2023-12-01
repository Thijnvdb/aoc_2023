use crate::shared::{day_setup::Part, file_utils::get_lines};
use regex::Regex;

pub struct Part1;
struct Part2;

impl Part<Vec<String>> for Part1 {
    fn parse(file: &str) -> Result<Vec<String>, ()> {
        let lines = get_lines(file);
        match lines {
            Ok(l) => return Ok(l),
            Err(_) => return Err(())
        }
    }

    fn run(input: Vec<String>) {
        let re = Regex::new(r"[0-9]+").unwrap();
        for line in input {
            let nums: Vec<i32> = re.captures_iter(&line).map(|c| c.extract()).collect();
        }
    }
}
