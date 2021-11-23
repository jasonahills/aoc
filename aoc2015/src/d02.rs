use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::sequence::delimited;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct Gift(u32, u32, u32);

impl Gift {
  /// Only returns three of the sides; the corresponding sides can be obtained by repeating these.
  fn side_areas(&self) -> [u32; 3] {
    [self.0 * self.1, self.0 * self.2, self.1 * self.2]
  }

  fn side_perimeters(&self) -> [u32; 3] {
    [
      2 * (self.0 + self.1),
      2 * (self.0 + self.2),
      2 * (self.1 + self.2),
    ]
  }

  fn volume(&self) -> u32 {
    self.0 * self.1 * self.2
  }
}

fn gift(input: &str) -> IResult<&str, Gift> {
  let mut parse_dimension = map_res(digit1, |s: &str| s.parse::<u32>());
  let (input, l) = parse_dimension(input)?;
  let (input, _) = tag("x")(input)?;
  let (input, w) = parse_dimension(input)?;
  let (input, _) = tag("x")(input)?;
  let (input, h) = parse_dimension(input)?;
  Ok((input, Gift(l, w, h)))
}

pub fn gifts(input: &str) -> IResult<&str, Vec<Gift>> {
  // TODO: there must be a better way to do this.
  let delimited_gift = delimited(many0(tag("\n")), gift, many0(tag("\n")));
  many1(delimited_gift)(input)
}

pub fn p1(gifts: Vec<Gift>) -> u32 {
  gifts
    .iter()
    .map(|g| 2 * g.side_areas().iter().sum::<u32>() + g.side_areas().iter().min().unwrap())
    .sum()
}

pub fn p2(gifts: Vec<Gift>) -> u32 {
  gifts
    .iter()
    .map(|g| g.side_perimeters().iter().min().unwrap() + g.volume())
    .sum()
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn parse() {
    let input = "123x456x789
1x2x3";
    assert_eq!(
      gifts(input).unwrap(),
      ("", vec![Gift(123, 456, 789), Gift(1, 2, 3)])
    );
  }

  #[test]
  fn p1() {
    let input = "1x2x3\n
3x2x1\n";
    let (_, gs) = gifts(&input).unwrap();
    assert_eq!(super::p1(gs), (((2 + 3 + 6) * 2) + 2) * 2);

    let input = std::fs::read_to_string("./inputs/d02.txt").unwrap();
    let (_, gs) = gifts(&input).unwrap();
    assert_eq!(super::p1(gs), 1588178);
  }

  #[test]
  fn p2() {
    let input = "1x2x3\n
3x2x1\n";
    let (_, gs) = gifts(&input).unwrap();
    assert_eq!(super::p2(gs), (2 * (1 + 2) + 6) * 2);

    let input = std::fs::read_to_string("./inputs/d02.txt").unwrap();
    let (_, gs) = gifts(&input).unwrap();
    assert_eq!(super::p2(gs), 3783758);
  }
}
