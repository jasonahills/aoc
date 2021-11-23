use nom::branch::alt;
use nom::character::complete::char;
use nom::multi::many1;
use nom::IResult;

#[derive(Debug, Eq, PartialEq)]
pub enum Direction {
  Up,
  Down,
}

fn up(input: &str) -> IResult<&str, Direction> {
  let (input, _) = char('(')(input)?;
  Ok((input, Direction::Up))
}

fn down(input: &str) -> IResult<&str, Direction> {
  let (input, _) = char(')')(input)?;
  Ok((input, Direction::Down))
}

fn direction(input: &str) -> IResult<&str, Direction> {
  alt((up, down))(input)
}

pub fn directions(input: &str) -> IResult<&str, Vec<Direction>> {
  many1(direction)(input)
}

pub fn p1(input: &[Direction]) -> i32 {
  input.iter().fold(0, |acc, d| match d {
    Direction::Up => acc + 1,
    Direction::Down => acc - 1,
  })
}

pub fn p2(input: &[Direction]) -> i32 {
  input
    .iter()
    .scan((0, 0_i32), |acc, d| {
      match d {
        Direction::Up => acc.1 += 1,
        Direction::Down => acc.1 -= 1,
      }
      acc.0 += 1; // Could use enumerate, but since we're using scan anyway...
      Some(acc.clone())
    })
    .find(|(_, current_floor)| *current_floor == -1)
    .unwrap()
    .0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn parse() {
    use super::Direction::*;
    assert_eq!(
      directions("(())(").unwrap(),
      ("", vec!(Up, Up, Down, Down, Up))
    );
  }

  #[test]
  fn p1() {
    let input = "(())(";
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p1(&ds), 1);

    let input = std::fs::read_to_string("./inputs/d01.txt").unwrap();
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p1(&ds), 138);
  }

  #[test]
  fn p2() {
    let input = "(()))((";
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p2(&ds), 5);

    let input = std::fs::read_to_string("./inputs/d01.txt").unwrap();
    let (_, ds) = directions(&input).unwrap();
    assert_eq!(super::p2(&ds), 1771);
  }
}
