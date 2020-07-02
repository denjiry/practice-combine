use combine::char::{letter, string};
use combine::{between, many1, ParseError, ParseResult, Parser, Stream};

fn main() {
    let mut input = "r###hello#world###";
    println!("{:?}", rawstr(&mut input));
}

pub fn rawstr<'a, I>(input: &mut I) -> ParseResult<(), I>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    let r = string("r").map(|_| ());
    let sharps1 = many1::<Vec<_>, _>(string("#"));
    let sharps2 = many1::<Vec<_>, _>(string("#"));
    let contents = many1::<Vec<_>, _>(letter());
    let sharps_paren = between(sharps1, sharps2, contents);
    let mut rawstr = r.and(sharps_paren).map(|_| ());
    rawstr.parse_stream(input)
}
