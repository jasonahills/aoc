type InputItem = ();

pub fn parse(input: &str) -> IResult<&str, Vec<Direction>> {
  unimplemented!()
}

pub fn p1(_xs: Vec<InputItem>) -> usize {
  0
}

pub fn p2(_xs: Vec<InputItem>) -> usize {
  0
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let input = "";
    assert_eq!(parse(input).finalize().unwrap(), ("", vec![]))
  }

  #[test]
  fn test_p1() {
    let input = "";
    let parsed = parse(input).finalize().unwrap();
    assert_eq!(p1(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d00.txt").unwrap();
    let parsed = parse(&input).finalize().unwrap();
    assert_eq!(p1(parsed), 0);
  }

  #[test]
  fn test_p2() {
    let input = "";
    let parsed = parse(input).finalize().unwrap();
    assert_eq!(p2(parsed), 0);

    let input = std::fs::read_to_string("./inputs/d00.txt").unwrap();
    let parsed = parse(&input).finalize().unwrap();
    assert_eq!(p2(parsed), 0);
  }
}
