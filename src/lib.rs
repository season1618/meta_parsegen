pub trait Parser {
    fn parse(s: &str) -> (&str, Self);
}
