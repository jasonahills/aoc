//! https://adventofcode.com/2021/day/24

use crate::nom_prelude::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Register {
  W,
  X,
  Y,
  Z,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Value {
  Literal(i64),
  Register(Register),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Inst {
  Inp(Register),
  Add(Register, Value),
  Mul(Register, Value),
  Div(Register, Value),
  Mod(Register, Value),
  Eql(Register, Value),
}

type Input = Vec<Inst>;

pub struct Alu {
  w: i64,
  x: i64,
  y: i64,
  z: i64,
}

impl Alu {
  pub fn new() -> Self {
    Self {
      w: 0,
      x: 0,
      y: 0,
      z: 0,
    }
  }

  pub fn run_program(&mut self, insts: &[Inst], mut inputs: Vec<i64>) {
    inputs.reverse();
    for inst in insts {
      match *inst {
        Inst::Inp(r) => self.write_register(r, inputs.pop().unwrap()),
        Inst::Add(r, v) => self.write_register(r, self.read_register(r) + self.get_value(v)),
        Inst::Mul(r, v) => self.write_register(r, self.read_register(r) * self.get_value(v)),
        Inst::Div(r, v) => self.write_register(r, self.read_register(r) / self.get_value(v)),
        Inst::Mod(r, v) => self.write_register(r, self.read_register(r) % self.get_value(v)),
        Inst::Eql(r, v) => {
          let val = if self.read_register(r) == self.get_value(v) {
            1
          } else {
            0
          };
          self.write_register(r, val)
        }
      }
    }
  }

  fn read_register(&self, r: Register) -> i64 {
    match r {
      Register::W => self.w,
      Register::X => self.x,
      Register::Y => self.y,
      Register::Z => self.z,
    }
  }

  fn write_register(&mut self, r: Register, v: i64) {
    match r {
      Register::W => self.w = v,
      Register::X => self.x = v,
      Register::Y => self.y = v,
      Register::Z => self.z = v,
    }
  }

  fn get_value(&self, value: Value) -> i64 {
    match value {
      Value::Literal(i) => i,
      Value::Register(r) => self.read_register(r),
    }
  }
}

pub fn parse_register(input: &str) -> IResult<&str, Register> {
  map(one_of("wxyz"), |r| match r {
    'w' => Register::W,
    'x' => Register::X,
    'y' => Register::Y,
    'z' => Register::Z,
    _ => panic!("non-matching character"),
  })(input)
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
  let literal = map(parse_i64, Value::Literal);
  let register = map(parse_register, Value::Register);
  alt((literal, register))(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  let unary = map(preceded(tag("inp "), parse_register), Inst::Inp);
  let binary_tag = alt((tag("add"), tag("mul"), tag("div"), tag("mod"), tag("eql")));
  let binary = map(
    tuple((binary_tag, space1, parse_register, space1, parse_value)),
    |(t, _, r, _, v)| match t {
      "add" => Inst::Add(r, v),
      "mul" => Inst::Mul(r, v),
      "div" => Inst::Div(r, v),
      "mod" => Inst::Mod(r, v),
      "eql" => Inst::Eql(r, v),
      _ => panic!("non-matching tag"),
    },
  );
  let inst = alt((unary, binary));
  lines_of(inst)(input)
}

// Ok, the naïve approach isn't going to cut it here. Maybe I should get a way to prove particular instructions are unneeded.  Or maybe I express z in terms of the original inputs
pub fn p1(insts: Input) -> u64 {
  // No need to left pad, as that would add a zero digit.
  let mut n = 99999999999999_u64;
  loop {
    let inputs: Vec<i64> = n
      .to_string()
      .split("")
      .filter(|s| !s.is_empty())
      .map(|d| d.parse::<i64>().unwrap())
      .collect();
    if inputs.iter().any(|v| *v == 0) {
      continue;
    }
    let mut alu = Alu::new();
    alu.run_program(&insts, inputs);
    if alu.z == 0 {
      return n;
    }
    n -= 1;
  }
}

pub fn p2(_input: Input) -> usize {
  0
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "inp w
  add z w
  mod z 2
  div w 2
  add y w
  mod y 2
  div w 2
  add x w
  mod x 2
  div w 2
  mod w 2";

  #[test]
  fn test_parse() {
    let input = "inp w
    add z w
    mod z 2
    div w -2";
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        vec![
          Inst::Inp(Register::W),
          Inst::Add(Register::Z, Value::Register(Register::W)),
          Inst::Mod(Register::Z, Value::Literal(2)),
          Inst::Div(Register::W, Value::Literal(-2)),
        ]
      )
    )
  }

  #[test]
  fn test_alu() {
    let input = "inp x
    mul x -1";
    let insts = parse(input).unwrap().1;
    let mut alu = Alu::new();
    alu.run_program(&insts, vec![8]);
    assert_eq!(alu.x, -8);

    let input = "inp z
    inp x
    mul z 3
    eql z x";
    let insts = parse(input).unwrap().1;
    let mut alu = Alu::new();
    alu.run_program(&insts, vec![3, 9]);
    assert_eq!(alu.z, 1);

    let mut alu = Alu::new();
    alu.run_program(&insts, vec![3, 8]);
    assert_eq!(alu.z, 0);
  }

  #[test]
  fn test_p1() {
    let input = std::fs::read_to_string("./inputs/d24.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 0);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d24.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 0);
  }
}
