/// # Reflection
///
/// In part two, you actually only need to compare the first number of the first triple with the third number of the second triple.  If you use this approach, p2 only differs from p1 only in how much you skip in the second iterator.
///
/// I had the amusing experience of half-way realizing this while I was writing my tests.  Somehow, I did not make the small additional leap to realizing that I could use this in my implementation.  My eye was too much on the goal, I guess.  On the other hand, I suppose that "make it work" comes before "make it fast", so it was reasonable to do a slower straightforward solution over one with a clever trick.  (Ok, the trick isn't that clever after all; but still!)
use itertools::Itertools;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

// Yes, the parser is overkill and so is the newtype.
#[derive(Debug, Eq, PartialEq)]
pub struct Depth(u32);

pub fn parse(input: &str) -> IResult<&str, Vec<Depth>> {
  let depth = map(map_res(digit1, |s: &str| s.parse::<u32>()), Depth);
  many1(terminated(depth, multispace0))(input)
}

pub fn p1(depths: Vec<Depth>) -> usize {
  depths
    .iter()
    .zip(depths.iter().skip(1))
    .filter(|(d1, d2)| d1.0 < d2.0)
    .count()
}

pub fn p2(depths: Vec<Depth>) -> usize {
  depths
    .iter()
    .tuple_windows()
    .map(|(x, y, z)| x.0 + y.0 + z.0)
    .tuple_windows()
    .filter(|(x, y)| x < y)
    .count()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let input = "123
321";
    assert_eq!(parse(input).unwrap(), ("", vec![Depth(123), Depth(321)]))
  }

  #[test]
  fn test_p1() {
    let input = "123
321
123";
    let ds = parse(input).unwrap().1;
    assert_eq!(p1(ds), 1);

    let input = std::fs::read_to_string("./inputs/d01.txt").unwrap();
    let ds = parse(&input).unwrap().1;
    assert_eq!(p1(ds), 1624);
  }

  #[test]
  fn test_p2() {
    let input = "123
321
123
321
123";
    let ds = parse(input).unwrap().1;
    assert_eq!(p2(ds), 1);

    let input = std::fs::read_to_string("./inputs/d01.txt").unwrap();
    let ds = parse(&input).unwrap().1;
    assert_eq!(p2(ds), 1653);
  }
}
