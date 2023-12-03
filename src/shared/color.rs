pub fn red(input: &str) -> String {
    format!("{}{}\x1b[0m", truecolor(255, 0, 0), input)
}

pub fn truecolor(r: u8, g: u8, b: u8) -> String {
    return format!("\x1b[38;2;{};{};{}m", r, g, b);
}

// pub fn ansi256(r: u8, g: u8, b: u8) -> String {
//     return format!("\x1b[38;5;{}m", rgb2ansi256::rgb_to_ansi256(r, g, b));
// }
