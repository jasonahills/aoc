use std::collections::HashMap;
use std::hash::Hash;

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
