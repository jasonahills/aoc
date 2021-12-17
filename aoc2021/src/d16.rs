//! https://adventofcode.com/2021/day/16

use crate::nom_prelude::*;
use Bit::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Bit {
  I,
  O,
}

impl Bit {
  pub fn from_byte_and_offset(u: u8, offset: usize) -> Self {
    if u & 1 << offset != 0 {
      Self::I
    } else {
      Self::O
    }
  }

  pub fn number_from_bits(bits: &[Bit]) -> u64 {
    let mut num = 0;
    for (offset, _) in bits.iter().rev().enumerate().filter(|(_, bit)| **bit == I) {
      num = num | 1 << offset
    }
    num
  }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Packet {
  // TODO: make number a number if we know they will always be small enough
  Literal {
    version: u8,
    number: Vec<Bit>,
  },
  Operation {
    version: u8,
    type_id: TypeId,
    subpackets: Vec<Packet>,
  },
}

#[derive(Debug, Eq, PartialEq)]
pub enum TypeId {
  Add,
  Mul,
  Min,
  Max,
  GT,
  LT,
  Eq,
}

impl TryFrom<u8> for TypeId {
  type Error = ();
  fn try_from(u: u8) -> Result<Self, Self::Error> {
    match u {
      0 => Ok(Self::Add),
      1 => Ok(Self::Mul),
      2 => Ok(Self::Min),
      3 => Ok(Self::Max),
      5 => Ok(Self::GT),
      6 => Ok(Self::LT),
      7 => Ok(Self::Eq),
      _ => Err(()),
    }
  }
}

// TODO: redo all of this in nom.
impl Packet {
  pub fn version(&self) -> u8 {
    match self {
      Self::Literal { version, .. } => *version,
      Self::Operation { version, .. } => *version,
    }
  }

  pub fn from_bits(bits: &[Bit]) -> (Self, usize) {
    let version = Bit::number_from_bits(&bits[0..3]) as u8;
    match Bit::number_from_bits(&bits[3..6]) {
      4 => {
        let mut offset = 6;
        let mut number = Vec::new();
        loop {
          number.append(&mut bits[(offset + 1)..(offset + 5)].to_vec());
          let old_offset = offset;
          offset = offset + 5;
          if bits[old_offset] == O {
            break;
          }
        }
        (Self::Literal { version, number }, offset)
      }
      type_id => {
        let mut offset = 7;
        let mut subpackets = Vec::new();
        let type_id = TypeId::try_from(type_id as u8).unwrap();
        if bits[6] == O {
          // length in bits
          let subpackets_bit_len = Bit::number_from_bits(&bits[offset..offset + 15]);
          offset += 15;
          let final_offset = offset + subpackets_bit_len as usize;
          while offset < final_offset {
            let (subpacket, offset_offset) = Packet::from_bits(&bits[offset..]);
            offset += offset_offset;
            subpackets.push(subpacket);
          }
        } else {
          // number of subpackets
          let num_subpackets = Bit::number_from_bits(&bits[offset..offset + 11]);
          offset += 11;
          for _ in 0..num_subpackets {
            let (subpacket, offset_offset) = Packet::from_bits(&bits[offset..]);
            offset += offset_offset;
            subpackets.push(subpacket);
          }
        }
        (
          Packet::Operation {
            version,
            type_id,
            subpackets,
          },
          offset,
        )
      }
    }
  }

  pub fn sum_versions(&self) -> u64 {
    match self {
      Self::Literal { version, .. } => *version as u64,
      Self::Operation {
        version,
        subpackets,
        ..
      } => subpackets.iter().map(|p| p.sum_versions()).sum::<u64>() + *version as u64,
    }
  }

  pub fn eval(&self) -> u64 {
    match self {
      Self::Literal { number, .. } => Bit::number_from_bits(&number),
      Self::Operation {
        type_id,
        subpackets,
        ..
      } => {
        let mut nums = subpackets.iter().map(Packet::eval);
        match type_id {
          TypeId::Add => nums.sum(),
          TypeId::Mul => nums.product(),
          TypeId::Min => nums.min().unwrap(), // Should always have at least one item.
          TypeId::Max => nums.max().unwrap(),
          TypeId::GT => {
            if nums.next().unwrap() > nums.next().unwrap() {
              1
            } else {
              0
            }
          }
          TypeId::LT => {
            if nums.next().unwrap() < nums.next().unwrap() {
              1
            } else {
              0
            }
          }
          TypeId::Eq => {
            if nums.next().unwrap() == nums.next().unwrap() {
              1
            } else {
              0
            }
          }
        }
      }
    }
  }
}

type Input = Vec<Bit>;

pub fn parse(input: &str) -> IResult<&str, Input> {
  let hex_digit = map_res(take(1_usize), |u: &str| u8::from_str_radix(u, 16));
  let (input, bytes) = many1(hex_digit)(input)?;
  let bits = bytes
    .into_iter()
    .flat_map(|byte| {
      (0..4)
        .rev()
        .map(move |offset| Bit::from_byte_and_offset(byte, offset))
    })
    .collect();
  Ok((input, bits))
}

pub fn p1(input: Input) -> u64 {
  let (p, _) = Packet::from_bits(&input);
  p.sum_versions()
}

pub fn p2(input: Input) -> u64 {
  let (p, _) = Packet::from_bits(&input);
  p.eval()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_LITERAL: &'static str = "D2FE28";
  const TEST_OPERATION: &'static str = "38006F45291200";

  #[test]
  fn test_parse() {
    let input = TEST_LITERAL;
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        vec![I, I, O, I, O, O, I, O, I, I, I, I, I, I, I, O, O, O, I, O, I, O, O, O]
      )
    )
  }

  #[test]
  fn test_literal() {
    let input = TEST_LITERAL;
    let literal = parse(input).unwrap().1;
    assert_eq!(
      Packet::from_bits(&literal),
      (
        Packet::Literal {
          version: 6,
          number: vec![O, I, I, I, I, I, I, O, O, I, O, I,]
        },
        21
      )
    );
  }

  #[test]
  fn test_operation() {
    let input = TEST_OPERATION;
    let operation = parse(input).unwrap().1;
    match Packet::from_bits(&operation) {
      (Packet::Operation { subpackets, .. }, _) => assert_eq!(subpackets.len(), 2),
      _ => panic!("parsed wrong"),
    }
  }

  #[test]
  fn test_p1() {
    let input = "8A004A801A8002F478";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 16);

    let input = "620080001611562C8802118E34";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 12);

    let input = "C0015000016115A2E0802F182340";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 23);

    let input = "A0016C880162017C3686B18A3D4780";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 31);

    let input = std::fs::read_to_string("./inputs/d16.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 947);
  }

  #[test]
  fn test_p2() {
    let input = "C200B40A82";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 3);

    let input = "04005AC33890";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 54);

    let input = "880086C3E88112";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 7);

    let input = "CE00C43D881120";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 9);

    let input = "D8005AC2A8F0";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 1);

    let input = "F600BC2D8F";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 0);

    let input = "9C005AC2F8F0";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 0);

    let input = "9C0141080250320F1802104A08";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 1);

    let input = std::fs::read_to_string("./inputs/d16.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 660797830937);
  }
}
