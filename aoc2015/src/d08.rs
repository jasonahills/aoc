use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, none_of};
use nom::combinator::map;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct StringLiteral {
  chars: Vec<Char>,
}

impl StringLiteral {
  fn literal_bytes(&self) -> u32 {
    self.chars.iter().map(Char::literal_bytes).sum::<u32>() + 2
  }

  fn memory_bytes(&self) -> u32 {
    self.chars.len() as u32
  }

  /// The number of bytes needed to escape the literal representation of this string
  fn escaped_bytes(&self) -> u32 {
    self.chars.iter().map(Char::escaped_bytes).sum::<u32>() + 6
  }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Char {
  Backslash,
  Quote,
  Hex(char, char),
  Char(char),
}

impl Char {
  fn literal_bytes(&self) -> u32 {
    match self {
      Char::Backslash => 2,
      Char::Quote => 2,
      Char::Hex(_, _) => 4,
      Char::Char(_) => 1,
    }
  }

  fn escaped_bytes(&self) -> u32 {
    match self {
      Char::Backslash => 4,
      Char::Quote => 4,
      Char::Hex(_, _) => 5,
      Char::Char(_) => 1,
    }
  }
}

pub fn p1(input: &[StringLiteral]) -> u32 {
  let lit = input.iter().map(StringLiteral::literal_bytes).sum::<u32>();
  let mem = input.iter().map(StringLiteral::memory_bytes).sum::<u32>();
  lit - mem
}

pub fn p2(input: &[StringLiteral]) -> u32 {
  let escaped = input.iter().map(StringLiteral::escaped_bytes).sum::<u32>();
  let lit = input.iter().map(StringLiteral::literal_bytes).sum::<u32>();
  escaped - lit
}

fn quote(input: &str) -> IResult<&str, Char> {
  map(tag(r#"\""#), |_| Char::Quote)(input)
}

fn backslash(input: &str) -> IResult<&str, Char> {
  map(tag(r#"\\"#), |_| Char::Backslash)(input)
}

fn hex(input: &str) -> IResult<&str, Char> {
  let (input, (_, c1, c2)) = tuple((tag(r#"\x"#), anychar, anychar))(input)?;
  Ok((input, Char::Hex(c1, c2)))
}

// Technically I should exclude the backslash character, but since `alt` "falls through", this is fine.
fn char_char(input: &str) -> IResult<&str, Char> {
  map(none_of(r#""\"#), |c| Char::Char(c))(input)
}

fn char_p(input: &str) -> IResult<&str, Char> {
  alt((quote, backslash, hex, char_char))(input)
}

fn string_literal(input: &str) -> IResult<&str, StringLiteral> {
  let (input, chars) = delimited(tag(r#"""#), many1(char_p), tag(r#"""#))(input)?;
  Ok((input, StringLiteral { chars }))
}

pub fn string_literals(input: &str) -> IResult<&str, Vec<StringLiteral>> {
  many1(delimited(
    many0(tag("\n")),
    string_literal,
    many0(tag("\n")),
  ))(input)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse() {
    use super::Char::*;
    let input = r#""a\\\"\xaaa""#;
    assert_eq!(char_char("a").unwrap(), ("", Char('a')));
    assert_eq!(char_char("a").unwrap(), ("", Char('a')));
    assert_eq!(
      string_literals(input).unwrap(),
      (
        "",
        vec![StringLiteral {
          chars: vec![Char('a'), Backslash, Quote, Hex('a', 'a'), Char('a')],
        }],
      ),
    );
  }

  #[test]
  fn p1() {
    let input = std::fs::read_to_string("./inputs/d08.txt").unwrap();
    let (_, input) = string_literals(&input).unwrap();
    assert_eq!(super::p1(&input), 1333);
  }

  #[test]
  fn p2() {
    let input = std::fs::read_to_string("./inputs/d08.txt").unwrap();
    let (_, input) = string_literals(&input).unwrap();
    assert_eq!(super::p2(&input), 2046);
  }
}
