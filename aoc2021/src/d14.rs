//! https://adventofcode.com/2021/day/14

use crate::nom_prelude::*;
use itertools::Itertools;
use std::collections::HashMap;

type Base = char;

#[derive(Debug, Eq, PartialEq)]
pub struct Polymer(Vec<Base>);

impl Polymer {
  pub fn step(self, rules: &PolymerizationRules) -> Self {
    let new_bases = self
      .0
      .iter()
      .cloned()
      .tuple_windows()
      .map(|pair| rules.get(&pair).cloned());
    Self(
      self
        .0
        .iter()
        .cloned()
        .map(Some)
        .interleave(new_bases)
        .filter_map(|e| e)
        .collect::<Vec<_>>(),
    )
  }
}

type PolymerizationRules = HashMap<(Base, Base), Base>;

fn output_pairs(rules: &PolymerizationRules, pair: (Base, Base)) -> Vec<(Base, Base)> {
  if let Some(b) = rules.get(&pair) {
    vec![(pair.0, *b), (*b, pair.1)]
  } else {
    vec![pair]
  }
}

type Input = (Polymer, PolymerizationRules);

pub struct BetterPolymer {
  pairs: HashMap<(Base, Base), usize>,
  start: Base,
  end: Base,
}

impl BetterPolymer {
  pub fn step(self, rules: &PolymerizationRules) -> Self {
    let start = self.start;
    let end = self.end;
    let pairs = self
      .pairs
      .iter()
      .fold(HashMap::new(), |mut acc, ((a, b), v)| {
        for p in output_pairs(rules, (*a, *b)) {
          let e = acc.entry(p).or_insert(0);
          *e += v;
        }
        acc
      });
    Self { pairs, start, end }
  }

  pub fn letter_counts(&self) -> HashMap<Base, usize> {
    let mut counts = HashMap::new();
    for ((a, b), v) in self.pairs.iter() {
      let e = counts.entry(a).or_insert(0);
      *e += v;
      let e = counts.entry(b).or_insert(0);
      *e += v;
    }
    // Everything else gets double counted but these two
    let e = counts.entry(&self.start).or_insert(0);
    *e += 1;
    let e = counts.entry(&self.end).or_insert(0);
    *e += 1;
    counts.into_iter().map(|(k, v)| (*k, v / 2)).collect()
  }
}

impl From<Polymer> for BetterPolymer {
  fn from(p: Polymer) -> Self {
    let start = p.0.first().copied().unwrap();
    let end = p.0.last().copied().unwrap();
    let mut pairs = HashMap::new();
    for k in p.0.iter().copied().tuple_windows() {
      let e = pairs.entry(k).or_insert(0);
      *e += 1;
    }
    Self { pairs, start, end }
  }
}

// TODO: better way to do this than a `one_of`;
pub fn parse_base(input: &str) -> IResult<&str, Base> {
  one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  let polymer = map(many1(parse_base), Polymer);
  let rule = separated_pair(tuple((parse_base, parse_base)), tag(" -> "), parse_base);
  let rules = lines_of(rule);
  map(
    tuple((delimited(multispace0, polymer, multispace0), rules)),
    |(p, rules)| (p, rules.into_iter().collect::<PolymerizationRules>()),
  )(input)
}

pub fn p1((mut poly, rules): Input) -> usize {
  for _ in 0..10 {
    poly = poly.step(&rules);
  }
  let counts = poly.0.into_iter().counts();
  let max = counts.iter().map(|(_, c)| c).max().unwrap();
  let min = counts.iter().map(|(_, c)| c).min().unwrap();
  max - min
}

pub fn aborted_p2((mut poly, rules): Input) -> usize {
  // Too large for brute force solution;  need to find a way to simplify the problem.

  // This is not the general answer, but we can solve it for each pair after 20 iterations, memoize the results, then use the memoized results
  let mut memos = HashMap::new();

  // Most of these will be empty.
  for (a, b) in ('A'..='Z').cartesian_product('A'..='Z') {
    let mut p = Polymer(vec![a, b]);
    for _ in 0..20 {
      p = p.step(&rules);
    }
    memos.insert((a, b), p.0.into_iter().counts());
  }

  // get halfway there
  for _ in 0..20 {
    poly = poly.step(&rules);
  }

  // then use the memoized values to go the rest of the way
  let final_counts = poly
    .0
    .into_iter()
    .tuple_windows()
    .fold(HashMap::new(), |acc, (a, b)| {
      let c = memos.get(&(a, b)).unwrap();
      add_hash_maps(&acc, c)
    });

  for (k, v) in final_counts.iter() {
    println!("{:?}: {:?}", k, v);
  }

  let max = final_counts.iter().map(|(_, c)| c).max().unwrap();
  let min = final_counts.iter().map(|(_, c)| c).min().unwrap();
  println!("max {:?}", max);
  println!("min {:?}", min);
  max - min

  // This solution is still orders of magnitude slower than what I think an optimal solution would be.
}

pub fn p2((poly, rules): Input) -> usize {
  // duh, the answer is to memoize how many of each _pair_ you get after each generation.
  let mut poly = BetterPolymer::from(poly);
  for _ in 0..40 {
    poly = poly.step(&rules);
  }
  let final_counts = poly.letter_counts();
  let max = final_counts.iter().map(|(_, c)| c).max().unwrap();
  let min = final_counts.iter().map(|(_, c)| c).min().unwrap();
  println!("max {:?}", max);
  println!("min {:?}", min);
  max - min
}

fn add_hash_maps(m1: &HashMap<Base, usize>, m2: &HashMap<Base, usize>) -> HashMap<Base, usize> {
  let mut new_m = HashMap::new();
  for (k, v) in m1.iter().chain(m2.iter()) {
    let e = new_m.entry(*k).or_insert(0);
    *e += v;
  }
  new_m
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "NNCB

  CH -> B
  HH -> N
  CB -> H
  NH -> C
  HB -> C
  HC -> B
  HN -> C
  NN -> C
  BH -> H
  NC -> B
  NB -> B
  BN -> B
  BB -> N
  BC -> B
  CC -> N
  CN -> C";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(parsed.0, Polymer(vec!['N', 'N', 'C', 'B']));
    assert_eq!(parsed.1.get(&('C', 'H')), Some(&'B'));
    assert_eq!(parsed.1.get(&('C', 'N')), Some(&'C'))
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 1588);

    let input = std::fs::read_to_string("./inputs/d14.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 2975);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 2188189693529);

    let input = std::fs::read_to_string("./inputs/d14.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 3015383850689);
  }
}
