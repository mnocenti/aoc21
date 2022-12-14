use std::fmt::Display;

use itertools::Itertools;

aoc22::main!(day14, "../inputs/input14.txt");

aoc22::test_with_example!(day14, "../inputs/example14.txt", 24, 93);

pub fn day14(input: &str) -> aoc22::MyResult<(usize, usize)> {
    let (cave, sand_entry) = parse_cave(input)?;

    let mut part1_cave = cave.clone();
    display_cave(&part1_cave);

    let part1 = (1..)
        .take_while(|_| add_sand(&mut part1_cave, sand_entry))
        .last()
        .unwrap_or(0);
    display_cave(&part1_cave);

    let mut part2_cave = cave;
    add_bedrock(&mut part2_cave);
    display_cave(&part2_cave);

    let part2 = (1..)
        .take_while(|_| add_sand(&mut part2_cave, sand_entry))
        .last()
        .unwrap_or(0);

    display_cave(&part2_cave);

    Ok((part1, part2))
}

type Coord = (usize, usize);
type Path = Vec<Coord>;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
        }
        .fmt(f)
    }
}

type Cave = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq)]
enum SandMovement {
    Move(Coord),
    Rest,
    EndlessAbyss,
    SourceBlocked,
}

fn add_sand(cave: &mut Cave, source_pos: Coord) -> bool {
    let mut sand_pos = source_pos;
    let mut result = sand_physics(cave, sand_pos, source_pos);
    while let SandMovement::Move(new_pos) = result {
        sand_pos = new_pos;
        result = sand_physics(cave, sand_pos, source_pos);
    }
    if let SandMovement::Rest = result {
        cave[sand_pos.0][sand_pos.1] = Tile::Sand;
    }

    result != SandMovement::EndlessAbyss && result != SandMovement::SourceBlocked
}

fn sand_physics(cave: &Cave, (x, y): Coord, source_pos: Coord) -> SandMovement {
    if cave[source_pos.0][source_pos.1] != Tile::Air {
        SandMovement::SourceBlocked
    } else if y == cave[0].len() - 1 {
        SandMovement::EndlessAbyss
    } else if cave[x][y + 1] == Tile::Air {
        SandMovement::Move((x, y + 1))
    } else if x > 0 && cave[x - 1][y + 1] == Tile::Air {
        SandMovement::Move((x - 1, y + 1))
    } else if x < cave.len() - 1 && cave[x + 1][y + 1] == Tile::Air {
        SandMovement::Move((x + 1, y + 1))
    } else {
        SandMovement::Rest
    }
}

fn display_cave(cave: &Cave) {
    for y in 0..cave[0].len() {
        for col in cave {
            print!("{}", col[y]);
        }
        println!();
    }
    println!();
}

fn parse_cave(input: &str) -> aoc22::MyResult<(Cave, Coord)> {
    let paths = input
        .lines()
        .map(parse_path)
        .collect::<Result<Vec<_>, _>>()?;
    let (min_x, max_x) = paths
        .iter()
        .flatten()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .ok_or("No paths")?;
    let max_y = paths
        .iter()
        .flatten()
        .map(|(_, y)| y)
        .max()
        .ok_or("No paths")?;

    // height : account for the added rock at the bottom in part 2
    let max_height = max_y + 3;
    // width : add 2 for the side holes needed to guarantee a path to the abyss
    let max_starting_width = (max_x - min_x) + 2;
    // width that is big enough for any possible pile of sand
    let max_final_width = max_height * 2 + max_starting_width;
    // shift every x coord to put the sand source in the middle
    let x_shift = (max_final_width / 2) as isize - 500;

    let mut cave = vec![vec![Tile::Air; max_height]; max_final_width];

    // add rocks from paths
    let mut add_rock = |x: usize, y: usize| cave[(x as isize + x_shift) as usize][y] = Tile::Rock;
    for p in paths {
        for (&(start_x, start_y), &(end_x, end_y)) in p.iter().tuple_windows() {
            if start_x == end_x {
                (start_y.min(end_y)..=start_y.max(end_y)).for_each(|y| add_rock(start_x, y));
            } else {
                (start_x.min(end_x)..=start_x.max(end_x)).for_each(|x| add_rock(x, start_y));
            }
        }
    }
    Ok((cave, ((500 + x_shift) as usize, 0)))
}

fn add_bedrock(cave: &mut Cave) {
    let max_y = cave[0].len() - 1;
    for col in cave {
        col[max_y] = Tile::Rock;
    }
}

fn parse_path(line: &str) -> aoc22::MyResult<Path> {
    line.split(" -> ")
        .map(|points| {
            let (x, y) = points.split_once(',').ok_or("parse error")?;
            Ok((x.parse::<usize>()?, y.parse::<usize>()?))
        })
        .collect()
}
