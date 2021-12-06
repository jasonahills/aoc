use nom::character::complete::{char as parse_char, digit1, multispace0, space0};
use nom::combinator::{map, map_res};
use nom::multi::{count, many1, separated_list1};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use std::convert::{identity, TryInto};

#[derive(Debug, Eq, PartialEq)]
pub struct Bingo {
  draws: Vec<u32>,
  boards: Vec<Board>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
  entries: [[u32; 5]; 5],
  marked: [[bool; 5]; 5],
}

impl Board {
  /// Returns `true` if a new entry was marked
  fn mark(&mut self, entry: u32) -> bool {
    for i in 0..5 {
      for j in 0..5 {
        if self.entries[i][j] == entry {
          self.marked[i][j] = true;
          return true;
        }
      }
    }
    false
  }

  fn unmarked<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
    self
      .entries
      .iter()
      .enumerate()
      .flat_map(|(row_num, row)| {
        let marked = &self.marked;
        row.iter().enumerate().filter_map(move |(col_num, entry)| {
          if marked[row_num][col_num] {
            None
          } else {
            Some(entry)
          }
        })
      })
      .copied()
  }

  fn marked_rows<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = bool> + 'a> + 'a {
    self.marked.iter().map(|r| r.iter().copied())
  }

  fn marked_cols<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = bool> + 'a> + 'a {
    let marked = &self.marked;
    (0..5).map(move |col_num| (0..5).map(move |row_num| marked[row_num][col_num]))
  }

  fn has_victory(&self) -> bool {
    self.marked_rows().any(|mut r| r.all(identity))
      || self.marked_cols().any(|mut r| r.all(identity))
  }
}

// I can't seem to get the out-of-the-box numerical parsers to work well on strs
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
  map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

pub fn parse_draws(input: &str) -> IResult<&str, Vec<u32>> {
  separated_list1(parse_char(','), parse_u32)(input)
}

pub fn parse_board(input: &str) -> IResult<&str, Board> {
  let entry = terminated(parse_u32, space0);
  let line = map(
    delimited(multispace0, count(entry, 5), multispace0),
    |l: Vec<u32>| l.try_into().unwrap(),
  );
  // Why is this mutable?
  let mut board = count(line, 5);
  let (input, parsed) = board(input)?;
  let entries = parsed.try_into().unwrap();
  Ok((
    input,
    Board {
      entries,
      marked: [[false; 5]; 5],
    },
  ))
}

pub fn parse(input: &str) -> IResult<&str, Bingo> {
  let (input, draws) = parse_draws(input)?;
  let (input, boards) = many1(parse_board)(input)?;
  Ok((input, Bingo { draws, boards }))
}

pub fn p1(mut input: Bingo) -> u32 {
  for draw in input.draws {
    for board in input.boards.iter_mut() {
      if board.mark(draw) && board.has_victory() {
        return board.unmarked().sum::<u32>() * draw;
      }
    }
  }
  panic!("no solution")
}

pub fn p2(mut input: Bingo) -> u32 {
  let scored = input
    .boards
    .iter_mut()
    .map(|board| {
      for (i, draw) in input.draws.iter().enumerate() {
        if board.mark(*draw) && board.has_victory() {
          return (board.unmarked().sum::<u32>() * *draw, i);
        }
      }
      panic!("no solution")
    })
    .max_by_key(|(_, i)| *i)
    .unwrap();
  scored.0
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str =
    "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

  22 13 17 11  0
   8  2 23  4 24
  21  9 14 16  7
   6 10  3 18  5
   1 12 20 15 19
  
   3 15  0  2 22
   9 18 13 17  5
  19  8  7 25 23
  20 11 10 24  4
  14 21 16 12  6
  
  14 21 17 24  4
  10 16 15  9 19
  18  8 23 26 20
  22 11 13  6  5
   2  0 12  3  7";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    let output = parse(input).unwrap().1;
    assert_eq!(*output.draws.first().unwrap(), 7);
    assert_eq!(*output.draws.last().unwrap(), 1);
    assert_eq!(output.boards.len(), 3);
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 4512);

    let input = std::fs::read_to_string("./inputs/d04.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 50008);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 1924);

    let input = std::fs::read_to_string("./inputs/d04.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 17408);
  }
}
