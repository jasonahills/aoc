//! https://adventofcode.com/2021/day/17

use crate::nom_prelude::*;
use crate::util::V;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub struct Target {
  xmin: i32,
  xmax: i32,
  ymin: i32,
  ymax: i32,
}

impl Target {
  fn contains(&self, v: V) -> bool {
    self.xmin <= v.0 && v.0 <= self.xmax && self.ymin <= v.1 && v.1 <= self.ymax
  }

  fn trajectory_hits(&self, mut v: V) -> bool {
    let mut p = V(0, 0);
    while v.1 >= 0 || v.1 >= self.ymin {
      p = p + v;
      let drag = match v.0 {
        v_x if v_x > 0 => -1,
        v_x if v_x < 0 => 1,
        _ => 0,
      };
      v = v + V(drag, -1);
      if self.contains(p) {
        return true;
      }
    }
    false
  }
}

type Input = Target;

pub fn parse(input: &str) -> IResult<&str, Input> {
  // target area: x=20..30, y=-10..-5
  let (input, (_, xmin, _, xmax, _, ymin, _, ymax)) = tuple((
    tag("target area: x="),
    parse_i32,
    tag(".."),
    parse_i32,
    tag(", y="),
    parse_i32,
    tag(".."),
    parse_i32,
  ))(input)?;
  Ok((
    input,
    Target {
      xmin,
      xmax,
      ymin,
      ymax,
    },
  ))
}

pub fn p1(target: Input) -> usize {
  // We could brute force this, but I wonder if it can be done analytically (I was able to do it by hand for my input, but only because I had some "full stop" trajectories which worked; with too narrow a hit box, this would not be possible).
  // Solve for range of steps.
  // steps_to_stopped = v0
  // final_dist_traveled = (v0 * (v0 + 1)) / 2 => 0 = v0^2 + v0 - (final_dist_traveled * 2)
  // given a final_dist_traveled, only interested in when that is traveled in the positive direction, so

  // Any elevation reached on the way up will also be reached on the way down, so if y=0 passes through the target area, then _any_ height
  // If x=0 passes through

  // brute force for now
  // don't need to explore any more than direct hit
  let x = target.xmin.abs().max(target.xmax.abs());
  let lower_x = -1 * x;
  let y = target.ymin.abs().max(target.ymax.abs());
  let lower_y = -1 * y;
  (lower_x..=x)
    .cartesian_product(lower_y..=y)
    .filter(|(x, y)| target.trajectory_hits(V(*x, *y)))
    .map(|(_, y)| (y * (y + 1)) / 2)
    .max()
    .unwrap() as usize
}

pub fn p2(target: Input) -> usize {
  // brute force for now
  // don't need to explore any more than direct hit
  let x = target.xmin.abs().max(target.xmax.abs());
  let lower_x = -1 * x;
  let y = target.ymin.abs().max(target.ymax.abs());
  let lower_y = -1 * y;

  (lower_x..=x)
    .cartesian_product(lower_y..=y)
    .filter(|(x, y)| target.trajectory_hits(V(*x, *y)))
    .count()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "target area: x=20..30, y=-10..-5";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        Target {
          xmin: 20,
          xmax: 30,
          ymin: -10,
          ymax: -5
        }
      )
    )
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 45);

    let input = std::fs::read_to_string("./inputs/d17.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 7750);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;

    assert!(parsed.trajectory_hits(V(6, 7)));
    assert_eq!(p2(parsed), 112);

    let input = std::fs::read_to_string("./inputs/d17.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 4120);
  }
}
