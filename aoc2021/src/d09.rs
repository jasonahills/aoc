use crate::nom_prelude::*;
use itertools::Itertools;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct HeightMap(Vec<Vec<u32>>);

impl HeightMap {
  fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize, u32)> {
    [
      (Some(x + 1), Some(y)),
      (x.checked_sub(1), Some(y)),
      (Some(x), Some(y + 1)),
      (Some(x), y.checked_sub(1)),
    ]
    .into_iter()
    .filter_map(|(x, y)| {
      let x = x?;
      let y = y?;
      Some((x, y, *self.0.get(y)?.get(x)?))
    })
    .collect()
  }

  fn pt_infos<'a>(&'a self) -> impl Iterator<Item = (usize, usize, u32)> + 'a {
    self.0.iter().enumerate().flat_map(|(row_num, r)| {
      r.iter()
        .enumerate()
        .map(move |(col_num, val)| (col_num, row_num, *val))
    })
  }

  fn low_pts<'a>(&'a self) -> impl Iterator<Item = (usize, usize, u32)> + 'a {
    self.pt_infos().filter(|(x, y, val)| {
      self
        .neighbors(*x, *y)
        .into_iter()
        .all(|(_, _, neighbor_val)| neighbor_val > *val)
    })
  }

  fn basin_step(&self, used: &mut HashSet<(usize, usize)>, x: usize, y: usize) -> u32 {
    let mut count = 0;
    for (x, y, val) in self.neighbors(x, y) {
      if val != 9 && !used.contains(&(x, y)) {
        used.insert((x, y));
        count += 1 + self.basin_step(used, x, y)
      }
    }
    count
  }
}

pub fn parse(input: &str) -> IResult<&str, HeightMap> {
  let line = many1(map_parser(take(1_u32), parse_u32));
  map(many1(delimited(multispace0, line, multispace0)), HeightMap)(input)
}

pub fn p1(input: HeightMap) -> u32 {
  input.low_pts().map(|(_, _, val)| val + 1).sum()
}

pub fn p2(input: HeightMap) -> u32 {
  // The way things are defined, there is a bijection between low points and basins,
  // and the basins are divided by 9s.
  let mut used = HashSet::new();
  // No need to mark the origin point; as long as we have three basins of size more than 1, it will get marked by `basin_step`.
  input
    .low_pts()
    .into_iter()
    .map(|(x, y, _)| input.basin_step(&mut used, x, y))
    .sorted()
    .rev()
    .take(3)
    .fold(1, |acc, size| acc * size)
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "2199943210
  3987894921
  9856789892
  8767896789
  9899965678";

  #[test]
  fn test_parse() {
    let input = "123
    456";
    assert_eq!(
      parse(input).unwrap(),
      ("", HeightMap(vec![vec![1, 2, 3], vec![4, 5, 6]]))
    )
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 15);

    let input = std::fs::read_to_string("./inputs/d09.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 491);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 1134);

    let input = std::fs::read_to_string("./inputs/d09.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 0);
  }
}
