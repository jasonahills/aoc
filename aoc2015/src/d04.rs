pub const INPUT: &str = "ckczppom";

pub fn p1(input: &str) -> u32 {
  let mut i = 0;
  loop {
    let input = format!("{}{}", input, i);
    let hash = md5::compute(&input);
    let hash_string = format!("{:x}", hash);
    if hash_string.starts_with("00000") {
      return i;
    }
    i += 1;
  }
}

pub fn p2(input: &str) -> u32 {
  let mut i = 0;
  loop {
    let input = format!("{}{}", input, i);
    let hash = md5::compute(&input);
    let hash_string = format!("{:x}", hash);
    if hash_string.starts_with("000000") {
      return i;
    }
    i += 1;
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn p1() {
    assert_eq!(super::p1(INPUT), 117946);
  }

  #[test]
  fn p2() {
    assert_eq!(super::p2(INPUT), 3938038);
  }
}
