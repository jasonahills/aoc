use crate::nom_prelude::parse_u32;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct Line {
  x1: i32,
  y1: i32,
  x2: i32,
  y2: i32,
}

impl Line {
  pub fn is_vertical(&self) -> bool {
    self.x1 == self.x2
  }

  pub fn is_horizontal(&self) -> bool {
    self.y1 == self.y2
  }

  pub fn points(&self) -> Vec<(i32, i32)> {
    let x_step = if self.x1 == self.x2 {
      0
    } else if self.x1 < self.x2 {
      1
    } else {
      -1
    };

    let y_step = if self.y1 == self.y2 {
      0
    } else if self.y1 < self.y2 {
      1
    } else {
      -1
    };

    let num_steps = (self.x1 - self.x2).abs().max((self.y1 - self.y2).abs()) + 1;

    let x1 = self.x1;
    let y1 = self.y1;
    (0..num_steps)
      .map(move |s| (x1 + (s * x_step), y1 + (s * y_step)))
      .collect()
  }
}

pub fn parse(input: &str) -> IResult<&str, Vec<Line>> {
  let line = map(
    tuple((
      parse_u32,
      tag(","),
      parse_u32,
      tag(" -> "),
      parse_u32,
      tag(","),
      parse_u32,
    )),
    |(x1, _, y1, _, x2, _, y2)| Line {
      x1: x1 as i32,
      y1: y1 as i32,
      x2: x2 as i32,
      y2: y2 as i32,
    },
  );
  many1(delimited(multispace0, line, multispace0))(input)
}

// The naïve way, until we find we need more efficiency.
pub fn p1(xs: Vec<Line>) -> usize {
  let mut counter = crate::util::HashCounter::new();
  for line in xs
    .into_iter()
    .filter(|line| line.is_horizontal() || line.is_vertical())
  {
    let xmin = line.x1.min(line.x2);
    let xmax = line.x1.max(line.x2);
    let ymin = line.y1.min(line.y2);
    let ymax = line.y1.max(line.y2);
    for point in (xmin..=xmax).cartesian_product(ymin..=ymax) {
      counter.inc(point);
    }
  }
  counter.iter().filter(|(_, count)| *count > 1).count()
}

pub fn p2(lines: Vec<Line>) -> usize {
  let mut counter = crate::util::HashCounter::new();
  for line in lines {
    for point in line.points().into_iter() {
      counter.inc(point);
    }
  }
  counter.iter().filter(|(_, count)| *count > 1).count()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let input = "0,9 -> 5,9
    8,0 -> 0,8";
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        vec![
          Line {
            x1: 0,
            y1: 9,
            x2: 5,
            y2: 9
          },
          Line {
            x1: 8,
            y1: 0,
            x2: 0,
            y2: 8
          }
        ]
      )
    );
  }

  #[test]
  fn test_p1() {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 5);

    let input = std::fs::read_to_string("./inputs/d05.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 6666);
  }

  #[test]
  fn test_p2() {
    let input = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 12);

    let input = std::fs::read_to_string("./inputs/d05.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 19081);
  }
}
