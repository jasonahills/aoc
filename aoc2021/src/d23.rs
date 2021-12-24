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

  pub fn room_entry_index(self) -> usize {
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
/// positions 11 - 18 are the rooms
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Cavern([Option<Pawn>; 19]);

impl Cavern {
  pub fn goal() -> Self {
    parse(
      "#############
#...........#
###B#C#A#D###
  #B#C#D#A#
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

  pub fn neighbors(&self) -> Vec<(Cavern, Dist)> {
    let mut ns = Vec::new();
    for (i, v) in self.0.iter().enumerate() {
      if let Some(p) = v {
        let cost = p.move_cost();
        // hall left
        if 0 < i && i <= 10 && self.0[i - 1].is_none() {
          ns.push((self.mv(i, i - 1), cost));
        }
        // hall right
        if 0 <= i && i < 10 && self.0[i + 1].is_none() {
          ns.push((self.mv(i, i + 1), cost));
        }
        // room up
        if 11 <= i && i % 2 == 0 && self.0[i - 1].is_none() {
          ns.push((self.mv(i, i - 1), cost));
        }
        // room down
        if 11 <= i && i % 2 == 1 && self.0[i + 1].is_none() {
          ns.push((self.mv(i, i + 1), cost));
        }
        // room enter
        if i == p.room_entry_index() && self.0[p.room_exit_index()].is_none() {
          ns.push((self.mv(i, p.room_exit_index()), cost));
        }
        // room exit.
        // Note that the entry and exits are exactly 10 off from each other, lol
        if 11 < i && i % 2 == 0 && self.0[i - 10].is_none() {
          ns.push((self.mv(i, i - 10), cost));
        }
      }
    }
    ns
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
      if visited.len() % 10000 == 0 {
        println!("visited {:?}", visited.len());
      }
      visited.insert(current);
      let current_dist = entries.get(&current).unwrap().0;
      if current == goal {
        return Dijkstra(entries);
      }
      for (neighbor, delta) in current.neighbors() {
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

type Input = Cavern;

pub struct Dijkstra(HashMap<Cavern, (u32, Option<Cavern>)>);

pub fn parse_pawn(input: &str) -> IResult<&str, Pawn> {
  map(alt((tag("A"), tag("B"), tag("C"), tag("D"))), |p| match p {
    "A" => Pawn::A,
    "B" => Pawn::B,
    "C" => Pawn::C,
    "D" => Pawn::D,
    _ => panic!("non matching tag"),
  })(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  map(
    tuple((
      tag("#############\n#...........#\n###"),
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
    |(_, a1, _, b1, _, c1, _, d1, _, a2, _, b2, _, c2, _, d2, _)| {
      Cavern([
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(a2),
        Some(a1),
        Some(b2),
        Some(b1),
        Some(c2),
        Some(c1),
        Some(d2),
        Some(d1),
      ])
    },
  )(input)
}

pub fn p1(input: Input) -> u32 {
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
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 12521);

    let input = std::fs::read_to_string("./inputs/d23.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 0);
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
