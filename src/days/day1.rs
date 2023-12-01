use crate::shared::{
    day_setup::{Day, Part},
    file_utils::get_lines,
};
use regex::Regex;

pub struct Day1;

impl Day for Day1 {
    fn execute(self: &Self, part: usize, file_path: &str) -> Result<(), String> {
        match part {
            1 => Part1::run(self::Part1::parse(file_path)),
            2 => Part2::run(self::Part2::parse(file_path)),
            _ => return Err(format!("part '{}' not found for day 1", part)),
        };
        Ok(())
    }
}

struct Part1;
struct Part2;

impl Part<Vec<String>> for Part1 {
    fn parse(file_path: &str) -> Vec<String> {
        get_lines(file_path)
    }

    fn run(input: Vec<String>) {
        let numbers = numbers_in_lines(input);
        let mut output = 0;
        for line in numbers {
            let number = line.join("");
            let first = number.chars().nth(0).unwrap();
            let last = number.chars().nth(number.len() - 1).unwrap();
            let value = format!("{}{}", first, last).parse::<i32>().unwrap();
            output += value;
        }

        println!("Value: {}", output);
    }
}

impl Part<Vec<String>> for Part2 {
    fn parse(file_path: &str) -> Vec<String> {
        get_lines(file_path)
    }

    fn run(input: Vec<String>) {
        let re = Regex::new(r"[0-9]").unwrap();
        let digits = get_first_and_last_digit_as_text(input);
        let mut output = 0;
        for line in digits {
            let values: Vec<String> = Vec::from([line.first().unwrap(), line.last().unwrap()])
                .into_iter()
                .map(|c| {
                    return match re.is_match(&c) {
                        true => c.to_owned(),
                        false => text_digit_to_value(c).to_string(),
                    };
                })
                .collect();

            let parsed = format!("{}{}", values.first().unwrap(), values.last().unwrap())
                .parse::<i32>()
                .unwrap();

            output += parsed;
        }

        println!("Output: {}", output);
    }
}

fn text_digit_to_value(digit: &str) -> i32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

// extract numbers and text digits from each line as strings
fn get_first_and_last_digit_as_text(lines: Vec<String>) -> Vec<Vec<String>> {
    let mut res: Vec<Vec<String>> = Vec::new();
    let first_re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let last_re = Regex::new(r"[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();

    for line in lines {
        let reversed = line.chars().rev().collect::<String>();
        let first_match = first_re.find(&line).unwrap();
        let last_match = last_re.find(&reversed).unwrap();

        res.push(Vec::from([
            first_match.as_str().to_string(),
            last_match.as_str().chars().rev().collect::<String>(),
        ]))
    }

    res
}

// extract numbers from each line as strings
fn numbers_in_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    let re = Regex::new(r"[a-zA-Z]+").unwrap();
    let mut res: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let nums: Vec<String> = re
            .split(line.as_str())
            .map(|c| c.to_string())
            .filter(|c| !c.is_empty())
            .collect();
        res.push(nums);
    }

    res
}
