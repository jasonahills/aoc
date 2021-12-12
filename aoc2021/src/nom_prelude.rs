pub use nom::branch::*;
pub use nom::bytes::complete::*;
pub use nom::character::complete::*;
pub use nom::combinator::*;
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
