//! https://adventofcode.com/2021/day/23

use crate::nom_prelude::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Pawn {
  A,
  B,
  C,
  D,
}

impl Pawn {
  pub fn move_cost(self) -> u32 {
    match self {
      Self::A => 1,
      Self::B => 10,
      Self::C => 100,
      Self::D => 1000,
    }
  }

  pub fn room_entrance_index(self) -> usize {
    match self {
      Self::A => 2,
      Self::B => 4,
      Self::C => 6,
      Self::D => 8,
    }
  }

  pub fn room_exit_index(self) -> usize {
    match self {
      Self::A => 12,
      Self::B => 14,
      Self::C => 16,
      Self::D => 18,
    }
  }
}

type Dist = u32;

/// positions 0 - 10 are the hall
/// positions 11 - 18 are the rooms; the evens are closest to the entrance.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cavern([Option<Pawn>; 19]);

impl Cavern {
  pub fn goal() -> Self {
    parse(
      "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
    )
    .unwrap()
    .1
  }

  pub fn mv(self, i1: usize, i2: usize) -> Self {
    let mut new = self.0;
    new[i1] = self.0[i2];
    new[i2] = self.0[i1];
    Self(new)
  }

  pub fn possible_moves(&self) -> Vec<(Cavern, Dist)> {
    let mut moves = Vec::new();
    for (i, v) in self.0.iter().enumerate() {
      if let Some(p) = v {
        // println!("i {:?} {:?}", i, p);
        let cost = p.move_cost();
        let in_hall = i <= 10;
        // room enter
        if in_hall {
          let (open_path, spaces) = self.open_hall_path(i, p.room_entrance_index());
          if open_path {
            match self.0[p.room_exit_index() - 1..=p.room_exit_index()] {
              [Some(pawn), None] if *p == pawn => {
                // println!("first");
                moves.push((self.mv(i, p.room_exit_index()), cost * (spaces + 1)));
              }
              [None, None] => {
                // println!("second");
                moves.push((self.mv(i, p.room_exit_index() - 1), cost * (spaces + 2)));
              }
              _ => (),
            }
          }
        } else {
          // exit near door
          if i % 2 == 0 {
            for j in [0, 1, 3, 5, 7, 9, 10] {
              let (open_path, spaces) = self.open_hall_path_include_start(j, i - 10);
              if open_path {
                moves.push((self.mv(i, j), cost * (spaces + 1)));
              }
            }
          }
          // exit from back
          if i % 2 == 1 && self.0[i + 1].is_none() {
            for j in [0, 1, 3, 5, 7, 9, 10] {
              let (open_path, spaces) = self.open_hall_path_include_start(j, i - 9);
              if open_path {
                moves.push((self.mv(i, j), cost * (spaces + 2)));
              }
            }
          }
        }
      }
    }
    moves
  }

  pub fn open_hall_path(&self, start: usize, end: usize) -> (bool, u32) {
    if start < end {
      (
        self.0[start + 1..=end].iter().all(|e| e.is_none()),
        (end - start) as u32,
      )
    } else {
      (
        self.0[end..=start - 1].iter().all(|e| e.is_none()),
        (start - end) as u32,
      )
    }
  }

  pub fn open_hall_path_include_start(&self, start: usize, end: usize) -> (bool, u32) {
    if start < end {
      (
        self.0[start..=end].iter().all(|e| e.is_none()),
        (end - start) as u32,
      )
    } else {
      (
        self.0[end..=start].iter().all(|e| e.is_none()),
        (start - end) as u32,
      )
    }
  }

  // TODO: refactor out of today and d15.
  pub fn dijkstra(self) -> Dijkstra {
    let goal = Cavern::goal();
    // Start empty instead of with `unvisited` full so that we don't need to create unreachable states, if any
    let mut visited = HashSet::new();
    let mut entries = HashMap::new();
    let mut distances = BinaryHeap::new();
    entries.insert(self, (0, None));
    let mut current = self;
    loop {
      if current.0[11..19].iter().filter(|i| i.is_some()).count() >= 7 {
        // println!("current {:?} {}", visited.len(), current);
      }
      if visited.len() % 10000 == 0 {
        // println!("visited {:?} {:?}", visited.len(), current);
      }
      visited.insert(current);
      let current_dist = entries.get(&current).unwrap().0;
      if current == goal {
        return Dijkstra(entries);
      }
      for (neighbor, delta) in current.possible_moves() {
        if !visited.contains(&neighbor) {
          let e = entries.entry(neighbor).or_insert((u32::MAX, None));
          let proposed_dist = current_dist + delta;
          if proposed_dist < e.0 {
            *e = (proposed_dist, Some(current));
            distances.push(Reverse((proposed_dist, neighbor)));
          }
        }
      }

      current = loop {
        if let Some(Reverse((_, new_current))) = distances.pop() {
          if !visited.contains(&new_current) {
            break new_current;
          }
        } else {
          return Dijkstra(entries);
        }
      };
    }
  }
}

impl std::fmt::Display for Cavern {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let item = |x| match x {
      Some(c) => format!("{:?}", c),
      _ => ".".to_owned(),
    };
    write!(
      f,
      "#############
#{}{}{}{}{}{}{}{}{}{}{}#
###{}#{}#{}#{}###
  #{}#{}#{}#{}#
  #########",
      item(self.0[0]),
      item(self.0[1]),
      item(self.0[2]),
      item(self.0[3]),
      item(self.0[4]),
      item(self.0[5]),
      item(self.0[6]),
      item(self.0[7]),
      item(self.0[8]),
      item(self.0[9]),
      item(self.0[10]),
      item(self.0[12]),
      item(self.0[14]),
      item(self.0[16]),
      item(self.0[18]),
      item(self.0[11]),
      item(self.0[13]),
      item(self.0[15]),
      item(self.0[17]),
    )
  }
}

// #D#C#B#A#
//   #D#B#A#C#
// pub struct ExpandedCavern();

// impl From<Cavern> for ExpandedCavern {
//   fn from(c: Cavern) -> ExpandedCavern {
//     let new = [0; 19 + 8];
//     for i in 0..11 {
//       if i < 11 {
//         new[i] = c[i];
//       }
//     }
//     for
//     for i in 0..4 {
//       new
//     }
//     Self()
//   }
// }

type Input = Cavern;

pub struct Dijkstra(HashMap<Cavern, (u32, Option<Cavern>)>);

pub fn parse_pawn(input: &str) -> IResult<&str, Option<Pawn>> {
  map(
    alt((tag("A"), tag("B"), tag("C"), tag("D"), tag("."))),
    |p| match p {
      "A" => Some(Pawn::A),
      "B" => Some(Pawn::B),
      "C" => Some(Pawn::C),
      "D" => Some(Pawn::D),
      "." => None,
      _ => panic!("non matching tag"),
    },
  )(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  map(
    tuple((
      tag("#############\n#"),
      many_m_n(11, 11, parse_pawn),
      tag("#\n###"),
      parse_pawn,
      tag("#"),
      parse_pawn,
      tag("#"),
      parse_pawn,
      tag("#"),
      parse_pawn,
      tag("###\n  #"),
      parse_pawn,
      tag("#"),
      parse_pawn,
      tag("#"),
      parse_pawn,
      tag("#"),
      parse_pawn,
      tag("#\n  #########"),
    )),
    |(_, mut xs, _, a1, _, b1, _, c1, _, d1, _, a2, _, b2, _, c2, _, d2, _)| {
      xs.append(&mut vec![a2, a1, b2, b1, c2, c1, d2, d1]);
      let xs = xs.try_into().unwrap(); // some type pain around the error return time for `map_res`
      Cavern(xs)
    },
  )(input)
}

pub fn p1(input: Input) -> u32 {
  // let mut current = input.dijkstra().0.into_iter().last().unwrap().1 .1.unwrap();
  // while let (dist, Some(c)) = input.dijkstra().0.get(&current).unwrap() {
  //   println!("{} {}", dist, c);
  //   current = *c;
  // }
  input.dijkstra().0.get(&Cavern::goal()).unwrap().0
}

pub fn p2(_input: Input) -> usize {
  0
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

  #[test]
  fn test_parse() {
    use super::Pawn::*;
    let input = TEST_INPUT;
    assert_eq!(
      parse(input).unwrap().1 .0[11..19],
      [
        Some(A),
        Some(B),
        Some(D),
        Some(C),
        Some(C),
        Some(B),
        Some(A),
        Some(D)
      ]
    )
  }

  #[test]
  fn test_possible_moves() {
    let parsed = parse(
      "#############
#A..........#
###.#.#.#.###
  #A#.#.#.#
  #########",
    )
    .unwrap()
    .1;
    println!("{:?}", parsed.possible_moves());
    panic!("asdf");
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 12521);

    let input = std::fs::read_to_string("./inputs/d23.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 11120);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d23.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 0);
  }
}
