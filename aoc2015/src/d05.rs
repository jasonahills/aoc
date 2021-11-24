fn is_nice(s: &str) -> bool {
  let enough_vowels = s
    .chars()
    .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
    .count()
    >= 3;
  let has_double = s.chars().zip(s.chars().skip(1)).any(|(c1, c2)| c1 == c2);
  let no_invalids = !(s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy"));
  enough_vowels && has_double && no_invalids
}

fn has_double_double(s: &str) -> bool {
  // n^2 oh well
  for ((i, c1), c2) in s.chars().enumerate().zip(s.chars().skip(1)) {
    for (c3, c4) in s.chars().zip(s.chars().skip(1)).skip(i + 2) {
      if c1 == c3 && c2 == c4 {
        return true;
      }
    }
  }
  false
}

fn better_is_nice(s: &str) -> bool {
  let one_between = s.chars().zip(s.chars().skip(2)).any(|(c1, c2)| c1 == c2);
  one_between && has_double_double(s)
}

pub fn p1(input: &[&str]) -> usize {
  input.iter().filter(|s| is_nice(s)).count()
}

pub fn p2(input: &[&str]) -> usize {
  input.iter().filter(|s| better_is_nice(s)).count()
}

#[cfg(test)]
mod test {
  #[test]
  fn p1() {
    let input = "asdf
aaa
aaab";
    let input = input.lines().collect::<Vec<_>>();
    assert_eq!(super::p1(&input), 1);

    let input = std::fs::read_to_string("./inputs/d05.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();
    assert_eq!(super::p1(&input), 255);
  }

  #[test]
  fn p2() {
    let input = "asdf
aaa
abab";
    let input = input.lines().collect::<Vec<_>>();
    assert_eq!(super::p2(&input), 1);

    let input = std::fs::read_to_string("./inputs/d05.txt").unwrap();
    let input = input.lines().collect::<Vec<_>>();
    assert_eq!(super::p2(&input), 55);
  }
}
