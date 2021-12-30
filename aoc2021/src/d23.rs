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

  pub fn expanded_room_exit_index(self) -> usize {
    match self {
      Self::A => 14,
      Self::B => 18,
      Self::C => 22,
      Self::D => 26,
    }
  }

  // lol this is terrible.
  pub fn corresponding_expanded_entrance(u: usize) -> usize {
    match u {
      14 => 2,
      18 => 4,
      22 => 6,
      26 => 8,
      _ => panic!("asdf"),
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

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct ExpandedCavern([Option<Pawn>; 19 + 8]);

impl ExpandedCavern {
  pub fn goal() -> Self {
    use Pawn::*;
    Self([
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
      Some(A),
      Some(A),
      Some(A),
      Some(A),
      Some(B),
      Some(B),
      Some(B),
      Some(B),
      Some(C),
      Some(C),
      Some(C),
      Some(C),
      Some(D),
      Some(D),
      Some(D),
      Some(D),
    ])
  }

  pub fn mv(self, i1: usize, i2: usize) -> Self {
    let mut new = self.0;
    new[i1] = self.0[i2];
    new[i2] = self.0[i1];
    Self(new)
  }

  pub fn possible_moves(&self) -> Vec<(ExpandedCavern, Dist)> {
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
            match self.0[p.expanded_room_exit_index() - 3..=p.expanded_room_exit_index()] {
              [Some(p1), Some(p2), Some(p3), None] if *p == p1 && *p == p2 && *p == p3 => {
                moves.push((
                  self.mv(i, p.expanded_room_exit_index()),
                  cost * (spaces + 1),
                ));
              }
              [Some(p1), Some(p2), None, None] if *p == p1 && *p == p2 => {
                moves.push((
                  self.mv(i, p.expanded_room_exit_index() - 1),
                  cost * (spaces + 2),
                ));
              }
              [Some(p1), None, None, None] if *p == p1 => {
                moves.push((
                  self.mv(i, p.expanded_room_exit_index() - 2),
                  cost * (spaces + 3),
                ));
              }
              [None, None, None, None] => {
                moves.push((
                  self.mv(i, p.expanded_room_exit_index() - 3),
                  cost * (spaces + 4),
                ));
              }
              _ => (),
            }
          }
        } else {
          // exit near door
          if i % 4 == 2 {
            let entrance = Pawn::corresponding_expanded_entrance(i);
            for j in [0, 1, 3, 5, 7, 9, 10] {
              let (open_path, spaces) = self.open_hall_path_include_start(j, entrance);
              if open_path {
                moves.push((self.mv(i, j), cost * (spaces + 1)));
              }
            }
          }
          // exit from next back
          if i % 4 == 1 && self.0[i + 1].is_none() {
            let entrance = Pawn::corresponding_expanded_entrance(i + 1);
            for j in [0, 1, 3, 5, 7, 9, 10] {
              let (open_path, spaces) = self.open_hall_path_include_start(j, entrance);
              if open_path {
                moves.push((self.mv(i, j), cost * (spaces + 2)));
              }
            }
          }

          // exit from third
          if i % 4 == 0 && self.0[i + 1].is_none() && self.0[i + 2].is_none() {
            let entrance = Pawn::corresponding_expanded_entrance(i + 2);
            for j in [0, 1, 3, 5, 7, 9, 10] {
              let (open_path, spaces) = self.open_hall_path_include_start(j, entrance);
              if open_path {
                moves.push((self.mv(i, j), cost * (spaces + 3)));
              }
            }
          }

          // exit from fourth
          if i % 4 == 3
            && self.0[i + 1].is_none()
            && self.0[i + 2].is_none()
            && self.0[i + 3].is_none()
          {
            let entrance = Pawn::corresponding_expanded_entrance(i + 3);
            for j in [0, 1, 3, 5, 7, 9, 10] {
              let (open_path, spaces) = self.open_hall_path_include_start(j, entrance);
              if open_path {
                moves.push((self.mv(i, j), cost * (spaces + 4)));
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
  pub fn dijkstra(self) -> HashMap<ExpandedCavern, (u32, Option<ExpandedCavern>)> {
    let goal = Self::goal();
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
        return entries;
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
          return entries;
        }
      };
    }
  }
}

impl From<Cavern> for ExpandedCavern {
  fn from(c: Cavern) -> ExpandedCavern {
    let mut new = [None; 19 + 8];
    for i in 0..11 {
      new[i] = c.0[i];
    }
    new[11] = c.0[11];
    new[12] = Some(Pawn::D);
    new[13] = Some(Pawn::D);
    new[14] = c.0[12];
    new[15] = c.0[13];
    new[16] = Some(Pawn::B);
    new[17] = Some(Pawn::C);
    new[18] = c.0[14];
    new[19] = c.0[15];
    new[20] = Some(Pawn::A);
    new[21] = Some(Pawn::B);
    new[22] = c.0[16];
    new[23] = c.0[17];
    new[24] = Some(Pawn::C);
    new[25] = Some(Pawn::A);
    new[26] = c.0[18];
    Self(new)
  }
}

impl std::fmt::Display for ExpandedCavern {
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
  #{}#{}#{}#{}#
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
      item(self.0[14]),
      item(self.0[18]),
      item(self.0[22]),
      item(self.0[26]),
      item(self.0[13]),
      item(self.0[17]),
      item(self.0[21]),
      item(self.0[25]),
      item(self.0[12]),
      item(self.0[16]),
      item(self.0[20]),
      item(self.0[24]),
      item(self.0[11]),
      item(self.0[15]),
      item(self.0[19]),
      item(self.0[23]),
    )
  }
}

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
  input.dijkstra().0.get(&Cavern::goal()).unwrap().0
}

pub fn p2(input: Input) -> u32 {
  ExpandedCavern::from(input)
    .dijkstra()
    .get(&ExpandedCavern::goal())
    .unwrap()
    .0
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
    assert_eq!(p2(parsed), 44169);

    let input = std::fs::read_to_string("./inputs/d23.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 0);
  }
}
