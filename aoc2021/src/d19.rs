//! https://adventofcode.com/2021/day/19

use itertools::Itertools;

use crate::nom_prelude::*;
use crate::vector::V3;
use std::collections::HashSet;

// lol all this because I didn't want to do the rotations by hand (and was reminded that group generators for non abelian groups are not quite as helpful for getting all elements as I thought)
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct RotMatrix((usize, bool), (usize, bool), (usize, bool));

impl RotMatrix {
  fn apply(self, v: V3) -> V3 {
    let x = v.get(self.0 .0) * if self.0 .1 { 1 } else { -1 };
    let y = v.get(self.1 .0) * if self.1 .1 { 1 } else { -1 };
    let z = v.get(self.2 .0) * if self.2 .1 { 1 } else { -1 };
    V3(x, y, z)
  }

  fn det(self) -> i32 {
    let sign_from_first = if self.0 .0 == 1 { -1 } else { 1 };
    let sign_from_sub = if self.1 .0 < self.2 .0 { 1 } else { -1 };
    let x = if self.0 .1 { 1 } else { -1 };
    let y = if self.1 .1 { 1 } else { -1 };
    let z = if self.2 .1 { 1 } else { -1 };
    sign_from_first * sign_from_sub * x * y * z
  }

  pub fn all() -> Vec<Self> {
    let bools = (0..3)
      .map(|_| [true, false].into_iter())
      .multi_cartesian_product();
    (0..=2)
      .permutations(3)
      .cartesian_product(bools)
      .map(|(vals, bools)| {
        RotMatrix(
          (vals[0], bools[0]),
          (vals[1], bools[1]),
          (vals[2], bools[2]),
        )
      })
      // Rotations are all the transformations with determinant 1.
      .filter(|m| m.det() == 1)
      .collect_vec()
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
  id: u32,
  readings: Vec<V3>,
}

impl Scanner {
  pub fn rotate_and_translate(&mut self, rotation: &RotMatrix, translation: V3) {
    self.readings = self
      .readings
      .iter()
      .map(|p| rotation.apply(*p) + translation)
      .collect()
  }
  /// If successful, returns the displacement from self to other and the rotation
  /// needed to get other to orient other with self.
  pub fn overlaps(&self, other: &Self) -> Option<(V3, RotMatrix)> {
    for (other_pts, rot) in other.rotations().into_iter() {
      for p in &self.readings {
        for other_p in &other_pts {
          let transform = *other_p - *p;
          let transformed = other_pts
            .iter()
            .map(|other_p| *other_p - transform)
            .collect::<HashSet<_>>();
          if self
            .readings
            .iter()
            .filter(|p| transformed.contains(p))
            .count()
            >= 12
          {
            // Probably can get away without checking that no other coordinates are in the box of matches; TODO: should check that though.
            return Some((transform, rot));
          }
        }
      }
    }
    None
  }

  /// Rotational symmetries of a cube/octahedron
  pub fn rotations(&self) -> Vec<(Vec<V3>, RotMatrix)> {
    RotMatrix::all()
      .into_iter()
      .map(|m| {
        let vs = self.readings.iter().map(|r| m.apply(*r)).collect_vec();
        (vs, m)
      })
      .collect_vec()
  }
}

type Input = Vec<Scanner>;

pub fn parse_scanner(input: &str) -> IResult<&str, Scanner> {
  let id = delimited(tag("--- scanner "), parse_u32, tag(" ---\n"));
  let readings = many1(map(
    tuple((
      parse_i32,
      tag(","),
      parse_i32,
      tag(","),
      parse_i32,
      tag("\n"),
    )),
    |(x, _, y, _, z, _)| V3(x, y, z),
  ));
  map(tuple((id, readings)), |(id, beacon_readings)| Scanner {
    id,
    readings: beacon_readings,
  })(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  lines_of(parse_scanner)(input)
}

// TODO: I'm brute forcing all of this; do it for real with speed next time.

pub fn p1(input: Input) -> usize {
  // Start with our first one as the reference frame.
  let mut assigned = input.iter().take(1).cloned().collect_vec();
  let mut unassigned = input.into_iter().skip(1).collect_vec();

  // This is gross.
  let mut assigned_i = 0; // once this reaches the end, we have made it.
  while !unassigned.is_empty() {
    let mut unassigned_i = 0;
    while unassigned_i < unassigned.len() {
      let unassigned_scanner = &unassigned[unassigned_i];
      let assigned_scanner = &assigned[assigned_i];
      println!("{} {}", assigned_scanner.id, unassigned_scanner.id);
      if let Some((trans, rot)) = assigned_scanner.overlaps(&unassigned_scanner) {
        let mut new_assigned = unassigned_scanner.clone();
        drop(unassigned_scanner);
        unassigned.remove(unassigned_i);
        // because we persist the scanners wrt 0 reference frame, we can just do the transformation
        new_assigned.rotate_and_translate(&rot, V3(0, 0, 0) - trans);
        assigned.push(new_assigned);
      } else {
        unassigned_i = unassigned_i + 1;
      }
    }
    assigned_i = assigned_i + 1;
  }

  assigned
    .into_iter()
    .flat_map(|s| s.readings)
    .unique()
    .count()
}

pub fn p2(input: Input) -> usize {
  // Start with our first one as the reference frame.
  let mut assigned = input.iter().take(1).cloned().collect_vec();
  let mut unassigned = input.into_iter().skip(1).collect_vec();

  let mut translations = vec![V3(0, 0, 0)];
  // This is gross.
  let mut assigned_i = 0; // once this reaches the end, we have made it.
  while !unassigned.is_empty() {
    let mut unassigned_i = 0;
    while unassigned_i < unassigned.len() {
      let unassigned_scanner = &unassigned[unassigned_i];
      let assigned_scanner = &assigned[assigned_i];
      println!("{} {}", assigned_scanner.id, unassigned_scanner.id);
      if let Some((trans, rot)) = assigned_scanner.overlaps(&unassigned_scanner) {
        translations.push(trans);
        let mut new_assigned = unassigned_scanner.clone();
        drop(unassigned_scanner);
        unassigned.remove(unassigned_i);
        // because we persist the scanners wrt 0 reference frame, we can just do the transformation
        new_assigned.rotate_and_translate(&rot, V3(0, 0, 0) - trans);
        assigned.push(new_assigned);
      } else {
        unassigned_i = unassigned_i + 1;
      }
    }
    assigned_i = assigned_i + 1;
  }

  translations
    .into_iter()
    .tuple_combinations()
    .map(|(x, y)| x.manhattan(y))
    .max()
    .unwrap() as usize
}

#[cfg(test)]
mod test {
  use itertools::Itertools;

  use super::*;

  #[test]
  fn test_parse() {
    let input = "--- scanner 0 ---
-1,-1,1
-2,-2,2
    
--- scanner 1 ---
1,2,3
4,5,6
";
    assert_eq!(
      parse(input).unwrap().1,
      vec![
        Scanner {
          id: 0,
          readings: vec![V3(-1, -1, 1), V3(-2, -2, 2)]
        },
        Scanner {
          id: 1,
          readings: vec![V3(1, 2, 3), V3(4, 5, 6)]
        }
      ]
    );
  }

  #[test]
  fn test_rotations() {
    let s = Scanner {
      id: 0,
      readings: vec![V3(1, 0, 0), V3(0, 2, 0), V3(0, 0, 3)],
    };
    let rots = s.rotations();
    let hashes = rots
      .iter()
      .map(|(r, _)| r.iter().copied().collect::<HashSet<_>>())
      .collect::<Vec<_>>();

    for (r1, r2) in hashes.iter().tuple_combinations() {
      assert_eq!(r1.len(), 3);
      assert_eq!(r2.len(), 3);
      assert!(r1.intersection(r2).collect::<Vec<_>>().len() < 3);
    }

    let s = Scanner {
      id: 0,
      readings: vec![V3(1, 1, 1)],
    };
    let rots = s.rotations();
    rots
      .into_iter()
      .flat_map(|(r, _)| r.into_iter())
      .counts()
      .into_iter()
      .for_each(|(_, c)| assert_eq!(c, 3));
  }

  #[test]
  fn test_p1() {
    let input = std::fs::read_to_string("./inputs/d19-test.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 79);

    let input = std::fs::read_to_string("./inputs/d19.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 459);
  }

  #[test]
  fn test_p2() {
    let input = std::fs::read_to_string("./inputs/d19-test.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 3621);

    let input = std::fs::read_to_string("./inputs/d19.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 19130);
  }
}
