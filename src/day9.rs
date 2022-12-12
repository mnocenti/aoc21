use std::collections::HashSet;

use std::ops::{Add, Sub};

aoc22::main!(day9, "../inputs/input9.txt");

aoc22::test_with_example!(
    part1,
    "../inputs/example9_1.txt",
    13,
    part2,
    "../inputs/example9_2.txt",
    36
);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord(i32, i32);

impl Add for Coord {
    type Output = Self;

    fn add(self, Self(dx, dy): Self) -> Self {
        let Coord(x, y) = self;
        Self(x + dx, y + dy)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, Self(dx, dy): Self) -> Self {
        let Coord(x, y) = self;
        Self(x - dx, y - dy)
    }
}

struct Step {
    offset: Coord,
    count: u32,
}

pub fn day9(input: &str) -> aoc22::MyResult<(usize, usize)> {
    Ok((part1(input)?, part2(input)?))
}

pub fn part1(input: &str) -> aoc22::MyResult<usize> {
    simulate_rope::<2>(input)
}

pub fn part2(input: &str) -> aoc22::MyResult<usize> {
    simulate_rope::<10>(input)
}

fn simulate_rope<const N: usize>(input: &str) -> aoc22::MyResult<usize> {
    let steps = parse_steps(input);

    let start = Coord(0, 0);
    let mut rope = vec![start; N];

    let mut visited = HashSet::new();
    visited.insert(start);

    for step in steps {
        for _ in 0..step.count {
            rope[0] = rope[0] + step.offset;
            for i in 1..N {
                rope[i] = rope_physics_step(rope[i - 1], rope[i]);
            }
            visited.insert(rope[N - 1]);
        }
    }

    Ok(visited.len())
}

fn parse_steps(input: &str) -> Vec<Step> {
    let steps: Vec<Step> = input
        .lines()
        .filter_map(|l| {
            let (direction, count) = l.split_once(' ')?;
            let count = count.parse().unwrap();
            let offset = match direction {
                "U" => Coord(0, 1),
                "D" => Coord(0, -1),
                "L" => Coord(-1, 0),
                "R" => Coord(1, 0),
                _ => panic!("Bad input parsing"),
            };
            Some(Step { offset, count })
        })
        .collect();
    steps
}

fn towards(pos: i32) -> i32 {
    match pos {
        x if x > 0 => 1,
        x if x < 0 => -1,
        _ => 0,
    }
}

// compute the new position of the tail based on the current position of head and tail
fn rope_physics_step(head: Coord, tail: Coord) -> Coord {
    match head - tail {
        Coord(2, y) => tail + Coord(1, towards(y)),
        Coord(-2, y) => tail + Coord(-1, towards(y)),
        Coord(x, 2) => tail + Coord(towards(x), 1),
        Coord(x, -2) => tail + Coord(towards(x), -1),
        _ => tail,
    }
}
