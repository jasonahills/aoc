use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
  Forward(u32),
  Up(u32),
  Down(u32),
}

fn direction<'a, F: Fn(u32) -> Direction>(
  dir_tag: &'static str,
  constructor: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Direction> {
  preceded(
    tag(dir_tag),
    map(map_res(digit1, |s: &str| s.parse::<u32>()), constructor),
  )
}

pub fn parse(input: &str) -> IResult<&str, Vec<Direction>> {
  let forward = direction("forward ", Direction::Forward);
  let down = direction("down ", Direction::Down);
  let up = direction("up ", Direction::Up);
  many1(delimited(
    multispace0,
    alt((forward, down, up)),
    multispace0,
  ))(input)
}

pub fn p1(directions: Vec<Direction>) -> usize {
  let (x, y) = directions
    .into_iter()
    .fold((0_i32, 0_i32), |(x, y), d| match d {
      Direction::Forward(n) => (x + n as i32, y),
      Direction::Up(n) => (x, y - n as i32),
      Direction::Down(n) => (x, y + n as i32),
    });
  (x * y) as usize
}

pub fn p2(directions: Vec<Direction>) -> usize {
  let (x, y, _aim) = directions
    .into_iter()
    .fold((0_i32, 0_i32, 0_i32), |(x, y, aim), d| match d {
      Direction::Forward(n) => (x + n as i32, y + (aim * n as i32), aim),
      Direction::Up(n) => (x, y, aim - n as i32),
      Direction::Down(n) => (x, y, aim + n as i32),
    });
  (x * y) as usize
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    use Direction::*;
    let input = "forward 5
    down 5
    up 3";
    assert_eq!(
      parse(input).unwrap(),
      ("", vec![Forward(5), Down(5), Up(3)])
    )
  }

  #[test]
  fn test_p1() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";
    let ds = parse(input).unwrap().1;
    assert_eq!(p1(ds), 150);

    let input = std::fs::read_to_string("./inputs/d02.txt").unwrap();
    let ds = parse(&input).unwrap().1;
    assert_eq!(p1(ds), 2091984);
  }

  #[test]
  fn test_p2() {
    let input = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";
    let ds = parse(input).unwrap().1;
    assert_eq!(p2(ds), 900);

    let input = std::fs::read_to_string("./inputs/d02.txt").unwrap();
    let ds = parse(&input).unwrap().1;
    assert_eq!(p2(ds), 2086261056);
  }
}
