use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::map_res;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::collections::HashMap;

type Id = String;

#[derive(Debug, Eq, PartialEq)]
pub enum IdOrVal {
  Id(Id),
  Val(u16),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
  And {
    from1: IdOrVal,
    from2: IdOrVal,
    to: Id,
  },
  Or {
    from1: IdOrVal,
    from2: IdOrVal,
    to: Id,
  },
  Not {
    from: IdOrVal,
    to: Id,
  },
  RShift {
    from: IdOrVal,
    shift: usize,
    to: Id,
  },
  LShift {
    from: IdOrVal,
    shift: usize,
    to: Id,
  },
  Assign {
    from: IdOrVal,
    to: Id,
  },
}

impl Instruction {
  fn to(&self) -> Id {
    match self {
      Instruction::And { to, .. } => to.clone(),
      Instruction::Or { to, .. } => to.clone(),
      Instruction::Not { to, .. } => to.clone(),
      Instruction::RShift { to, .. } => to.clone(),
      Instruction::LShift { to, .. } => to.clone(),
      Instruction::Assign { to, .. } => to.clone(),
    }
  }
}

fn identifier(input: &str) -> IResult<&str, Id> {
  map_res(alpha1, |s: &str| Ok::<Id, ()>(s.to_string()))(input)
}

// TODO: is there an easy way to get `nom::number::complete::u16` to work for me.  The type checker is having trouble with the error type on it in the `tuple` combinator.
fn u16p(input: &str) -> IResult<&str, u16> {
  map_res(digit1, |s: &str| s.parse::<u16>())(input)
}

fn id_or_val_id(input: &str) -> IResult<&str, IdOrVal> {
  let (input, i) = identifier(input)?;
  Ok((input, IdOrVal::Id(i)))
}

fn id_or_val_val(input: &str) -> IResult<&str, IdOrVal> {
  let (input, v) = u16p(input)?;
  Ok((input, IdOrVal::Val(v)))
}

fn id_or_val(input: &str) -> IResult<&str, IdOrVal> {
  alt((id_or_val_id, id_or_val_val))(input)
}

fn or(input: &str) -> IResult<&str, Instruction> {
  let (input, (from1, _, from2, _, to)) =
    tuple((id_or_val, tag(" OR "), id_or_val, tag(" -> "), identifier))(input)?;
  Ok((input, Instruction::Or { from1, from2, to }))
}

fn and(input: &str) -> IResult<&str, Instruction> {
  let (input, (from1, _, from2, _, to)) =
    tuple((id_or_val, tag(" AND "), id_or_val, tag(" -> "), identifier))(input)?;
  Ok((input, Instruction::And { from1, from2, to }))
}

fn not(input: &str) -> IResult<&str, Instruction> {
  let (input, (_, from, _, to)) = tuple((tag("NOT "), id_or_val, tag(" -> "), identifier))(input)?;
  Ok((input, Instruction::Not { from, to }))
}

fn rshift(input: &str) -> IResult<&str, Instruction> {
  let (input, (from, _, shift, _, to)) =
    tuple((id_or_val, tag(" RSHIFT "), u16p, tag(" -> "), identifier))(input)?;
  let shift = shift as usize;
  Ok((input, Instruction::RShift { from, shift, to }))
}

fn lshift(input: &str) -> IResult<&str, Instruction> {
  let (input, (from, _, shift, _, to)) =
    tuple((id_or_val, tag(" LSHIFT "), u16p, tag(" -> "), identifier))(input)?;
  let shift = shift as usize;
  Ok((input, Instruction::LShift { from, shift, to }))
}

fn assign(input: &str) -> IResult<&str, Instruction> {
  let (input, (from, _, to)) = tuple((id_or_val, tag(" -> "), identifier))(input)?;
  Ok((input, Instruction::Assign { from, to }))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
  alt((and, or, not, rshift, lshift, assign))(input)
}

pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
  // TODO: there must be a better way to do this.
  let delimited_instruction = delimited(many0(tag("\n")), instruction, many0(tag("\n")));
  many1(delimited_instruction)(input)
}

/// Recursive eval with memoization
fn eval(insts: &HashMap<Id, Instruction>, memos: &mut HashMap<Id, u16>, id: &Id) -> u16 {
  if let Some(val) = memos.get(id) {
    return *val;
  }
  let inst = insts.get(id).unwrap();
  let val = match inst {
    Instruction::And { from1, from2, .. } => {
      eval_id_or_val(insts, memos, from1) & eval_id_or_val(insts, memos, from2)
    }
    Instruction::Or { from1, from2, .. } => {
      eval_id_or_val(insts, memos, from1) | eval_id_or_val(insts, memos, from2)
    }
    Instruction::Not { from, .. } => !eval_id_or_val(insts, memos, from),
    Instruction::RShift { from, shift, .. } => eval_id_or_val(insts, memos, from) >> shift,
    Instruction::LShift { from, shift, .. } => eval_id_or_val(insts, memos, from) << shift,
    Instruction::Assign { from, .. } => eval_id_or_val(insts, memos, from),
  };
  memos.insert(id.clone(), val);
  val
}

fn eval_id_or_val(
  insts: &HashMap<Id, Instruction>,
  memos: &mut HashMap<Id, u16>,
  id_or_val: &IdOrVal,
) -> u16 {
  match id_or_val {
    IdOrVal::Id(id) => eval(insts, memos, id),
    IdOrVal::Val(val) => *val,
  }
}

pub fn p1(input: Vec<Instruction>) -> u16 {
  let insts = input
    .into_iter()
    .map(|i| (i.to(), i))
    .collect::<HashMap<_, _>>();
  let mut memos = HashMap::new();
  eval(&insts, &mut memos, &"a".to_string())
}

pub fn p2(input: Vec<Instruction>) -> u16 {
  let insts = input
    .into_iter()
    .map(|i| (i.to(), i))
    .collect::<HashMap<_, _>>();
  let mut memos = HashMap::new();
  memos.insert("b".to_string(), 3176);
  eval(&insts, &mut memos, &"a".to_string())
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn parse() {
    let input = "123 -> a
a -> c
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT y -> i";
    assert_eq!(
      instructions(input).unwrap(),
      (
        "",
        vec![
          Instruction::Assign {
            from: IdOrVal::Val(123),
            to: "a".to_string(),
          },
          Instruction::Assign {
            from: IdOrVal::Id("a".to_string()),
            to: "c".to_string(),
          },
          Instruction::And {
            from1: IdOrVal::Id("x".to_string()),
            from2: IdOrVal::Id("y".to_string()),
            to: "d".to_string(),
          },
          Instruction::Or {
            from1: IdOrVal::Id("x".to_string()),
            from2: IdOrVal::Id("y".to_string()),
            to: "e".to_string(),
          },
          Instruction::LShift {
            from: IdOrVal::Id("x".to_string()),
            shift: 2,
            to: "f".to_string(),
          },
          Instruction::RShift {
            from: IdOrVal::Id("y".to_string()),
            shift: 2,
            to: "g".to_string(),
          },
          Instruction::Not {
            from: IdOrVal::Id("y".to_string()),
            to: "i".to_string(),
          },
        ],
      ),
    )
  }

  #[test]
  fn p1() {
    let input = "1 -> a
a LSHIFT 1 -> b
a OR b -> c
a AND c -> d
NOT a -> e
b RSHIFT 1 -> f
a -> g";

    let (_, input) = instructions(input).unwrap();

    let insts = input
      .into_iter()
      .map(|i| (i.to(), i))
      .collect::<HashMap<_, _>>();
    let mut memos = HashMap::new();
    assert_eq!(eval(&insts, &mut memos, &"a".to_string()), 1);
    assert_eq!(eval(&insts, &mut memos, &"b".to_string()), 2);
    assert_eq!(eval(&insts, &mut memos, &"c".to_string()), 3);
    assert_eq!(eval(&insts, &mut memos, &"d".to_string()), 1);
    assert_eq!(eval(&insts, &mut memos, &"e".to_string()), 65534);
    assert_eq!(eval(&insts, &mut memos, &"f".to_string()), 1);
    assert_eq!(eval(&insts, &mut memos, &"g".to_string()), 1);

    let input = std::fs::read_to_string("./inputs/d07.txt").unwrap();
    let (_, input) = instructions(&input).unwrap();

    assert_eq!(super::p1(input), 3176);
  }

  #[test]
  fn p2() {
    let input = std::fs::read_to_string("./inputs/d07.txt").unwrap();
    let (_, input) = instructions(&input).unwrap();

    assert_eq!(super::p2(input), 14710);
  }
}
