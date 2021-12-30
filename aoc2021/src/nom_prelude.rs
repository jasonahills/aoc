pub use nom::branch::*;
pub use nom::bytes::complete::*;
pub use nom::character::complete::*;
pub use nom::combinator::*;
use nom::error::ParseError;
pub use nom::multi::*;
pub use nom::sequence::*;
pub use nom::*;

// It's a little silly that I'm using this rather than the u32 parser provided by nom, but the types work out nicely.
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
  map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

// It's a little silly that I'm using this rather than the u32 parser provided by nom, but the types work out nicely.
pub fn parse_u8(input: &str) -> IResult<&str, u8> {
  map_res(digit1, |s: &str| s.parse::<u8>())(input)
}

// TODO: perhaps it is time to generalize this.
// It's a little silly that I'm using this rather than the u32 parser provided by nom, but the types work out nicely.
pub fn parse_i32(input: &str) -> IResult<&str, i32> {
  let is_i32_digit = |c: char| "-1234567890".contains(c);
  map_res(take_while(is_i32_digit), |s: &str| s.parse::<i32>())(input)
}

// TODO: perhaps it is time to generalize this.
// It's a little silly that I'm using this rather than the u32 parser provided by nom, but the types work out nicely.
pub fn parse_i64(input: &str) -> IResult<&str, i64> {
  let is_i32_digit = |c: char| "-1234567890".contains(c);
  map_res(take_while(is_i32_digit), |s: &str| s.parse::<i64>())(input)
}

pub fn lines_of<'a, O, E: ParseError<&'a str>, F: Parser<&'a str, O, E>>(
  parser: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>, E> {
  many1(delimited(multispace0, parser, multispace0))
}
