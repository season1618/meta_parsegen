pub trait Parser: Sized {
    fn parse(s: &str) -> Option<(&str, Self)>;
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use meta_parsegen_derive::Parser;

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
    struct A;

    #[derive(Debug, PartialEq)]
    struct B;

    impl Parser for A {
        fn parse(s: &str) -> Option<(&str, Self)> {
            let mut chs = s.chars();
            if Some('a') == chs.next() {
                Some((chs.as_str(), A))
            } else {
                None
            }
        }
    }

    impl Parser for B {
        fn parse(s: &str) -> Option<(&str, Self)> {
            let mut chs = s.chars();
            if Some('b') == chs.next() {
                Some((chs.as_str(), B))
            } else {
                None
            }
        }
    }

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
