//! https://adventofcode.com/2021/day/00

use crate::nom_prelude::*;

type InputItem = ();

pub fn parse(input: &str) -> IResult<&str, Vec<InputItem>> {
  unimplemented!()
}

pub fn p1(_xs: Vec<InputItem>) -> usize {
  0
}

pub fn p2(_xs: Vec<InputItem>) -> usize {
  0
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "";

  #[test]
  fn test_parse() {
    let input = "";
    assert_eq!(parse(input).unwrap(), ("", vec![]))
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 0);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 0);
  }
}
