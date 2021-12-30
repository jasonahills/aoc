//! https://adventofcode.com/2021/day/25

use crate::nom_prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Cucumber {
  East,
  South,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid<T> {
  width: usize,
  height: usize,
  v: Vec<T>,
}

impl<T: Default> Grid<T> {
  pub fn new(width: usize, height: usize) -> Self {
    let v = (0..(width * height)).map(|_| T::default()).collect();
    Self { width, height, v }
  }

  pub fn from_vec(v: Vec<T>, width: usize, height: usize) -> Option<Self> {
    if v.len() == width * height {
      Some(Self { width, height, v })
    } else {
      None
    }
  }

  pub fn at(&self, x: usize, y: usize) -> &T {
    &self.v[(y * self.width) + x]
  }

  pub fn at_mut(&mut self, x: usize, y: usize) -> &mut T {
    &mut self.v[(y * self.width) + x]
  }
}

impl Grid<Option<Cucumber>> {
  /// Returns true if step produced a change.
  fn step(&self) -> (Self, bool) {
    let mut east_grid = Grid::new(self.width, self.height);
    let mut changed = false;
    for x in 0..self.width {
      for y in 0..self.height {
        match self.at(x, y) {
          Some(Cucumber::East) => {
            if self.at((x + 1) % self.width, y).is_none() {
              *east_grid.at_mut((x + 1) % self.width, y) = Some(Cucumber::East);
              changed = true;
            } else {
              *east_grid.at_mut(x, y) = Some(Cucumber::East);
            }
          }
          Some(Cucumber::South) => {
            *east_grid.at_mut(x, y) = Some(Cucumber::South);
          }
          _ => (),
        }
      }
    }
    let mut new = Grid::new(east_grid.width, east_grid.height);
    for x in 0..east_grid.width {
      for y in 0..east_grid.height {
        match east_grid.at(x, y) {
          Some(Cucumber::South) => {
            if east_grid.at(x, (y + 1) % east_grid.height).is_none() {
              *new.at_mut(x, (y + 1) % east_grid.height) = Some(Cucumber::South);
              changed = true;
            } else {
              *new.at_mut(x, y) = Some(Cucumber::South);
            }
          }
          Some(Cucumber::East) => {
            *new.at_mut(x, y) = Some(Cucumber::East);
          }
          _ => (),
        }
      }
    }
    (new, changed)
  }
}

impl std::fmt::Display for Grid<Option<Cucumber>> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (i, c) in self.v.iter().enumerate() {
      let d = match c {
        None => ".",
        Some(Cucumber::East) => ">",
        Some(Cucumber::South) => "v",
      };
      if i % self.width == 0 {
        write!(f, "\n")?;
      }
      write!(f, "{}", d)?;
    }
    Ok(())
  }
}

type Input = Grid<Option<Cucumber>>;

pub fn parse(input: &str) -> IResult<&str, Input> {
  let cucumber = map(one_of(".>v"), |c| match c {
    '.' => None,
    '>' => Some(Cucumber::East),
    'v' => Some(Cucumber::South),
    _ => panic!("non-matching char"),
  });
  map(lines_of(many1(cucumber)), |xss| {
    let height = xss.len();
    let width = xss[0].len();
    let xss = xss.into_iter().flatten().collect();
    Grid::from_vec(xss, width, height).unwrap()
  })(input)
}

pub fn p1(mut input: Input) -> usize {
  let mut stopped = false;
  let mut count = 0;
  while !stopped {
    let (next_input, changed) = input.step();
    input = next_input;
    stopped = !changed;
    count += 1;
  }
  count
}

pub fn p2(_input: Input) -> usize {
  0
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

  #[test]
  fn test_parse() {
    use Cucumber::*;
    let input = ">..
v.v";
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        Grid::from_vec(
          vec![Some(East), None, None, Some(South), None, Some(South)],
          3,
          2
        )
        .unwrap()
      )
    )
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 58);

    let input = std::fs::read_to_string("./inputs/d25.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 429);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d25.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 0);
  }
}
