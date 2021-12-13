use itertools::Itertools;

use crate::nom_prelude::*;

type Coord = (i8, i8);

#[derive(Debug, Eq, PartialEq)]
pub struct OctoGrid([u8; 100]);

impl OctoGrid {
  fn from_vec(v: Vec<u8>) -> Result<Self, ()> {
    let arr: [u8; 100] = v.try_into().map_err(|_| ())?;
    Ok(Self(arr))
  }

  fn at(&self, (x, y): Coord) -> Option<u8> {
    if 0 <= x && x < 10 && 0 <= y && y < 10 {
      Some(self.0[(y as usize * 10) + x as usize])
    } else {
      None
    }
  }

  fn reset(&mut self, (x, y): Coord) {
    if 0 <= x && x < 10 && 0 <= y && y < 10 {
      self.0[(y as usize * 10) + x as usize] = 0;
    }
  }

  /// Returns true if "flashed".
  fn inc(&mut self, (x, y): Coord) -> bool {
    let mut flashed = false;
    if 0 <= x && x < 10 && 0 <= y && y < 10 {
      self.0[(y as usize * 10) + x as usize] += 1;
      if self.0[(y as usize * 10) + x as usize] == 10 {
        flashed = true;
      }
    }
    flashed
  }

  fn neighbors((x, y): Coord) -> impl Iterator<Item = Coord> {
    [
      (x - 1, y - 1),
      (x, y - 1),
      (x + 1, y - 1),
      (x + 1, y),
      (x + 1, y + 1),
      (x, y + 1),
      (x - 1, y + 1),
      (x - 1, y),
    ]
    .into_iter()
  }

  fn coords() -> impl Iterator<Item = Coord> {
    (0..10).cartesian_product(0..10)
  }

  /// Returns number of flashes
  fn step(&mut self) -> usize {
    let mut flashed = Vec::new();
    for c in Self::coords() {
      // I wanted to do this recursively, but this is easier.
      let mut to_inc = vec![c];
      while !to_inc.is_empty() {
        let c = to_inc.pop().unwrap();
        if self.inc(c) {
          to_inc.extend(Self::neighbors(c));
          flashed.push(c);
        }
      }
    }

    for c in flashed.iter().cloned() {
      self.reset(c);
    }
    flashed.len()
  }
}

pub fn parse(input: &str) -> IResult<&str, OctoGrid> {
  let entry = many1(delimited(
    multispace0,
    map_parser(take(1_u32), parse_u8),
    multispace0,
  ));
  map_res(entry, OctoGrid::from_vec)(input)
}

pub fn p1(mut g: OctoGrid) -> usize {
  let mut sum = 0;
  for _ in 0..100 {
    sum += g.step();
  }
  sum
}

pub fn p2(mut g: OctoGrid) -> usize {
  let mut i = 0;
  loop {
    i += 1;
    if g.step() == 100 {
      return i;
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "5483143223
  2745854711
  5264556173
  6141336146
  6357385478
  4167524645
  2176841721
  6882881134
  4846848554
  5283751526";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(parsed.at((0, 0)), Some(5));
    assert_eq!(parsed.at((9, 0)), Some(3));
    assert_eq!(parsed.at((9, 9)), Some(6));
  }

  #[test]
  fn test_step() {
    let input = TEST_INPUT;
    let mut parsed = parse(input).unwrap().1;

    let after_step_1 = parse(
      "6594254334
    3856965822
    6375667284
    7252447257
    7468496589
    5278635756
    3287952832
    7993992245
    5957959665
    6394862637",
    )
    .unwrap()
    .1;
    parsed.step();
    assert_eq!(parsed, after_step_1);

    let after_step_2 = parse(
      "8807476555
    5089087054
    8597889608
    8485769600
    8700908800
    6600088989
    6800005943
    0000007456
    9000000876
    8700006848",
    )
    .unwrap()
    .1;
    parsed.step();
    assert_eq!(parsed, after_step_2);
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 1656);

    let input = std::fs::read_to_string("./inputs/d11.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 1585);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 195);

    let input = std::fs::read_to_string("./inputs/d11.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 382);
  }
}
