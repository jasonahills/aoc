use nom::character::complete::digit1;
use nom::combinator::map_res;
use nom::IResult;
use std::collections::HashMap;
use std::hash::Hash;

// It's a little silly that I'm using this rather than the u32 parser provided by nom, but the types work out nicely.
pub fn parse_u32(input: &str) -> IResult<&str, u32> {
  map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

pub struct HashCounter<Key: Hash + Eq>(HashMap<Key, u32>);

impl<Key: Hash + Eq> HashCounter<Key> {
  pub fn new() -> Self {
    Self(HashMap::new())
  }

  pub fn inc(&mut self, key: Key) -> u32 {
    let e = self.0.entry(key).or_insert(0);
    *e = *e + 1;
    *e
  }

  /// Iterates over non-zero values
  pub fn iter(&self) -> impl Iterator<Item = (&Key, u32)> {
    self.0.iter().map(|(k, v)| (k, *v))
  }
}
