pub trait Parser: Sized {
    fn parse(s: &str) -> Option<(&str, Self)>;
}

impl<T: Parser> Parser for Box<T> {
    fn parse(s: &str) -> Option<(&str, Self)> {
        let (s, res) = T::parse(s)?;
        Some((s, Box::new(res)))
    }
}

impl Parser for u32 {
    fn parse(s: &str) -> Option<(&str, Self)> {
        let mut chs = s.chars();
        let mut val = 0;
        match chs.clone().peekable().peek() {
            Some(c) if c.is_ascii_digit() => {},
            _ => return None,
        }
        loop {
            match chs.clone().peekable().peek() {
                Some(c) if c.is_ascii_digit() => {
                    chs.next();
                    val = 10 * val + c.to_digit(10).unwrap();
                },
                _ => { break; },
            }
        }
        Some((chs.as_str(), val))
    }
}

impl Parser for char {
    fn parse(s: &str) -> Option<(&str, Self)> {
        let mut chs = s.chars();
        match chs.next() {
            Some(c) => Some((chs.as_str(), c)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use meta_parsegen_derive::{Parser, parse_unit};

    #[test]
    fn test_u32() {
        assert_eq!(u32::parse(""), None);
        assert_eq!(u32::parse("0"), Some(("", 0)));
        assert_eq!(u32::parse("1"), Some(("", 1)));
        assert_eq!(u32::parse("12"), Some(("", 12)));
        assert_eq!(u32::parse("123"), Some(("", 123)));
        assert_eq!(u32::parse("123abc"), Some(("abc", 123)));
    }

    #[test]
    fn test_char() {
        assert_eq!(char::parse(""), None);
        assert_eq!(char::parse("a"), Some(("", 'a')));
        assert_eq!(char::parse("abc"), Some(("bc", 'a')));
    }

    #[test]
    fn test_box() {
        assert_eq!(Box::<u32>::parse("123abc"), Some(("abc", Box::new(123))));
        assert_eq!(Box::<char>::parse("abc"), Some(("bc", Box::new('a'))));
    }

    #[derive(Debug, PartialEq, Parser)]
    struct AorB2(AorB, AorB);

    #[derive(Debug, PartialEq, Parser)]
    enum AorB {
        ItemA(A),
        ItemB(B),
    }

    #[derive(Debug, PartialEq, Parser)]
    struct AA(A, A);

    #[derive(Debug, PartialEq, Parser)]
    struct AA2 { a: A, a2: A }

    #[derive(Debug, PartialEq)]
    #[parse_unit('a')]
    struct A;

    #[derive(Debug, PartialEq)]
    #[parse_unit('b')]
    struct B;

    #[test]
    fn test() {
        assert_eq!(A::parse(""), None);
        assert_eq!(A::parse("abc"), Some(("bc", A)));
        assert_eq!(A::parse("bbc"), None);
        assert_eq!(AA::parse("aab"), Some(("b", AA(A, A))));
        assert_eq!(AA::parse("bbc"), None);
        assert_eq!(AA2::parse("aab"), Some(("b", AA2 { a: A, a2: A })));
        assert_eq!(AA2::parse("bbc"), None);
        assert_eq!(AorB::parse("aac"), Some(("ac", AorB::ItemA(A))));
        assert_eq!(AorB::parse("bbc"), Some(("bc", AorB::ItemB(B))));
        assert_eq!(AorB2::parse("abc"), Some(("c", AorB2(AorB::ItemA(A), AorB::ItemB(B)))));
    }
}
