//! https://adventofcode.com/2021/day/20

use crate::nom_prelude::*;
use itertools::Itertools;
use std::collections::HashSet;

pub struct ImageEnhance {
  alg: [bool; 512],
  image: HashSet<(i32, i32)>,
  model_bounds: (i32, i32, i32, i32),
  infinite_pixel_val: bool,
}

impl ImageEnhance {
  pub fn step(&mut self) {
    let (xmin, xmax, ymin, ymax) = self.model_bounds;
    let mut image = HashSet::new();
    for (x, y) in (xmin..=xmax).cartesian_product(ymin..=ymax) {
      let mut i = 0;
      let mut idx = 0;
      for y_prime in ((y - 1)..=(y + 1)).rev() {
        for x_prime in ((x - 1)..=(x + 1)).rev() {
          if x_prime < xmin || x_prime > xmax || y_prime < ymin || y_prime > ymax {
            if self.infinite_pixel_val {
              idx = idx | (1 << i);
            }
          }
          if self.image.contains(&(x_prime, y_prime)) {
            idx = idx | (1 << i);
          }
          i += 1;
        }
      }
      if self.alg[idx] {
        image.insert((x, y));
      }
    }

    self.image = image;
    self.infinite_pixel_val = if self.infinite_pixel_val {
      self.alg[511]
    } else {
      self.alg[0]
    };
    self.model_bounds = (xmin - 1, xmax + 1, ymin - 1, ymax + 1);
    if self.infinite_pixel_val {
      for x in (xmin - 1)..=(xmax + 1) {
        self.image.insert((x, ymin - 1));
        self.image.insert((x, ymax + 1));
      }
      for y in (ymin - 1)..=(ymax + 1) {
        self.image.insert((xmin - 1, y));
        self.image.insert((xmax + 1, y));
      }
    }
  }
}

impl std::fmt::Display for ImageEnhance {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for b in self.alg {
      if b {
        write!(f, "#")?;
      } else {
        write!(f, ".")?;
      }
    }

    write!(f, "\n\n")?;

    let xmax = self.image.iter().map(|(x, _)| *x).max().unwrap();
    let xmin = self.image.iter().map(|(x, _)| *x).min().unwrap();
    let ymax = self.image.iter().map(|(_, y)| *y).max().unwrap();
    let ymin = self.image.iter().map(|(_, y)| *y).min().unwrap();

    for y in ymin..=ymax {
      for x in xmin..=xmax {
        if self.image.contains(&(x, y)) {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}

type Input = ImageEnhance;

pub fn parse_bit(input: &str) -> IResult<&str, bool> {
  alt((map(tag("."), |_| false), map(tag("#"), |_| true)))(input)
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  let alg = map_res(many1(parse_bit), |xs| xs.try_into());
  let image = map(lines_of(many1(parse_bit)), |xxs| {
    xxs
      .into_iter()
      .enumerate()
      .flat_map(|(row, xs)| {
        xs.into_iter()
          .enumerate()
          .filter(|(_, b)| *b)
          .map(move |(col, _)| (col as i32, row as i32))
      })
      .collect()
  });
  map(
    tuple((alg, image)),
    |(alg, image): ([bool; 512], HashSet<(i32, i32)>)| {
      let xmax = image.iter().map(|(x, _)| x).max().unwrap() + 1;
      let xmin = image.iter().map(|(x, _)| x).min().unwrap() - 1;
      let ymax = image.iter().map(|(_, y)| y).max().unwrap() + 1;
      let ymin = image.iter().map(|(_, y)| y).min().unwrap() - 1;
      ImageEnhance {
        alg,
        image,
        infinite_pixel_val: false,
        model_bounds: (xmin, xmax, ymin, ymax),
      }
    },
  )(input)
}

pub fn p1(mut input: Input) -> usize {
  input.step();
  input.step();
  input.image.len()
}

pub fn p2(mut input: Input) -> usize {
  for _ in 0..50 {
    input.step();
  }
  input.image.len()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let input = std::fs::read_to_string("./inputs/d20-test.txt").unwrap();
    let img_enh = parse(&input).unwrap().1;
    assert!(!img_enh.alg[0]);
    assert!(img_enh.alg[2]);
    assert!(img_enh.alg[511]);

    assert!(img_enh.image.contains(&(0, 0)));
    assert!(img_enh.image.contains(&(1, 2)));
    assert!(img_enh.image.contains(&(4, 4)));
    assert!(!img_enh.image.contains(&(1, 0)));
    assert!(!img_enh.image.contains(&(4, 3)));
  }

  #[test]
  fn test_p1() {
    let input = std::fs::read_to_string("./inputs/d20-test.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 35);

    let input = std::fs::read_to_string("./inputs/d20.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 5479);
  }

  #[test]
  fn test_p2() {
    let input = std::fs::read_to_string("./inputs/d20-test.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 3351);

    let input = std::fs::read_to_string("./inputs/d20.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 19012);
  }
}
