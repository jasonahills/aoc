use crate::nom_prelude::*;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Wire {
  A,
  B,
  C,
  D,
  E,
  F,
  G,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SevenSegment {
  wire_diagram: Vec<HashSet<Wire>>,
  display: Vec<HashSet<Wire>>,
}

impl SevenSegment {
  pub fn digit(&self, d: &HashSet<Wire>) -> Option<u32> {
    match d.len() {
      2 => Some(1),
      3 => Some(7),
      4 => Some(4),
      7 => Some(8),
      6 => {
        // could be 0, 6, 9
        if self.get_4().intersection(d).count() == 4 {
          Some(9)
        } else if self.get_1().intersection(d).count() == 2 {
          Some(0)
        } else {
          Some(6)
        }
      }
      5 => {
        // could be 2, 3, 5
        if self.get_4().intersection(d).count() == 2 {
          Some(2)
        } else if self.get_1().intersection(d).count() == 2 {
          Some(3)
        } else {
          Some(5)
        }
      }
      _ => panic!("invalid digit"),
    }
  }

  pub fn get_4(&self) -> &HashSet<Wire> {
    self.wire_diagram.iter().find(|w| w.len() == 4).unwrap()
  }

  pub fn get_1(&self) -> &HashSet<Wire> {
    self.wire_diagram.iter().find(|w| w.len() == 2).unwrap()
  }
}

type InputItem = SevenSegment;

fn parse_wires(input: &str) -> IResult<&str, HashSet<Wire>> {
  map(
    many1(map(one_of("abcdefg"), |c| match c {
      'a' => Wire::A,
      'b' => Wire::B,
      'c' => Wire::C,
      'd' => Wire::D,
      'e' => Wire::E,
      'f' => Wire::F,
      'g' => Wire::G,
      _ => panic!("must be one of the above"),
    })),
    |wires| wires.into_iter().collect::<HashSet<_>>(),
  )(input)
}

pub fn parse_wires_list(input: &str) -> IResult<&str, Vec<HashSet<Wire>>> {
  many1(delimited(space0, parse_wires, space0))(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<InputItem>> {
  // It's silly that I have to wrap this in a closure.
  let seven_segment = map(
    separated_pair(
      parse_wires_list,
      tuple((space0, tag("|"), space0)),
      parse_wires_list,
    ),
    |(wire_diagram, display)| SevenSegment {
      wire_diagram,
      display,
    },
  );
  many1(delimited(multispace0, seven_segment, multispace0))(input)
}

pub fn p1(xs: Vec<InputItem>) -> usize {
  xs.iter()
    .map(|x| {
      x.display
        .iter()
        .filter(|digit_display| matches!(digit_display.len(), 2 | 3 | 4 | 7))
        .count()
    })
    .sum()
}

pub fn p2(xs: Vec<InputItem>) -> u32 {
  xs.iter()
    .map(|seven_segment| {
      seven_segment
        .display
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, d)| {
          let p = 10_u32.pow(i as u32);
          let d = seven_segment.digit(&d).unwrap();
          let acc = acc + (d * p);
          acc
        })
    })
    .sum::<u32>()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
  edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
  fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
  fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
  aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
  fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
  dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
  bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
  egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
  gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

  #[test]
  fn test_parse() {
    let input = "abc";
    let output = &parse_wires(input).unwrap().1;
    assert!(output.contains(&Wire::A));
    assert!(output.contains(&Wire::B));
    assert!(output.contains(&Wire::C));
    assert_eq!(output.len(), 3);

    let input = "ab cde | fgab";
    let output = &parse(input).unwrap().1[0];
    assert_eq!(output.wire_diagram.len(), 2);
    assert_eq!(output.display.len(), 1);
    assert_eq!(output.wire_diagram[0].len(), 2);
    assert_eq!(output.wire_diagram[1].len(), 3);
    assert_eq!(output.display[0].len(), 4);
    assert!(output.display[0].contains(&Wire::A));
    assert!(output.display[0].contains(&Wire::B));
    assert!(!output.display[0].contains(&Wire::C));
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 26);

    let input = std::fs::read_to_string("./inputs/d08.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 479);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 61229);

    let input = std::fs::read_to_string("./inputs/d08.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 1041746);
  }
}
