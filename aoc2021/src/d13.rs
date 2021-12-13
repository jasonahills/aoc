//! https://adventofcode.com/2021/day/13

use crate::nom_prelude::*;
use std::collections::HashSet;

type Point = (i32, i32);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FoldInstruction {
  Up(i32),
  Left(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Instructions {
  points: Vec<Point>,
  folds: Vec<FoldInstruction>,
}

pub struct DisplayGrid(HashSet<Point>);

impl std::fmt::Display for DisplayGrid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let max_x = self.0.iter().map(|(x, _)| x).max().unwrap();
    let max_y = self.0.iter().map(|(_, y)| y).max().unwrap();
    for y in 0..=(*max_y) {
      for x in 0..=(*max_x) {
        if self.0.contains(&(x, y)) {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

fn fold_points(pts: HashSet<Point>, fold_instruction: FoldInstruction) -> HashSet<Point> {
  pts.into_iter().map(|(x, y)| {
    match fold_instruction {
      FoldInstruction::Up(u) => (x, u - (u - y).abs()),
      FoldInstruction::Left(u) => (u - (u - x).abs(), y),
    }
  }).collect()
}

pub fn parse(input: &str) -> IResult<&str, Instructions> {
  let point = separated_pair(parse_i32, tag(","), parse_i32);
  let fold_instruction = map(tuple((alt((tag("fold along x="), tag("fold along y="))), parse_i32)), |(s, u)| {
    match s {
      "fold along x=" => FoldInstruction::Left(u),
      "fold along y=" => FoldInstruction::Up(u),
      _ => panic!("incorrect tag"),
    }
  });
  map(tuple((lines_of(point), lines_of(fold_instruction))), |(points, folds)| {
    Instructions { points, folds }
  })(input)
}

pub fn p1(instructions: Instructions) -> usize {
  let points = instructions.points.into_iter().collect();
  let instruction = instructions.folds.first().unwrap();
  fold_points(points, *instruction).len()
}

pub fn p2(instructions: Instructions) -> String {
  let mut points = instructions.points.into_iter().collect();
  for fold in instructions.folds {
    points = fold_points(points, fold);
  }
  format!("{}", DisplayGrid(points))
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "6,10
  0,14
  9,10
  0,3
  10,4
  4,11
  6,0
  6,12
  4,1
  0,13
  10,12
  3,4
  3,0
  8,4
  1,10
  2,14
  8,10
  9,0
  
  fold along y=7
  fold along x=5";

  #[test]
  fn test_parse() {
    let input = "6,10
    0,14
    
    fold along y=7
    fold along x=2";
    assert_eq!(parse(input).unwrap(), ("", Instructions {
      points: vec![(6,10), (0,14)], folds: vec![FoldInstruction::Up(7), FoldInstruction::Left(2)],
    }))
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 17);

    let input = std::fs::read_to_string("./inputs/d13.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 638);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), "#####
#...#
#...#
#...#
#####
");

    let input = std::fs::read_to_string("./inputs/d13.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), ".##....##..##..#..#.###...##..###..###.
#..#....#.#..#.#.#..#..#.#..#.#..#.#..#
#.......#.#....##...###..#..#.#..#.###.
#.......#.#....#.#..#..#.####.###..#..#
#..#.#..#.#..#.#.#..#..#.#..#.#....#..#
.##...##...##..#..#.###..#..#.#....###.");
  }
}
