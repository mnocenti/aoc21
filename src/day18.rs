use std::{collections::HashMap, ops::Add};

use itertools::Itertools;

aoc22::main!(day18, "../inputs/input18.txt");

aoc22::test_with_example!(day18, "../inputs/example18.txt", 64, 58);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
    z: isize,
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<(isize, isize, isize)> for Coord {
    fn from((x, y, z): (isize, isize, isize)) -> Self {
        Coord { x, y, z }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Matter {
    Lava,
    ExteriorAir,
}

pub fn day18(input: &str) -> aoc22::MyResult<(usize, usize)> {
    let mut cubes: HashMap<_, _> = input
        .lines()
        .filter_map(|l| match l.split(',').collect_vec()[..] {
            [x, y, z] => Some(Coord::from((
                x.parse().unwrap(),
                y.parse().unwrap(),
                z.parse().unwrap(),
            ))),
            _ => None,
        })
        .map(|pos| (pos, Matter::Lava))
        .collect();

    let part1 = cubes
        .keys()
        .map(|pos| 6 - count_neighboring_lava(pos, &cubes))
        .sum();

    simulate_exterior_air(&mut cubes);

    let part2 = cubes
        .iter()
        .filter_map(|(pos, matter)| (*matter == Matter::Lava).then_some(pos))
        .map(|pos| count_neighboring_air(pos, &cubes))
        .sum();

    Ok((part1, part2))
}

fn count_neighboring_lava(cube: &Coord, cubes: &HashMap<Coord, Matter>) -> usize {
    cubes
        .keys()
        .filter(|c| c.x.abs_diff(cube.x) + c.y.abs_diff(cube.y) + c.z.abs_diff(cube.z) == 1)
        .count()
}

fn count_neighboring_air(cube: &Coord, cubes: &HashMap<Coord, Matter>) -> usize {
    cubes
        .iter()
        .filter_map(|(pos, matter)| (*matter == Matter::ExteriorAir).then_some(pos))
        .filter(|c| c.x.abs_diff(cube.x) + c.y.abs_diff(cube.y) + c.z.abs_diff(cube.z) == 1)
        .count()
}

fn minmax(cubes: &HashMap<Coord, Matter>, f: impl Fn(&Coord) -> isize) -> (isize, isize) {
    cubes.keys().map(f).minmax().into_option().unwrap()
}

fn simulate_exterior_air(cubes: &mut HashMap<Coord, Matter>) {
    let (min_x, max_x) = minmax(cubes, |pos| pos.x);
    let (min_y, max_y) = minmax(cubes, |pos| pos.y);
    let (min_z, max_z) = minmax(cubes, |pos| pos.z);
    // add layers of air below the lava
    (min_x - 1..=max_x + 1)
        .cartesian_product((min_z - 1)..max_z + 1)
        .for_each(|(x, z)| {
            cubes.insert(Coord::from((x, min_y - 1, z)), Matter::ExteriorAir);
            cubes.insert(Coord::from((x, max_y + 1, z)), Matter::ExteriorAir);
        });

    let in_bounds = |pos: &Coord| {
        pos.x >= min_x - 1
            && pos.x <= max_x + 1
            && pos.y >= min_y - 1
            && pos.y <= max_y + 1
            && pos.z >= min_z - 1
            && pos.z <= max_z + 1
    };
    let sides = [
        Coord::from((1, 0, 0)),
        Coord::from((0, 1, 0)),
        Coord::from((0, 0, 1)),
        Coord::from((-1, 0, 0)),
        Coord::from((0, -1, 0)),
        Coord::from((0, 0, -1)),
    ];
    // let the air propagate
    // NB: the number of iterations is a bit arbitrary here,
    // if the air takes a really long path inside the lava we won't detect it.
    let iterations = max_y - min_y;
    for i in 0..iterations {
        println!("{}/{}", i, iterations);
        let new_air_cubes = cubes
            .iter()
            .filter(|(_, matter)| **matter == Matter::ExteriorAir)
            .flat_map(|(pos, _)| {
                sides.iter().filter_map(|side| {
                    let neighbor = *pos + *side;
                    if in_bounds(&neighbor) && !cubes.contains_key(&neighbor) {
                        Some((*pos + *side, Matter::ExteriorAir))
                    } else {
                        None
                    }
                })
            });
        cubes.extend(new_air_cubes.collect::<HashMap<_, _>>());
    }
}
