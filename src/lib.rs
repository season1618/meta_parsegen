pub trait Parser: Sized {
    fn parse(s: &str) -> Option<(&str, Self)>;
}
