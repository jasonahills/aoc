//! https://adventofcode.com/2021/day/22

use itertools::Itertools;

use crate::nom_prelude::*;
use crate::vector::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CubeState {
  On,
  Off,
}

// Small enough that it's worth making it copy for ease of use.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Cuboid {
  /// all three of these coordinates should be smaller than the `maxes`
  min: V3,
  /// This coordinate is just outside of the cuboid
  max: V3,
}

impl Cuboid {
  fn from_v3s(v1: V3, v2: V3) -> Self {
    Self {
      min: V3(v1.0.min(v2.0), v1.1.min(v2.1), v1.2.min(v2.2)),
      max: V3(v1.0.max(v2.0) + 1, v1.1.max(v2.1) + 1, v1.2.max(v2.2) + 1),
    }
  }

  fn intersection(self, rhs: Self) -> Option<Self> {
    let xmin = self.min.0.max(rhs.min.0);
    let xmax = self.max.0.min(rhs.max.0);
    let ymin = self.min.1.max(rhs.min.1);
    let ymax = self.max.1.min(rhs.max.1);
    let zmin = self.min.2.max(rhs.min.2);
    let zmax = self.max.2.min(rhs.max.2);
    if xmin >= xmax || ymin >= ymax || zmin >= zmax {
      None
    } else {
      Some(Self {
        min: V3(xmin, ymin, zmin),
        max: V3(xmax, ymax, zmax),
      })
    }
  }

  fn points(&self) -> Vec<V3> {
    (self.min.0..self.max.0)
      .cartesian_product(self.min.1..self.max.1)
      .cartesian_product(self.min.2..self.max.2)
      .map(|((x, y), z)| V3(x, y, z))
      .collect()
  }

  fn volume(&self) -> u64 {
    (self.max.0 - self.min.0) as u64
      * (self.max.1 - self.min.1) as u64
      * (self.max.2 - self.min.2) as u64
  }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
  state: CubeState,
  cuboid: Cuboid,
}

type Input = Vec<Instruction>;

pub struct LightCube(HashSet<V3>);

impl LightCube {
  fn new() -> Self {
    Self(HashSet::new())
  }

  fn apply(&mut self, state: CubeState, cuboid: Cuboid) {
    match state {
      CubeState::On => cuboid.points().into_iter().for_each(|p| {
        self.0.insert(p);
      }),
      CubeState::Off => cuboid.points().into_iter().for_each(|p| {
        self.0.remove(&p);
      }),
    }
  }

  fn count_lights(&self) -> usize {
    self.0.len()
  }

  fn lights<'a>(&'a self) -> impl Iterator<Item = V3> + 'a {
    self.0.iter().copied()
  }
}

pub fn parse(input: &str) -> IResult<&str, Input> {
  let cube_state = map(alt((tag("on"), tag("off"))), |s| match s {
    "on" => CubeState::On,
    "off" => CubeState::Off,
    _ => panic!("unknown tag"),
  });
  let cuboid = map(
    tuple((
      tag("x="),
      parse_i32,
      tag(".."),
      parse_i32,
      tag(",y="),
      parse_i32,
      tag(".."),
      parse_i32,
      tag(",z="),
      parse_i32,
      tag(".."),
      parse_i32,
    )),
    |(_, x1, _, x2, _, y1, _, y2, _, z1, _, z2)| Cuboid::from_v3s(V3(x1, y1, z1), V3(x2, y2, z2)),
  );
  let instruction = map(
    tuple((cube_state, tag(" "), cuboid)),
    |(state, _, cuboid)| Instruction { state, cuboid },
  );
  lines_of(instruction)(input)
}

pub fn p1(input: Input) -> usize {
  let window = Cuboid::from_v3s(V3(-50, -50, -50), V3(50, 50, 50));
  let mut grid = LightCube::new();
  for (state, cuboid) in input
    .into_iter()
    .filter_map(|i| Some((i.state, i.cuboid.intersection(window)?)))
  {
    grid.apply(state, cuboid);
  }
  grid.count_lights()
}

pub fn p2(input: Input) -> u64 {
  // lol, still pretty much brute force, though brought into the realm of feasibility.
  // TODO: think about an actually efficient way to do this.
  let xs = input
    .iter()
    .flat_map(|i| [i.cuboid.min.0, i.cuboid.max.0])
    .sorted()
    .unique()
    .collect_vec();
  let ys = input
    .iter()
    .flat_map(|i| [i.cuboid.min.1, i.cuboid.max.1])
    .sorted()
    .unique()
    .collect_vec();
  let zs = input
    .iter()
    .flat_map(|i| [i.cuboid.min.2, i.cuboid.max.2])
    .sorted()
    .unique()
    .collect_vec();
  println!("xs {:?}", xs);
  let possible_entries = xs
    .iter()
    .tuple_windows()
    .cartesian_product(ys.iter().tuple_windows())
    .cartesian_product(zs.iter().tuple_windows())
    .map(|(((x1, x2), (y1, y2)), (z1, z2))| {
      Cuboid::from_v3s(V3(*x1, *y1, *z1), V3(*x2 - 1, *y2 - 1, *z2 - 1))
    })
    .collect_vec();

  let mut actual_entries = HashSet::new();

  for instruction in input {
    match instruction.state {
      CubeState::On => possible_entries
        .iter()
        .filter(|p| p.intersection(instruction.cuboid).is_some())
        .for_each(|e| {
          actual_entries.insert(*e);
        }),
      CubeState::Off => possible_entries
        .iter()
        .filter(|p| p.intersection(instruction.cuboid).is_some())
        .for_each(|e| {
          actual_entries.remove(e);
        }),
    }
  }
  actual_entries.iter().map(|e| e.volume()).sum::<u64>()
}

pub fn better_p2(input: Input) -> u64 {
  // lol, still pretty much brute force, though brought into the realm of feasibility.
  // TODO: think about an actually efficient way to do this.
  let xs = input
    .iter()
    .flat_map(|i| [i.cuboid.min.0, i.cuboid.max.0])
    .sorted()
    .unique()
    .collect_vec();
  let ys = input
    .iter()
    .flat_map(|i| [i.cuboid.min.1, i.cuboid.max.1])
    .sorted()
    .unique()
    .collect_vec();
  let zs = input
    .iter()
    .flat_map(|i| [i.cuboid.min.2, i.cuboid.max.2])
    .sorted()
    .unique()
    .collect_vec();
  println!("xs {:?}", xs);
  println!("lens {} {} {}", xs.len(), ys.len(), zs.len());
  // let possible_entries = xs
  //   .iter()
  //   .tuple_windows()
  //   .cartesian_product(ys.iter().tuple_windows())
  //   .cartesian_product(zs.iter().tuple_windows())
  //   .map(|(((x1, x2), (y1, y2)), (z1, z2))| {
  //     Cuboid::from_v3s(V3(*x1, *y1, *z1), V3(*x2 - 1, *y2 - 1, *z2 - 1))
  //   })
  //   .collect_vec();

  let x_indices = xs
    .iter()
    .enumerate()
    .map(|(i, x)| (*x, i))
    .collect::<HashMap<_, _>>();
  let y_indices = ys
    .iter()
    .enumerate()
    .map(|(i, y)| (*y, i))
    .collect::<HashMap<_, _>>();
  let z_indices = zs
    .iter()
    .enumerate()
    .map(|(i, z)| (*z, i))
    .collect::<HashMap<_, _>>();

  let x_indices_rev = xs
    .iter()
    .enumerate()
    .map(|(i, x)| (i, *x))
    .collect::<HashMap<_, _>>();
  let y_indices_rev = ys
    .iter()
    .enumerate()
    .map(|(i, y)| (i, *y))
    .collect::<HashMap<_, _>>();
  let z_indices_rev = zs
    .iter()
    .enumerate()
    .map(|(i, z)| (i, *z))
    .collect::<HashMap<_, _>>();

  // translate instructions to this new grid, perform
  let new_instructions = input
    .iter()
    .map(|i| {
      let Instruction {
        state,
        cuboid: Cuboid { min, max },
      } = i;
      let xmin = *x_indices.get(&min.0).unwrap() as i32;
      let ymin = *y_indices.get(&min.1).unwrap() as i32;
      let zmin = *z_indices.get(&min.2).unwrap() as i32;
      let xmax = (x_indices.get(&max.0).unwrap() - 1) as i32;
      let ymax = (y_indices.get(&max.1).unwrap() - 1) as i32;
      let zmax = (z_indices.get(&max.2).unwrap() - 1) as i32;
      Instruction {
        state: *state,
        cuboid: Cuboid::from_v3s(V3(xmin, ymin, zmin), V3(xmax, ymax, zmax)),
      }
    })
    .collect_vec();

  let mut grid = LightCube::new();
  for (i, inst) in new_instructions.iter().enumerate() {
    println!("new inst {:?}", i);
    grid.apply(inst.state, inst.cuboid);
  }

  grid
    .lights()
    .map(|v| {
      let x = v.0 as usize;
      let y = v.1 as usize;
      let z = v.2 as usize;
      (x_indices_rev.get(&(x + 1)).unwrap() - x_indices_rev.get(&x).unwrap()) as u64
        * (y_indices_rev.get(&(y + 1)).unwrap() - y_indices_rev.get(&y).unwrap()) as u64
        * (z_indices_rev.get(&(z + 1)).unwrap() - z_indices_rev.get(&z).unwrap()) as u64
    })
    .sum()

  // let mut actual_entries = HashSet::new();

  // for instruction in input {
  //   match instruction.state {
  //     CubeState::On => possible_entries
  //       .iter()
  //       .filter(|p| p.intersection(instruction.cuboid).is_some())
  //       .for_each(|e| {
  //         actual_entries.insert(*e);
  //       }),
  //     CubeState::Off => possible_entries
  //       .iter()
  //       .filter(|p| p.intersection(instruction.cuboid).is_some())
  //       .for_each(|e| {
  //         actual_entries.remove(e);
  //       }),
  //   }
  // }
  // actual_entries.iter().map(|e| e.volume()).sum::<u64>()
}

#[cfg(test)]
mod test {
  use super::*;

  const TEST_INPUT: &'static str = "on x=-20..26,y=-36..17,z=-47..7
  on x=-20..33,y=-21..23,z=-26..28
  on x=-22..28,y=-29..23,z=-38..16
  on x=-46..7,y=-6..46,z=-50..-1
  on x=-49..1,y=-3..46,z=-24..28
  on x=2..47,y=-22..22,z=-23..27
  on x=-27..23,y=-28..26,z=-21..29
  on x=-39..5,y=-6..47,z=-3..44
  on x=-30..21,y=-8..43,z=-13..34
  on x=-22..26,y=-27..20,z=-29..19
  off x=-48..-32,y=26..41,z=-47..-37
  on x=-12..35,y=6..50,z=-50..-2
  off x=-48..-32,y=-32..-16,z=-15..-5
  on x=-18..26,y=-33..15,z=-7..46
  off x=-40..-22,y=-38..-28,z=23..41
  on x=-16..35,y=-41..10,z=-47..6
  off x=-32..-23,y=11..30,z=-14..3
  on x=-49..-5,y=-3..45,z=-29..18
  off x=18..30,y=-20..-8,z=-3..13
  on x=-41..9,y=-7..43,z=-33..15
  on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
  on x=967..23432,y=45373..81175,z=27513..53682";

  const BIGGER_TEST_INPUT: &'static str = "on x=-5..47,y=-31..22,z=-19..33
  on x=-44..5,y=-27..21,z=-14..35
  on x=-49..-1,y=-11..42,z=-10..38
  on x=-20..34,y=-40..6,z=-44..1
  off x=26..39,y=40..50,z=-2..11
  on x=-41..5,y=-41..6,z=-36..8
  off x=-43..-33,y=-45..-28,z=7..25
  on x=-33..15,y=-32..19,z=-34..11
  off x=35..47,y=-46..-34,z=-11..5
  on x=-14..36,y=-6..44,z=-16..29
  on x=-57795..-6158,y=29564..72030,z=20435..90618
  on x=36731..105352,y=-21140..28532,z=16094..90401
  on x=30999..107136,y=-53464..15513,z=8553..71215
  on x=13528..83982,y=-99403..-27377,z=-24141..23996
  on x=-72682..-12347,y=18159..111354,z=7391..80950
  on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
  on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
  on x=-52752..22273,y=-49450..9096,z=54442..119054
  on x=-29982..40483,y=-108474..-28371,z=-24328..38471
  on x=-4958..62750,y=40422..118853,z=-7672..65583
  on x=55694..108686,y=-43367..46958,z=-26781..48729
  on x=-98497..-18186,y=-63569..3412,z=1232..88485
  on x=-726..56291,y=-62629..13224,z=18033..85226
  on x=-110886..-34664,y=-81338..-8658,z=8914..63723
  on x=-55829..24974,y=-16897..54165,z=-121762..-28058
  on x=-65152..-11147,y=22489..91432,z=-58782..1780
  on x=-120100..-32970,y=-46592..27473,z=-11695..61039
  on x=-18631..37533,y=-124565..-50804,z=-35667..28308
  on x=-57817..18248,y=49321..117703,z=5745..55881
  on x=14781..98692,y=-1341..70827,z=15753..70151
  on x=-34419..55919,y=-19626..40991,z=39015..114138
  on x=-60785..11593,y=-56135..2999,z=-95368..-26915
  on x=-32178..58085,y=17647..101866,z=-91405..-8878
  on x=-53655..12091,y=50097..105568,z=-75335..-4862
  on x=-111166..-40997,y=-71714..2688,z=5609..50954
  on x=-16602..70118,y=-98693..-44401,z=5197..76897
  on x=16383..101554,y=4615..83635,z=-44907..18747
  off x=-95822..-15171,y=-19987..48940,z=10804..104439
  on x=-89813..-14614,y=16069..88491,z=-3297..45228
  on x=41075..99376,y=-20427..49978,z=-52012..13762
  on x=-21330..50085,y=-17944..62733,z=-112280..-30197
  on x=-16478..35915,y=36008..118594,z=-7885..47086
  off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
  off x=2032..69770,y=-71013..4824,z=7471..94418
  on x=43670..120875,y=-42068..12382,z=-24787..38892
  off x=37514..111226,y=-45862..25743,z=-16714..54663
  off x=25699..97951,y=-30668..59918,z=-15349..69697
  off x=-44271..17935,y=-9516..60759,z=49131..112598
  on x=-61695..-5813,y=40978..94975,z=8655..80240
  off x=-101086..-9439,y=-7088..67543,z=33935..83858
  off x=18020..114017,y=-48931..32606,z=21474..89843
  off x=-77139..10506,y=-89994..-18797,z=-80..59318
  off x=8476..79288,y=-75520..11602,z=-96624..-24783
  on x=-47488..-1262,y=24338..100707,z=16292..72967
  off x=-84341..13987,y=2429..92914,z=-90671..-1318
  off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
  off x=-27365..46395,y=31009..98017,z=15428..76570
  off x=-70369..-16548,y=22648..78696,z=-1892..86821
  on x=-53470..21291,y=-120233..-33476,z=-44150..38147
  off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";

  #[test]
  fn test_parse() {
    let input = "on x=-29..18,y=-27..17,z=-32..22
    off x=-39..-20,y=-32..-18,z=36..47";
    assert_eq!(
      parse(input).unwrap(),
      (
        "",
        vec![
          Instruction {
            state: CubeState::On,
            cuboid: Cuboid::from_v3s(V3(-29, -27, -32), V3(18, 17, 22)),
          },
          Instruction {
            state: CubeState::Off,
            cuboid: Cuboid::from_v3s(V3(-39, -32, 36), V3(-20, -18, 47)),
          }
        ]
      )
    )
  }

  #[test]
  fn test_p1() {
    let input = TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(p1(parsed), 590784);

    let input = std::fs::read_to_string("./inputs/d22.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(p1(parsed), 580012);
  }

  #[test]
  fn test_p2() {
    let input = BIGGER_TEST_INPUT;
    let parsed = parse(input).unwrap().1;
    assert_eq!(better_p2(parsed), 2758514936282235);

    let input = std::fs::read_to_string("./inputs/d22.txt").unwrap();
    let parsed = parse(&input).unwrap().1;
    assert_eq!(better_p2(parsed), 0);
  }
}
