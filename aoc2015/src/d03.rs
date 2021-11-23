use std::collections::HashSet;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn direction(input: &str) -> IResult<&str, Direction> {
  let up = map_res(tag("^"), |_| Ok::<Direction, ()>(Direction::Up));
  let down = map_res(tag("v"), |_| Ok::<Direction, ()>(Direction::Down));
  let left = map_res(tag("<"), |_| Ok::<Direction, ()>(Direction::Left));
  let right = map_res(tag(">"), |_| Ok::<Direction, ()>(Direction::Right));
  alt((up, down, left, right))(input)
}

pub fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
  many1(direction)(input)
}

pub fn p1(input: Vec<Direction>) -> u32 {
  let mut loc = (0, 0);
  let mut delivered = HashSet::new();
  delivered.insert(loc.clone());
  for d in input {
    match d {
      Direction::Up => loc.1 += 1,
      Direction::Down => loc.1 -= 1,
      Direction::Left => loc.0 -= 1,
      Direction::Right => loc.0 += 1,
    }
    delivered.insert(loc.clone());
  }
  delivered.len() as u32
}

pub fn p2(input: Vec<Direction>) -> u32 {
  let mut santa = (0, 0);
  let mut robo = (0, 0);
  let mut delivered = HashSet::new();
  delivered.insert(santa.clone());
  delivered.insert(robo.clone());
  // alternatively could have just split the list, etc.
  for (i, d) in input.iter().enumerate() {
    let santa_step = if i % 2 == 0 { 1 } else { 0 };
    let robo_step = 1 - santa_step;
    match d {
      Direction::Up => {
        santa.1 += santa_step;
        robo.1 += robo_step;
      }
      Direction::Down => {
        santa.1 -= santa_step;
        robo.1 -= robo_step;
      }
      Direction::Left => {
        santa.0 -= santa_step;
        robo.0 -= robo_step;
      }
      Direction::Right => {
        santa.0 += santa_step;
        robo.0 += robo_step;
      }
    }
    delivered.insert(santa.clone());
    delivered.insert(robo.clone());
  }
  delivered.len() as u32
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse() {
    use super::Direction::*;
    assert_eq!(
      directions("^v<>>").unwrap(),
      ("", vec!(Up, Down, Left, Right, Right))
    );
  }

  #[test]
  fn p1() {
    let input = "^>v<<>";
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p1(ds), 5);

    let input = std::fs::read_to_string("./inputs/d03.txt").unwrap();
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p1(ds), 2081);
  }

  #[test]
  fn p2() {
    let input = "^>v<<>";
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p2(ds), 4);

    let input = std::fs::read_to_string("./inputs/d03.txt").unwrap();
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p2(ds), 2341);
  }
}
