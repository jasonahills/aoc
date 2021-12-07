use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub struct LanternFish(u64);

pub struct School([u64; 9]);

impl School {
  pub fn from_fish(fishes: Vec<LanternFish>) -> Self {
    let mut counter = crate::util::HashCounter::new();
    for fish in fishes {
      counter.inc(fish.0);
    }
    let mut fishes = [0; 9];
    for (cohort, number) in counter.iter() {
      fishes[*cohort as usize] = number as u64;
    }
    School(fishes)
  }

  pub fn next_gen(&self) -> Self {
    let mut next_cohorts = [0; 9];
    let new_fish = self.0[0];
    for i in 1..next_cohorts.len() {
      next_cohorts[i - 1] = self.0[i];
    }
    next_cohorts[6] += new_fish;
    next_cohorts[8] += new_fish;
    Self(next_cohorts)
  }

  pub fn total_fish(&self) -> u64 {
    self.0.iter().sum()
  }
}

pub fn parse(input: &str) -> IResult<&str, Vec<LanternFish>> {
  nom::multi::separated_list1(
    nom::bytes::complete::tag(","),
    nom::combinator::map(crate::util::parse_u32, |u| LanternFish(u as u64)),
  )(input)
}

pub fn p1(xs: Vec<LanternFish>) -> u64 {
  let mut school = School::from_fish(xs);
  for _ in 0..80 {
    school = school.next_gen()
  }
  school.total_fish()
}

pub fn p2(xs: Vec<LanternFish>) -> u64 {
  let mut school = School::from_fish(xs);
  for _ in 0..256 {
    school = school.next_gen()
  }
  school.total_fish()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "3,4,3,1,2";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        vec![
          LanternFish(3),
          LanternFish(4),
          LanternFish(3),
          LanternFish(1),
          LanternFish(2),
        ]
      )
    )
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 5934);

    let input = std::fs::read_to_string("./inputs/d06.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 350149);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 26984457539);

    let input = std::fs::read_to_string("./inputs/d06.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 1590327954513);
  }
}
