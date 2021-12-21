//! https://adventofcode.com/2021/day/21

use crate::nom_prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DiracStart {
  pos1: u32,
  pos2: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Dirac {
  pos1: u32,
  score1: u32,
  pos2: u32,
  score2: u32,
  rolls: u32,
  is_p1_turn: bool,
}

impl Dirac {
  fn play(&mut self, r1: u32, r2: u32, r3: u32) -> bool {
    self.rolls += 3;
    let over = if self.is_p1_turn {
      self.pos1 = ((self.pos1 - 1 + r1 + r2 + r3) % 10) + 1;
      self.score1 += self.pos1;
      self.score1 >= 1000
    } else {
      self.pos2 = ((self.pos2 - 1 + r1 + r2 + r3) % 10) + 1;
      self.score2 += self.pos2;
      self.score2 >= 1000
    };
    self.is_p1_turn = !self.is_p1_turn;
    over
  }
}

impl From<DiracStart> for Dirac {
  fn from(d: DiracStart) -> Self {
    Self {
      pos1: d.pos1,
      score1: 0,
      pos2: d.pos2,
      score2: 0,
      rolls: 0,
      is_p1_turn: true,
    }
  }
}

#[derive(Debug, Eq, PartialEq)]
pub struct RealDiracPlayer {
  pos_score: [[u128; 21]; 10],
}

impl RealDiracPlayer {
  fn from_starting_pos(starting_pos: u128) -> Self {
    let mut pos_score = [[0; 21]; 10];
    pos_score[(starting_pos - 1) as usize][0] = 1;
    Self { pos_score }
  }

  fn num_games(&self) -> u128 {
    self.pos_score.iter().flat_map(|scores| scores.iter()).sum()
  }

  /// Returns number of games which would end after this roll.
  fn roll(&mut self) -> u128 {
    let start_num_games = self.num_games();
    let total_games_after_roll = start_num_games * 27;

    // 3: #       (1,1,1)
    // 4: ###     (2,1,1) x 3
    // 5: ######  (3,1,1) x 3 , (2,2,1) x 3
    // 6: ####### (3,2,1) x 6 , (2,2,2)
    // 7: ######  (3,3,1) x 3 , (3,2,2) x 3
    // 8: ###     (3,3,2) x 3
    // 9: #       (3,3,3)
    let outcomes = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

    let mut new_pos_score = [[0; 21]; 10];
    for start_pos in 0..10 {
      for start_score in 0..=20 {
        for (pos_delta, new_games) in outcomes.iter().enumerate() {
          let next_pos = (start_pos + pos_delta) % 10;
          let next_score = start_score + next_pos + 1;
          if next_score < 21 {
            new_pos_score[next_pos][next_score] +=
              self.pos_score[start_pos][start_score] * new_games
          }
        }
      }
    }
    self.pos_score = new_pos_score;

    let remaining_games_after_roll = self.num_games();
    total_games_after_roll - remaining_games_after_roll
  }
}

type Input = DiracStart;

pub fn parse(input: &str) -> IResult<&str, Input> {
  map(
    tuple((
      tag("Player 1 starting position: "),
      parse_u32,
      tag("\nPlayer 2 starting position: "),
      parse_u32,
    )),
    |(_, pos1, _, pos2)| DiracStart { pos1, pos2 },
  )(input)
}

pub fn p1(input: Input) -> u32 {
  let mut game = Dirac::from(input);
  let mut nums = (1..=100_u32).cycle();
  loop {
    let r1 = nums.next().unwrap();
    let r2 = nums.next().unwrap();
    let r3 = nums.next().unwrap();
    if game.play(r1, r2, r3) {
      return game.score1.min(game.score2) * game.rolls;
    }
  }
}

pub fn p2(input: Input) -> u128 {
  let mut d1 = RealDiracPlayer::from_starting_pos(input.pos1 as u128);
  let mut d2 = RealDiracPlayer::from_starting_pos(input.pos2 as u128);
  // Because we are dealing with each player separately, we need to account for how many universes get created between plays
  let mut p1_wins = 0;
  let mut p2_wins = 0;
  for _ in 0..20 {
    p1_wins += d1.roll() * d2.num_games();
    p2_wins += d2.roll() * d1.num_games();
  }

  p1_wins.max(p2_wins)
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";

  #[test]
  fn test_parse() {
    let input = TEST_INPUT;
    assert_eq!(parse(input).unwrap(), ("", DiracStart { pos1: 4, pos2: 8 }))
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 739785);

    let input = std::fs::read_to_string("./inputs/d21.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 551901);
  }

  #[test]
  fn test_p2() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p2(parsed), 444356092776315);

    let input = std::fs::read_to_string("./inputs/d21.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p2(parsed), 272847859601291);
  }
}
