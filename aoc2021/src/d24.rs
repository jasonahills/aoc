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

#[derive(Debug, Eq, PartialEq)]
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
      if matches!(inst, Inst::Inp(_)) {
        // println!("inst {:?} alu {:?}", inst, self);
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

fn digits(mut n: i64) -> [i64; 7] {
  let mut to_return = [0; 7];
  for i in 0..7 {
    let d = n % 10;
    n /= 10;
    to_return[6 - i] = d;
  }
  to_return
}

// Ok, the naïve approach isn't going to cut it here. Maybe I should get a way to prove particular instructions are unneeded.  Or maybe I express z in terms of the original inputs
pub fn p1(insts: Input) -> i64 {
  // for my input, positions 3, 5, and 9 through 13 were div by 26 positions.

  // No need to left pad, as that would add a zero digit.
  // the "bigger" steps can never fail to increase 26-fold, so they might as well be 9s.  Brute-forcing is now manageable.
  // let mut n = 9999999_i64;
  let mut n = 9999999_i64;

  // let start = std::time::Instant::now();
  loop {
    let mut inputs = [9; 14];
    let other_inputs = digits(n);
    // only need to try different inputs where we call the "smaller" variant.
    inputs[3] = other_inputs[0];
    inputs[5] = other_inputs[1];
    for i in 0..5 {
      inputs[9 + i] = other_inputs[2 + i];
    }
    n -= 1;
    // println!("inputs {:?}", inputs);
    if n < 0 {
      panic!("tried them all");
    }
    if inputs.iter().any(|v| *v == 0) {
      continue;
    }
    let inputs: [i64; 14] = inputs.clone().try_into().unwrap();
    let out = my_program(inputs.clone());
    if out == 0 {
      return inputs
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, digit)| digit * (10_i64.pow(i as u32)))
        .sum();
    }
  }
}

pub fn p2(_input: Input) -> usize {
  0
}

pub fn bigger(w: i64, v1: i64, v2: i64, mut z: i64) -> i64 {
  // inp w
  // mul x 0
  // add x z
  // mod x 26
  // div z 1
  // add x 15
  // eql x w
  // eql x 0
  // mul y 0
  // add y 25
  // mul y x
  // add y 1
  // mul z y
  // mul y 0
  // add y w
  // add y 15
  // mul y x
  // add z y
  if w != (z % 26) + v1 {
    z *= 26;
  } else {
    // println!("hit this");
  }
  z += v2 + w;
  z
}

pub fn smaller(w: i64, v1: i64, v2: i64, mut z: i64) -> i64 {
  // inp w
  // mul x 0
  // add x z
  // mod x 26
  // div z 26
  // add x -10
  // eql x w
  // eql x 0
  // mul y 0
  // add y 25
  // mul y x
  // add y 1
  // mul z y
  // mul y 0
  // add y w
  // add y 1
  // mul y x
  // add z y
  println!("smaller {}", w);
  let x = z % 26;
  z /= 26;
  if w != x + v1 {
    z *= 26;
    z += v2 + w + 1;
  } else {
    println!("and hit this");
    panic!("I want to hit this");
  }
  z
}

pub fn my_program(xs: [i64; 14]) -> i64 {
  // Think of encoding a base 26 number.  Each time we go "bigger", we multiply by 26 and add our input and the number from our input.
  // Because there are seven bigger steps and seven smaller steps, we need all the smaller steps to not have the mult by 26 step; our life is easier.
  let z = bigger(xs[0], 15, 15, 0); // z = xs[0] + 15
  let z = bigger(xs[1], 12, 5, z); // z = xs[0] + 15 | xs[1] + 5
  let z = bigger(xs[2], 13, 6, z); // z = xs[0] + 15 | xs[1] + 5 | xs[2] + 6
  let z = smaller(xs[3], -14, 7, z); // choose x[3] = xs[2] + 6 - 14, then z = xs[0] + 15 | xs[1] + 5
  let z = bigger(xs[4], 15, 9, z); // z = xs[0] + 15 | xs[1] + 5 | xs[4] + 9
  let z = smaller(xs[5], -7, 6, z); // choose x[5] = xs[4] + 9 - 7, then z = xs[0] + 15 | xs[1] + 5
  let z = bigger(xs[6], 14, 14, z); // z = xs[0] + 15 | xs[1] + 5 | xs[6] + 14
  let z = bigger(xs[7], 15, 3, z); // z = xs[0] + 15 | xs[1] + 5 | xs[6] + 14 | xs[7] + 3
  let z = bigger(xs[8], 15, 1, z); // z = xs[0] + 15 | xs[1] + 5 | xs[6] + 14 | xs[7] + 3 | xs[8] + 1
  let z = smaller(xs[9], -7, 3, z); // choose xs[9] = xs[8] + 1 - 7
  let z = smaller(xs[10], -8, 4, z); // choose xs[10] = xs[7] + 3 - 8
  let z = smaller(xs[11], -7, 6, z); // choose xs[11] = xs[6] + 14 - 7
  let z = smaller(xs[12], -5, 7, z); // choose xs[12] = xs[1] + 5 - 5
  let z = smaller(xs[13], -10, 1, z); // choose xs[13] = xs[0] + 15 - 10

  // constraints
  // x[3] + 8 = xs[2]  => 1, 9
  // x[5] = xs[4] + 2
  // x[9] + 6 =x[8]
  // xs[10] + 5 = xs[7]
  // xs[11] = xs[6] + 7
  // xs[12] = xs[1]
  // xs[13] = xs[0] + 5
  // 49917929934999
  // 11911316711816
  z
}

pub fn my_mini_program(first_input: i64, third_input: i64) -> i64 {
  // let xs = [9, 9, 9];
  let z = 0;
  let z = bigger(first_input, 12, 5, z); // z = xs[0] + 5
  let z = bigger(9, 13, 6, z); // z = xs[1] + 5 | xs[2] + 6
  let z = smaller(third_input, -14, 7, z); // choose xs[3] == xs[1] + 5 - 14
  z
}

pub fn my_mini_program2(x1: i64, x2: i64, x3: i64, x4: i64) -> i64 {
  let z = bigger(x1, 15, 15, 0); // z = xs[0] + 15
  let z = bigger(x2, 12, 5, z); // z = xs[0] + 15 | xs[1] + 5
  let z = bigger(x3, 13, 6, z); // z = xs[0] + 15 | xs[1] + 5 | xs[2] + 6
  let z = smaller(x4, -14, 7, z);
  z
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_mini2() {
    for i in 1..10 {
      for j in 1..10 {
        for k in 1..10 {
          for l in 1..10 {
            my_mini_program2(i, j, k, l);
          }
        }
      }
    }
  }

  #[test]
  fn test_mini() {
    for i in 1..10 {
      for j in 1..10 {
        println!("my mini_program {} {} {}", i, j, my_mini_program(i, j));
      }
    }

    panic!("asf")
  }

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
  fn test_bigger() {
    for i in 1..=9 {
      for j in 1..=9 {
        let b = bigger(i, 15, 15, 0);
        println!("{} {} {} {}", i, j, b, bigger(j, 12, 5, b));
      }
    }
    panic!("asf");
  }

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

    let input = "inp w
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

    let insts = parse(input).unwrap().1;
    let mut alu = Alu::new();
    alu.run_program(&insts, vec![9]);
    assert_eq!(
      alu,
      Alu {
        w: 1,
        x: 0,
        y: 0,
        z: 1
      }
    );

    let mut alu = Alu::new();
    alu.run_program(&insts, vec![7]);
    assert_eq!(
      alu,
      Alu {
        w: 0,
        x: 1,
        y: 1,
        z: 1
      }
    );

    let mut alu = Alu::new();
    alu.run_program(&insts, vec![5]);
    assert_eq!(
      alu,
      Alu {
        w: 0,
        x: 1,
        y: 0,
        z: 1
      }
    );
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
