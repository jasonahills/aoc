//! https://adventofcode.com/2021/day/15

use crate::nom_prelude::*;
use itertools::Itertools;
use std::{
  cmp::Reverse,
  collections::{BinaryHeap, HashMap, HashSet},
};

type Coord = (i32, i32);

#[derive(Debug, PartialEq, Eq)]
pub struct Grid {
  width: usize,
  items: Vec<u32>,
}

impl Grid {
  fn height(&self) -> usize {
    self.items.len() / self.width
  }

  fn bigger_grid(self, mult_by: u32) -> Self {
    let column = (0..mult_by)
      .into_iter()
      .flat_map(|i| self.items.iter().map(move |x| ((*x - 1 + i) % 9) + 1))
      .collect::<Vec<_>>();

    assert_eq!(self.width * self.height() * mult_by as usize, column.len());

    let whole = column
      .chunks(self.width)
      .flat_map(|w| {
        (0..mult_by)
          .into_iter()
          .flat_map(|i| w.iter().map(move |x| ((*x - 1 + i) % 9) + 1))
      })
      .collect::<Vec<_>>();

    assert_eq!(
      self.width * self.height() * mult_by as usize * mult_by as usize,
      whole.len()
    );

    Self {
      width: self.width * mult_by as usize,
      items: whole,
    }
  }

  pub fn at(&self, (x, y): Coord) -> Option<u32> {
    if 0 <= x && x < self.width as i32 && 0 <= y && y < self.height() as i32 {
      Some(self.items[(y as usize * self.width) + x as usize])
    } else {
      None
    }
  }

  pub fn entry(&self, c: Coord) -> Option<(Coord, u32)> {
    let v = self.at(c)?;
    Some((c, v))
  }

  pub fn neighbors(&self, (x, y): Coord) -> [Coord; 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
  }

  fn coords(&self) -> Vec<Coord> {
    (0..self.width)
      .cartesian_product(0..self.height())
      .map(|(x, y)| (x as i32, y as i32))
      .collect()
  }

  pub fn dijkstra(&self, start: Coord) -> Dijkstra {
    let mut unvisited = self.coords().into_iter().collect::<HashSet<_>>();
    let mut entries = HashMap::new();
    let mut distances = BinaryHeap::new();
    entries.insert(start, (0, None));
    let mut current = start;
    loop {
      unvisited.remove(&current);
      let current_dist = entries.get(&current).unwrap().0;
      for neighbor in self.neighbors(current) {
        if unvisited.contains(&neighbor) {
          if let Some(delta) = self.at(neighbor) {
            let e = entries.entry(neighbor).or_insert((u32::MAX, None));
            let proposed_dist = current_dist + delta;
            if proposed_dist < e.0 {
              *e = (proposed_dist, Some(current));
              distances.push(Reverse((proposed_dist, neighbor)));
            }
          }
        }
      }

      current = loop {
        if let Some(Reverse((_, new_current))) = distances.pop() {
          if unvisited.contains(&new_current) {
            break new_current;
          }
        } else {
          return Dijkstra(entries);
        }
      };
    }
  }
}

impl std::fmt::Display for Grid {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for chunk in self.items.chunks(self.width) {
      for item in chunk {
        write!(f, "{}", item)?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

pub struct Dijkstra(HashMap<Coord, (u32, Option<Coord>)>);

type InputItem = Grid;

pub fn parse(input: &str) -> IResult<&str, InputItem> {
  let entry = map_parser(take(1_u32), parse_u32);
  map(lines_of(many1(entry)), |xss| {
    let width = xss[0].len();
    let height = xss.len();
    let items = xss.into_iter().flatten().collect::<Vec<_>>();
    assert_eq!(items.len(), width * height);
    Grid { width, items }
  })(input)
}

pub fn p1(input: InputItem) -> usize {
  let d = input.dijkstra((0, 0));
  let end = (input.width as i32 - 1, input.height() as i32 - 1);
  d.0.get(&end).unwrap().0 as usize
}

pub fn p2(input: InputItem) -> usize {
  let input = input.bigger_grid(5);
  let d = input.dijkstra((0, 0));
  let end = (input.width as i32 - 1, input.height() as i32 - 1);
  d.0.get(&end).unwrap().0 as usize
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "1163751742
  1381373672
  2136511328
  3694931569
  7463417111
  1319128137
  1359912421
  3125421639
  1293138521
  2311944581";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    let g = parse(input).unwrap().1;
    assert_eq!(g.at((0, 0)), Some(1));
    assert_eq!(g.at((9, 0)), Some(2));
    assert_eq!(g.at((8, 9)), Some(8));
  }

  #[test]
  fn test_bigger_grid() {
    let g = parse("6").unwrap().1;
    let bigger = parse(
      "678
    789
    891",
    )
    .unwrap()
    .1;
    assert_eq!(g.bigger_grid(3), bigger);

    let g = parse("34").unwrap().1;
    let bigger = parse(
      "344556
    455667
    566778",
    )
    .unwrap()
    .1;
    assert_eq!(g.bigger_grid(3), bigger);
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 40);

    let input = std::fs::read_to_string("./inputs/d15.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 602);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 315);

    let input = std::fs::read_to_string("./inputs/d15.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 2935);
  }
}
