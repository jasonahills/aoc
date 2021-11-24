use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Copy, Clone)]
pub struct Point {
  x: u32,
  y: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Rect {
  xmin: u32,
  ymin: u32,
  xmax: u32,
  ymax: u32,
}

impl Rect {
  fn values(&self) -> impl Iterator<Item = Point> {
    (self.xmin..=self.xmax)
      .cartesian_product(self.ymin..=self.ymax)
      .map(|(x, y)| Point { x, y })
  }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
  TurnOn(Rect),
  TurnOff(Rect),
  Toggle(Rect),
}

fn point(input: &str) -> IResult<&str, Point> {
  let mut parse_dimension = map_res(digit1, |s: &str| s.parse::<u32>());
  let (input, x) = parse_dimension(input)?;
  let (input, _) = tag(",")(input)?;
  let (input, y) = parse_dimension(input)?;
  Ok((input, Point { x, y }))
}

fn rect(input: &str) -> IResult<&str, Rect> {
  let (input, (p1, _, p2)) = tuple((point, tag(" through "), point))(input)?;
  Ok((
    input,
    Rect {
      xmin: p1.x,
      ymin: p1.y,
      xmax: p2.x,
      ymax: p2.y,
    },
  ))
}

fn turn_on(input: &str) -> IResult<&str, Instruction> {
  let (input, r) = preceded(tag("turn on "), rect)(input)?;
  Ok((input, Instruction::TurnOn(r)))
}

fn turn_off(input: &str) -> IResult<&str, Instruction> {
  let (input, r) = preceded(tag("turn off "), rect)(input)?;
  Ok((input, Instruction::TurnOff(r)))
}

fn toggle(input: &str) -> IResult<&str, Instruction> {
  let (input, r) = preceded(tag("toggle "), rect)(input)?;
  Ok((input, Instruction::Toggle(r)))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
  alt((turn_on, turn_off, toggle))(input)
}

pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
  // TODO: there must be a better way to do this.
  let delimited_instruction = delimited(many0(tag("\n")), instruction, many0(tag("\n")));
  many1(delimited_instruction)(input)
}

struct LightGrid {
  // We're doing this the naive way. lol
  lights: HashSet<Point>,
}

impl LightGrid {
  fn new() -> Self {
    Self {
      lights: HashSet::new(),
    }
  }

  fn update(&mut self, instruction: Instruction) {
    match instruction {
      Instruction::TurnOn(r) => r.values().for_each(|v| {
        self.lights.insert(v);
      }),
      Instruction::TurnOff(r) => r.values().for_each(|v| {
        self.lights.remove(&v);
      }),
      Instruction::Toggle(r) => r.values().for_each(|v| {
        if !self.lights.remove(&v) {
          self.lights.insert(v);
        }
      }),
    }
  }
}

pub fn p1(instructions: Vec<Instruction>) -> usize {
  let mut lights = LightGrid::new();
  for i in instructions {
    lights.update(i)
  }
  lights.lights.iter().count()
}

struct DimmableLightGrid {
  lights: HashMap<Point, i32>,
}

impl DimmableLightGrid {
  fn new() -> Self {
    Self {
      lights: HashMap::new(),
    }
  }

  fn update(&mut self, instruction: Instruction) {
    let (step, r) = match instruction {
      Instruction::TurnOn(r) => (1, r),
      Instruction::TurnOff(r) => (-1, r),
      Instruction::Toggle(r) => (2, r),
    };
    r.values().for_each(|v| {
      let entry = self.lights.entry(v).or_insert(0);
      *entry = (*entry + step).max(0);
    })
  }
}

pub fn p2(instructions: Vec<Instruction>) -> u32 {
  let mut lights = DimmableLightGrid::new();
  for i in instructions {
    lights.update(i)
  }
  lights.lights.values().copied().sum::<i32>() as u32
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn parse() {
    let input = "turn off 660,55 through 986,197
toggle 322,558 through 977,958
turn on 317,329 through 451,798";
    assert_eq!(
      instructions(input).unwrap(),
      (
        "",
        vec![
          Instruction::TurnOff(Rect {
            xmin: 660,
            ymin: 55,
            xmax: 986,
            ymax: 197,
          }),
          Instruction::Toggle(Rect {
            xmin: 322,
            ymin: 558,
            xmax: 977,
            ymax: 958,
          }),
          Instruction::TurnOn(Rect {
            xmin: 317,
            ymin: 329,
            xmax: 451,
            ymax: 798,
          }),
        ]
      )
    );
  }

  #[test]
  fn p1() {
    let input = "turn on 1,2 through 3,4
turn off 3,4 through 5,6
toggle 2,3 through 4,5
";
    let (_, xs) = instructions(&input).unwrap();
    assert_eq!(super::p1(xs), 11);

    let input = std::fs::read_to_string("./inputs/d06.txt").unwrap();
    let (_, xs) = instructions(&input).unwrap();
    assert_eq!(super::p1(xs), 400410);
  }

  #[test]
  fn p2() {
    let input = "turn on 1,2 through 3,4
turn off 3,4 through 5,6
toggle 2,3 through 4,5
";
    let (_, xs) = instructions(&input).unwrap();
    assert_eq!(super::p2(xs), 26);

    let input = std::fs::read_to_string("./inputs/d06.txt").unwrap();
    let (_, xs) = instructions(&input).unwrap();
    assert_eq!(super::p2(xs), 15343601);
  }
}
