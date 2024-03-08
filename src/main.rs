use meta_parsegen::Parser;
use meta_parsegen_derive::Parser;

#[derive(Debug, Parser)]
struct Struct {
}

fn main() {
    println!("{:?}", Struct::parse("abc"));
}
