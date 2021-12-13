use std::collections::{HashMap, HashSet};
use itertools::Itertools;

use crate::nom_prelude::*;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Location<'a> {
  Start,
  Large(&'a str),
  Small(&'a str),
  End,
}

impl<'a> Location<'a> {
  fn can_revisit_repeatedly(&self) -> bool {
    matches!(self, Location::Large(_))
  }
}

type Connection<'a> = (Location<'a>, Location<'a>);

type Path<'a> = Vec<Location<'a>>;

pub struct Cavern<'a> {
  locations: HashMap<Location<'a>, HashSet<Location<'a>>>,
}

impl<'a> Cavern<'a> {
  pub fn new() -> Self {
    Self {
      locations: HashMap::new(),
    }
  }

  pub fn add_directed_edge(&mut self, conn: Connection<'a>) {
    let e = self.locations.entry(conn.0).or_default();
    e.insert(conn.1);
  }

  pub fn from_connections(conns: Vec<Connection<'a>>) -> Self {
    let mut c = Self::new();
    for conn in conns {
      c.add_directed_edge(conn);
      c.add_directed_edge((conn.1, conn.0));
    }
    c
  }

  pub fn paths(&self, loc_test: fn(&Path, &Location) -> bool) -> Vec<Path<'a>> {
    self.path_step(vec![Location::Start], loc_test)
  }

  pub fn path_step(&self, path_so_far: Path<'a>, loc_test: fn(&Path, &Location) -> bool) -> Vec<Path<'a>> {
    match path_so_far.last() {
      Some(Location::End) => vec![path_so_far],
      Some(loc) => {
        self.locations.get(loc).unwrap().iter().filter(|loc| loc_test(&path_so_far, loc)).flat_map(|loc| {
          // Sigh, but it's easy.
          let mut new_path_so_far = path_so_far.clone();
          new_path_so_far.push(loc.clone());
          self.path_step(new_path_so_far, loc_test)
        }).collect()
      }
      None => vec![vec![Location::Start]],
    }
  }
}

pub fn parse_location(input: &str) -> IResult<&str, Location> {
  map(alpha1, |s| match s {
    "start" => Location::Start,
    "end" => Location::End,
    s_prime if &s_prime.to_lowercase() == s => Location::Small(s),
    s => Location::Large(s),
  })(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Connection>> {
  let connection = separated_pair(parse_location, tag("-"), parse_location);
  many1(delimited(multispace0, connection, multispace0))(input)
}

fn p1_loc_test(path_so_far: &Path, loc: &Location) -> bool {
  loc.can_revisit_repeatedly() || !path_so_far.contains(loc)
}

pub fn p1(conns: Vec<Connection>) -> usize {
  let c = Cavern::from_connections(conns);
  c.paths(p1_loc_test).iter().filter(|p| matches!(p.last(), Some(Location::End))).count()
}

fn p2_loc_test(path_so_far: &Path, loc: &Location) -> bool {
  match loc {
    Location::Start => false,
    Location::Large(_) | Location::End => true,
    small => {
      !path_so_far.contains(small) || path_so_far.iter().filter(|loc| matches!(loc, Location::Small(_))).count() == path_so_far.iter().filter(|loc| matches!(loc, Location::Small(_))).unique().count()
    }
  }
}

// lol, I'm doing this in a pretty slow manner.
pub fn p2(conns: Vec<Connection>) -> usize {
  let c = Cavern::from_connections(conns);
  c.paths(p2_loc_test).iter().filter(|p| matches!(p.last(), Some(Location::End))).count()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "fs-end
  he-DX
  fs-he
  start-DX
  pj-DX
  end-zg
  zg-sl
  zg-pj
  pj-he
  RW-he
  fs-DX
  pj-RW
  zg-RW
  start-pj
  he-WI
  zg-he
  pj-fs
  start-RW";

  #[test]
  fn test_parse() {
    let input = "start-A
    b-end";
    assert_eq!(
      parse(input).unwrap().1,
      vec![
        (Location::Start, Location::Large("A")),
        (Location::Small("b"), Location::End),
      ]
    )
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 226);

    let input = std::fs::read_to_string("./inputs/d12.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 4104);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 3509);

    let input = std::fs::read_to_string("./inputs/d12.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 119760);
  }
}
