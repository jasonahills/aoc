use crate::nom_prelude::*;

pub fn parse(input: &str) -> IResult<&str, Vec<&str>> {
  Ok(("", input.lines().collect()))
}

fn score_error(c: char) -> u32 {
  match c {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
    c => {
      println!("non-matching character {:?}", c);
      0
    }
  }
}

pub fn p1(input: Vec<&str>) -> u32 {
  input
    .iter()
    .map(|line| {
      let mut stack = Vec::new();
      for c in line.chars().filter(|c| "{}[]<>()".contains(*c)) {
        match c {
          '{' => stack.push('}'),
          '[' => stack.push(']'),
          '<' => stack.push('>'),
          '(' => stack.push(')'),
          c => match stack.last() {
            Some(last) if c == *last => {
              stack.pop();
            }
            _ => {
              return score_error(c);
            }
          },
        }
      }
      0
    })
    .sum::<u32>()
}

pub fn p2(input: Vec<&str>) -> u64 {
  let mut scores = input
    .iter()
    .filter_map(|line| {
      let mut stack = Vec::new();
      for c in line.chars().filter(|c| "{}[]<>()".contains(*c)) {
        match c {
          '{' => stack.push('}'),
          '[' => stack.push(']'),
          '<' => stack.push('>'),
          '(' => stack.push(')'),
          c => match stack.last() {
            Some(last) if c == *last => {
              stack.pop();
            }
            _ => return None,
          },
        }
      }
      Some(stack.into_iter().rev().fold(0_u64, |acc, c| {
        (acc * 5)
          + match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("non matching character"),
          }
      }))
    })
    .collect::<Vec<_>>();
  scores.sort();
  scores[scores.len() / 2]
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = r#"[({(<(())[]>[[{[]{<()<>>
    [(()[<>])]({[<{<<[]>>(
    {([(<{}[<>[]}>{[]{[(<()>
    (((({<>}<{<{<>}{[]{[]{}
    [[<[([]))<([[{}[[()]]]
    [{[{({}]{}}([{[{{{}}([]
    {<[[]]>}<{[{[{[]{()[[[]
    [<(<(<(<{}))><([]([]()
    <{([([[(<>()){}]>(<<{{
    <{([{{}}[<[[[<>{}]]]>[]]"#;

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    assert_eq!(parse(input).unwrap().1.len(), 10)
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 26397);

    let input = std::fs::read_to_string("./inputs/d10.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 266301);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 288957);

    let input = std::fs::read_to_string("./inputs/d10.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 3404870164);
  }
}
