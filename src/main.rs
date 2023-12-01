mod shared;
mod days;

use clap::{Parser, builder::TypedValueParser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    day: usize,
    part: usize,
    #[arg(short = 'p', long = "practice", default_value_t = false)]
    practice: bool
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    match args.day {
        1 => match args.part {
            1 => days::day1::Part1,
        }
    };
}
