use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, map_res};
use nom::multi::{many0, many1};
use nom::sequence::{delimited, tuple};
use nom::IResult;
use std::collections::HashMap;

type DistanceEntry = (City, City, u32);
type City = String;
type Distances = HashMap<(City, City), u32>;

fn city(input: &str) -> IResult<&str, City> {
  map(alpha1, |s: &str| s.to_string())(input)
}

fn distance_entry(input: &str) -> IResult<&str, DistanceEntry> {
  let (input, (c1, _, c2, _, d)) = tuple((
    city,
    tag(" to "),
    city,
    tag(" = "),
    map_res(digit1, |s: &str| s.parse::<u32>()),
  ))(input)?;
  Ok((input, (c1, c2, d)))
}

pub fn distance_entries(input: &str) -> IResult<&str, Vec<DistanceEntry>> {
  many1(delimited(
    many0(tag("\n")),
    distance_entry,
    many0(tag("\n")),
  ))(input)
}

// TODO: learn more about graph traversal stuff, especially travelling salesman.
// But 7! is not too much for a naive approach.

pub fn cities(distances: &Distances) -> Vec<City> {
  distances
    .keys()
    .flat_map(|(c1, c2)| [c1, c2].into_iter())
    .unique()
    .cloned()
    .collect()
}

pub fn path_distance(distances: &Distances, path: &[&City]) -> u32 {
  path
    .iter()
    .zip(path.iter().skip(1))
    .map(|(c1, c2)| {
      distances
        .get(&(c1.to_string(), c2.to_string()))
        .or_else(|| distances.get(&(c2.to_string(), c1.to_string())))
        .unwrap()
    })
    .sum::<u32>()
}

// The distances passed in can be unidirectional; we'll make them bidirectional ourselves
pub fn p1(distances: &Distances) -> u32 {
  let cs = cities(distances);
  cs.iter().permutations(cs.len());
  0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse() {
    let input = "asdf to fdas = 2
asdf to qwer = 3";
    assert_eq!(
      distance_entries(input).unwrap(),
      (
        "",
        vec![
          ("asdf".to_string(), "fdas".to_string(), 2),
          ("asdf".to_string(), "qwer".to_string(), 3)
        ]
      )
    )
  }
}
