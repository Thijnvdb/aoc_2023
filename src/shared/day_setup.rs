pub trait Part<T> {
    fn parse(file_path: &str) -> T;
    fn run(input: T);
}

pub trait Day {
    fn execute(self: &Self, part: usize, file_path: &str) -> Result<(), String>;
}
