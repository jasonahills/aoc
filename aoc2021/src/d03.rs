/// # Reflection
///
/// I definitely paid for my decision to parse these numbers directly into u32s.  Specifically, I got bit in places where the width of the binary representation of the number is important (e.g. negating 0110 is not the same as negating 00000110).
///
/// For part two, if I were to sort my numbers I could do things without cloning, since filtering would just be adjusting the bounds of a slice.
use nom::bytes::complete::take_while;
use nom::character::complete::multispace0;
use nom::combinator::map_res;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::IResult;

type InputItem = u32;

pub fn parse(input: &str) -> IResult<&str, Vec<InputItem>> {
  let binary_digit = |c: char| c == '0' || c == '1';
  let binary_parse = |s: &str| u32::from_str_radix(s, 2);
  many1(delimited(
    multispace0,
    map_res(take_while(binary_digit), binary_parse),
    multispace0,
  ))(input)
}

fn ones_in_pos(xs: &[u32], loc: usize) -> usize {
  xs.iter().filter(|x| (*x & 1 << loc) != 0).count()
}

pub fn p1(xs: Vec<InputItem>, item_bits: usize) -> u32 {
  let mut gamma = 0;
  let threshold = xs.len() / 2;
  for i in 0..item_bits {
    if ones_in_pos(&xs, i) > threshold {
      gamma = gamma | 1 << i;
    }
  }
  // Don't want the leading bits to get flipped.
  let epsilon = !gamma & (u32::MAX >> (32 - item_bits));
  epsilon * gamma
}

pub fn p2(xs: Vec<InputItem>, item_bits: usize) -> u32 {
  let mut oxygen_candidates = xs.clone();
  let mut oxygen = 0;
  // Want to start with most significant first
  for i in (0..item_bits).rev() {
    let ones = ones_in_pos(&oxygen_candidates, i);
    let zeros = oxygen_candidates.len() - ones;
    oxygen_candidates.retain(|x| {
      if ones >= zeros {
        (x & 1 << i) != 0
      } else {
        (x & 1 << i) == 0
      }
    });
    if oxygen_candidates.len() == 1 {
      oxygen = oxygen_candidates[0]
    }
  }

  let mut co2_candidates = xs.clone();
  let mut co2 = 0;
  // Want to start with most significant first
  for i in (0..item_bits).rev() {
    let ones = ones_in_pos(&co2_candidates, i);
    let zeros = co2_candidates.len() - ones;
    co2_candidates.retain(|x| {
      if ones >= zeros {
        (x & 1 << i) == 0
      } else {
        (x & 1 << i) != 0
      }
    });
    if co2_candidates.len() == 1 {
      co2 = co2_candidates[0]
    }
  }

  oxygen * co2
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let input = "00100
    11110
    10110";
    assert_eq!(parse(input).unwrap(), ("", vec![0b00100, 0b11110, 0b10110]))
  }

  #[test]
  fn test_p1() {
    let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed, 5), 198);

    let input = std::fs::read_to_string("./inputs/d03.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed, 12), 3374136);
  }

  #[test]
  fn test_p2() {
    let input = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed, 5), 230);

    let input = std::fs::read_to_string("./inputs/d03.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed, 12), 4432698);
  }
}
