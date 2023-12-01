pub trait Part<T> {
    fn parse(file: &str) -> Result<T, ()>;
    fn run(input: T);
}
