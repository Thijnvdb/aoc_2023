use std::{borrow::BorrowMut, collections::HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, satisfy},
    error_position,
    multi::many0,
    AsChar, IResult, Parser,
};
use regex::Regex;
use uuid::Uuid;

use crate::shared::{
    color::red,
    day_setup::{Day, Part},
    file_utils::get_lines,
};

pub struct Day3;

impl Day for Day3 {
    fn execute(self: &Self, part: usize, file_path: &str) -> Result<(), String> {
        match part {
            1 => Part1::run(self::Part1::parse(file_path)),
            // 2 => Part2::run(self::Part2::parse(file_path)),
            _ => return Err(format!("part '{}' not found for day 2", part)),
        };
        Ok(())
    }
}

struct Part1;
struct Part2;

struct Board {
    squares: Vec<Vec<Square>>,
}

struct Square {
    id: Option<Uuid>,
    text: String,
    is_symbol: bool,
    value: u32,
}

fn parse_board(lines: Vec<String>) -> Board {
    let mut squares = Vec::<Vec<Square>>::new();
    for line in lines {
        let parsed = parse_line(&line).unwrap();
        squares.push(parsed.1);
    }

    Board { squares }
}

fn parse_line(input: &str) -> IResult<&str, Vec<Square>> {
    let (input, nommed) = many0(alt((
        tag(".").map(|c: &str| c.to_string()),
        digit1.map(|c: &str| c.to_string()),
        parse_symbol.map(|c| c.to_string()),
    )))(input)?;

    let mut res: Vec<Square> = Vec::new();
    for input in nommed {
        if input == "." || is_symbol(&input) {
            let square = match input.as_str() {
                "." => Square {
                    id: None,
                    text: input.to_string(),
                    is_symbol: false,
                    value: 0,
                },
                _ => Square {
                    id: None,
                    text: input.to_string(),
                    is_symbol: true,
                    value: 0,
                },
            };
            res.push(square);
            continue;
        }

        // did not return yet, so this is a number
        // enter an entry for each character as a square with the total value of the text block
        let value = input.parse::<u32>().unwrap();
        let id = Uuid::new_v4();
        for char in input.chars() {
            res.push(Square {
                id: Some(id),
                text: char.to_string(),
                is_symbol: false,
                value,
            })
        }
    }

    Ok((input, res))
}

fn is_symbol(input: &str) -> bool {
    let symbol_regex = Regex::new(r"[^0-9\.]{1}").unwrap();
    symbol_regex.is_match(input)
}

fn parse_symbol(input: &str) -> IResult<&str, char> {
    let (input, symbol) = satisfy(|c| is_symbol(c.to_string().as_str()))(input)?;
    if symbol.len() == 0 {
        return Err(nom::Err::Error(error_position!(
            input,
            nom::error::ErrorKind::Satisfy
        )));
    }

    Ok((input, symbol))
}

fn get_surrounding_values(x: usize, y: usize, board: &mut Board) -> HashSet<Uuid> {
    let x_range = match x {
        0 => vec![x, x + 1],
        _ => vec![x - 1, x, x + 1],
    };

    let y_range = match y {
        0 => vec![y, y + 1],
        _ => vec![y - 1, y, y + 1],
    };

    let mut accounted = std::collections::hash_set::HashSet::<Uuid>::new();
    for dy in y_range.to_owned() {
        for dx in x_range.to_owned() {
            if dx >= board.squares.first().unwrap().len() || dy >= board.squares.len() {
                continue;
            }

            if let Some(id) = board.squares[dy][dx].id {
                accounted.insert(id);
            }
        }
    }

    accounted
}

impl Part<Board> for Part1 {
    fn parse(file_path: &str) -> Board {
        let lines = get_lines(file_path);
        parse_board(lines)
    }

    fn run(mut input: Board) {
        let board = input.borrow_mut();

        let mut part_number_ids = HashSet::<Uuid>::new();
        for y in 0..board.squares.len() {
            for x in 0..board.squares.first().unwrap().len() {
                if board.squares[y][x].is_symbol {
                    let surrounding = get_surrounding_values(x, y, board);
                    for x in surrounding {
                        part_number_ids.insert(x);
                    }
                }
            }
        }

        let mut output = 0;
        let mut checked = HashSet::<Uuid>::new();
        for y in 0..board.squares.len() {
            for x in 0..board.squares.first().unwrap().len() {
                if let Some(id) = board.squares[y][x].id {
                    if part_number_ids.contains(&id) {
                        print!("{}", red(board.squares[y][x].text.as_str()));
                        if checked.contains(&id) {
                            continue;
                        }
                        output += board.squares[y][x].value;
                    } else {
                        print!("{}", board.squares[y][x].text.as_str());
                    }

                    checked.insert(id);
                } else {
                    print!("{}", board.squares[y][x].text.as_str());
                }
            }
            println!();
        }

        println!("Output: {}", output);
    }
}
