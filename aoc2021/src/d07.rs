use nom::IResult;

type InputItem = Crab;

#[derive(Debug, Eq, PartialEq)]
pub struct Crab(u32);

pub fn parse(input: &str) -> IResult<&str, Vec<Crab>> {
  nom::multi::separated_list1(
    nom::bytes::complete::tag(","),
    nom::combinator::map(crate::util::parse_u32, Crab),
  )(input)
}

pub fn p1(mut xs: Vec<InputItem>) -> u32 {
  // The median should minimize the error.  For any two entries, any number chosen between them will give the same error as any other number chosen between them (which is better than a number chosen outside of them).  The median maximizes how many entries we have "split".
  xs.sort_by_key(|x| x.0);
  // Add one so that if odd we get the middle value.  If even we get the upper "median", which will give the same error as the "lower" one.
  let index = (xs.len() + 1) / 2;
  let median = xs[index].0;
  xs.iter()
    .map(|x| (x.0 as i32 - median as i32).abs())
    .sum::<i32>() as u32
}

pub fn p2(xs: Vec<InputItem>) -> u32 {
  // Brute force
  // I'm actually surprised that the mean didn't work right out of the box (must be something weird about the integer values; I'm guessing that the rational mean would work just fine with the same loss function.)
  let min = xs.iter().min_by_key(|x| x.0).unwrap().0;
  let max = xs.iter().max_by_key(|x| x.0).unwrap().0;
  (min..=max)
    .map(|alignment| {
      xs.iter()
        .map(|x| {
          let n = (x.0 as i32 - alignment as i32).abs();
          (n * (n + 1)) / 2
        })
        .sum::<i32>() as u32
    })
    .min()
    .unwrap()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "16,1,2,0,4,2,7,1,2,14";

  #[test]
  fn test_parse() {
    let input = "1,2,3";
    assert_eq!(parse(input).unwrap(), ("", vec![Crab(1), Crab(2), Crab(3)]))
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 37);

    let input = std::fs::read_to_string("./inputs/d07.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 337833);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 168);

    let input = std::fs::read_to_string("./inputs/d07.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 96678103);
  }
}
