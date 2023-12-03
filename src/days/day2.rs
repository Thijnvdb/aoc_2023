use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space0},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use crate::shared::{
    day_setup::{Day, Part},
    file_utils::get_lines,
};

pub struct Game {
    id: u32,
    selections: Vec<CubeSelection>,
}

pub struct CubeSelection {
    red: u32,
    green: u32,
    blue: u32,
}

pub struct Day2;

impl Day for Day2 {
    fn execute(self: &Self, part: usize, file_path: &str) -> Result<(), String> {
        match part {
            1 => Part1::run(self::Part1::parse(file_path)),
            2 => Part2::run(self::Part2::parse(file_path)),
            _ => return Err(format!("part '{}' not found for day 2", part)),
        };
        Ok(())
    }
}

struct Part1;
struct Part2;

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = nom::character::complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, selections) = parse_selections(input)?;

    Ok((input, Game { id, selections }))
}

fn parse_selections(input: &str) -> IResult<&str, Vec<CubeSelection>> {
    separated_list1(tag("; "), parse_selection)(input)
}

fn parse_selection(input: &str) -> IResult<&str, CubeSelection> {
    let (input, selections) = separated_list1(
        tag(", "),
        alt((
            tuple((digit1, preceded(space0, tag("red")))),
            tuple((digit1, preceded(space0, tag("green")))),
            tuple((digit1, preceded(space0, tag("blue")))),
        )),
    )(input)?;

    let mut res = CubeSelection {
        red: 0,
        green: 0,
        blue: 0,
    };

    for entry in selections {
        let value = entry.0.parse::<u32>().unwrap();
        match entry.1 {
            "red" => res.red = value,
            "green" => res.green = value,
            "blue" => res.blue = value,
            _ => {}
        }
    }

    Ok((input, res))
}

fn is_possible(input: &CubeSelection, bag: &CubeSelection) -> bool {
    input.blue <= bag.blue && input.green <= bag.green && input.red <= bag.red
}

fn get_power(input: &CubeSelection) -> u32 {
    input.red * input.green * input.blue
}

fn get_minimal_required(input: &Game) -> CubeSelection {
    let mut res = CubeSelection {
        red: 0,
        green: 0,
        blue: 0,
    };

    for selection in input.selections.iter() {
        if selection.blue > res.blue {
            res.blue = selection.blue
        }
        if selection.green > res.green {
            res.green = selection.green
        }
        if selection.red > res.red {
            res.red = selection.red
        }
    }
    res
}

impl Part<Vec<Game>> for Part1 {
    fn parse(file_path: &str) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        for line in get_lines(file_path) {
            let game = parse_game(&line).unwrap();
            games.push(game.1);
        }

        games
    }

    fn run(input: Vec<Game>) {
        let bag = CubeSelection {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut output: u32 = 0;
        for game in input {
            let mut valid = true;
            for selection in game.selections {
                if !is_possible(&selection, &bag) {
                    valid = false;
                    break;
                }
            }
            if valid {
                output += game.id;
            }
        }

        println!("Sum of ID's: {}", output);
    }
}

impl Part<Vec<Game>> for Part2 {
    fn parse(file_path: &str) -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        for line in get_lines(file_path) {
            let game = parse_game(&line).unwrap();
            games.push(game.1);
        }

        games
    }

    fn run(input: Vec<Game>) {
        let mut output: u32 = 0;
        for game in input {
            let minimal = get_minimal_required(&game);
            output += get_power(&minimal);
        }

        println!("Sum of power: {}", output);
    }
}
