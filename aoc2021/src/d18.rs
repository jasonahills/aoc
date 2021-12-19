//! https://adventofcode.com/2021/day/18

use itertools::Itertools;

use crate::nom_prelude::*;

type Input = Vec<SnailFishNum>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SnailFishNum {
  Regular(u32),
  Pair(Box<SnailFishNum>, Box<SnailFishNum>),
}

impl SnailFishNum {
  fn magnitude(&self) -> u32 {
    match self {
      Self::Regular(u) => *u,
      Self::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
    }
  }

  fn extract_val(&self) -> Option<u32> {
    if let Self::Regular(u) = self {
      Some(*u)
    } else {
      None
    }
  }

  fn add_left(&mut self, u: u32) {
    match self {
      Self::Regular(old) => *self = Self::Regular(*old + u),
      Self::Pair(left, _) => left.add_left(u),
    }
  }

  fn add_right(&mut self, u: u32) {
    match self {
      Self::Regular(old) => *self = Self::Regular(*old + u),
      Self::Pair(_, right) => right.add_right(u),
    }
  }

  /// Returns the left and right numbers if we still need to explode left, right respectively; returns true if explode occurred.
  pub fn explode(&mut self, depth: u8) -> (Option<u32>, Option<u32>, bool) {
    match self {
      Self::Regular(_) => (None, None, false),
      Self::Pair(left, right) => {
        if depth == 4 {
          // If I ever get something with depth greater than 4 this won't work right.
          // But we only increase depth when adding, so this should be fine.
          let left = left.extract_val();
          let right = right.extract_val();
          *self = Self::Regular(0);
          return (left, right, true);
        }

        let (left_explode, right_explode, exploded) = left.explode(depth + 1);

        if let Some(u) = right_explode {
          right.add_left(u);
        }
        if exploded {
          return (left_explode, None, true);
        }

        let (left_explode, right_explode, exploded) = right.explode(depth + 1);

        if let Some(u) = left_explode {
          left.add_right(u);
        }
        (None, right_explode, exploded)
      }
    }
  }

  /// Returns true if a split occurred
  pub fn split(&mut self) -> bool {
    match self {
      Self::Regular(u) if *u < 10 => false,
      Self::Regular(u) => {
        let left = *u / 2;
        let right = *u - left;
        *self = Self::Pair(
          Box::new(Self::Regular(left)),
          Box::new(Self::Regular(right)),
        );
        true
      }
      Self::Pair(left, right) => left.split() || right.split(),
    }
  }

  pub fn reduce(&mut self) {
    loop {
      let (_, _, reduced) = self.explode(0);
      if reduced || self.split() {
        continue;
      }
      break;
    }
  }
}

impl std::ops::Add for SnailFishNum {
  type Output = SnailFishNum;
  fn add(self, rhs: Self) -> Self::Output {
    let mut n = SnailFishNum::Pair(Box::new(self), Box::new(rhs));
    n.reduce();
    n
  }
}

impl std::fmt::Display for SnailFishNum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SnailFishNum::Regular(u) => write!(f, "{}", u)?,
      SnailFishNum::Pair(n1, n2) => write!(f, "[{},{}]", n1, n2)?,
    }
    Ok(())
  }
}

pub fn parse_regular(input: &str) -> IResult<&str, SnailFishNum> {
  map(parse_u32, SnailFishNum::Regular)(input)
}

pub fn parse_pair(input: &str) -> IResult<&str, SnailFishNum> {
  map(
    delimited(
      tag("["),
      separated_pair(parse_snail_fish_num, tag(","), parse_snail_fish_num),
      tag("]"),
    ),
    |(n1, n2)| SnailFishNum::Pair(Box::new(n1), Box::new(n2)),
  )(input)
}

pub fn parse_snail_fish_num(input: &str) -> IResult<&str, SnailFishNum> {
  alt((parse_regular, parse_pair))(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  lines_of(parse_snail_fish_num)(input)
}

pub fn p1(input: Input) -> u32 {
  let sum = input.into_iter().reduce(std::ops::Add::add).unwrap();
  sum.magnitude()
}

pub fn p2(input: Input) -> u32 {
  input
    .iter()
    .cartesian_product(input.iter())
    .map(|(x, y)| (x.clone() + y.clone()).magnitude())
    .max()
    .unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
  [[[5,[2,8]],4],[5,[[9,9],0]]]
  [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
  [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
  [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
  [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
  [[[[5,4],[7,7]],8],[[8,3],8]]
  [[9,3],[[9,9],[6,[4,9]]]]
  [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
  [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

  #[test]
  fn test_parse() {
    let input = "[1,2]
    [[1,2],3]
    [9,[8,7]]
    [[1,9],[8,5]]
    [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
    [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
    [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
    let parsed = parse(input).unwrap().1;
    for (i, line) in input.lines().enumerate() {
      let line = line.trim();
      assert_eq!(&format!("{}", parsed[i]), line);
    }
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 4140);

    let input = std::fs::read_to_string("./inputs/d18.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 3892);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 3993);

    let input = std::fs::read_to_string("./inputs/d18.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 4909);
  }
}
