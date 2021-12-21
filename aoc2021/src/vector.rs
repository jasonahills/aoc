#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct V(pub i32, pub i32);

impl std::ops::Add for V {
  type Output = Self;
  fn add(self, other: Self) -> Self {
    Self(self.0 + other.0, self.1 + other.1)
  }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
pub struct V3(pub i32, pub i32, pub i32);

impl V3 {
  /// # Panics
  ///
  /// Panics if index out of range
  pub const fn get(self, i: usize) -> i32 {
    match i {
      0 => self.0,
      1 => self.1,
      2 => self.2,
      _ => panic!("out of range"),
    }
  }

  pub fn manhattan(self, other: V3) -> i32 {
    (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
  }
}

impl std::ops::Add for V3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

impl std::ops::Sub for V3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}
