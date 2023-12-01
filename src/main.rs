mod days;
mod shared;

use clap::Parser;
use shared::day_setup::Day;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: usize,
    part: usize,
    #[arg(short = 'p', long = "practice", default_value_t = false)]
    practice: bool,
}

fn get_day(day: usize) -> Result<impl Day, ()> {
    match day {
        1 => Ok(days::day1::Day1),
        _ => Err(()),
    }
}

fn get_file_path(day: usize, part: usize, practice: bool) -> String {
    let file = match practice {
        true => "example",
        false => "input",
    };

    format!("src/input/day{}/{}{}.txt", day, file, part)
}

fn main() {
    let args = Args::parse();
    let day = get_day(args.day);
    let file_path = get_file_path(args.day, args.part, args.practice);
    match day.unwrap().execute(args.part, &file_path) {
        Err(_) => {
            println!("an error occured!")
        }
        _ => {}
    };
}
