use std::collections::HashSet;

use ndarray::Array2;

aoc22::main!(day12, "../inputs/input12.txt");

aoc22::test_with_example!(day12, "../inputs/example12.txt", 31, 29);

pub fn day12(input: &str) -> aoc22::MyResult<(u32, u32)> {
    let (elevation_map, start, end) = parse_elevations(input)?;

    let part1 = shortest_path(&elevation_map, start, end).ok_or("Shortest path not found")?;

    let lowest_points = elevation_map
        .indexed_iter()
        .filter(|(_, &elevation)| elevation == 'a')
        .map(|(coord, _)| coord);
    let part2 = lowest_points
        .filter_map(|coord| shortest_path(&elevation_map, coord, end))
        .min()
        .ok_or("Couldn't reach end")?;

    Ok((part1, part2))
}

type Coord = (usize, usize);

fn parse_elevations(input: &str) -> aoc22::MyResult<(Array2<char>, Coord, Coord)> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.len();
    let height = lines[0].len();
    let flat: Vec<char> = lines.into_iter().flat_map(|line| line.chars()).collect();
    let mut elevation_map = Array2::from_shape_vec((width, height), flat)?;
    let start = elevation_map
        .indexed_iter()
        .find(|(_, &elevation)| elevation == 'S')
        .ok_or("No starting point")?
        .0;
    let end = elevation_map
        .indexed_iter()
        .find(|(_, &elevation)| elevation == 'E')
        .ok_or("No end point")?
        .0;

    elevation_map[start] = 'a';
    elevation_map[end] = 'z';

    Ok((elevation_map, start, end))
}

fn shortest_path(elevation_map: &Array2<char>, start: Coord, end: Coord) -> Option<u32> {
    let mut distances: Array2<Option<u32>> = Array2::from_elem(elevation_map.raw_dim(), None);

    let mut to_process: HashSet<Coord> = HashSet::new();
    to_process.insert(start);
    let mut i = 0u32;
    while !to_process.is_empty() {
        to_process.iter().for_each(|&coord| {
            distances[coord] = Some(i);
        });
        i += 1;
        to_process = to_process
            .into_iter()
            .flat_map(steps_from)
            .filter(|&(from, to)| reachable(from, to, elevation_map) && distances[to].is_none())
            .map(|(_, to)| to)
            .collect();
    }
    distances[end]
}

fn reachable(
    from: (usize, usize),
    (to_x, to_y): (usize, usize),
    elevation_map: &Array2<char>,
) -> bool {
    if to_x >= elevation_map.shape()[0] || to_y >= elevation_map.shape()[1] {
        false
    } else {
        elevation_map[(to_x, to_y)] as u32 <= elevation_map[from] as u32 + 1
    }
}

fn steps_from((x, y): Coord) -> [(Coord, Coord); 4] {
    [
        ((x, y), (x + 1, y)),
        ((x, y), (if x > 0 { x - 1 } else { usize::MAX }, y)),
        ((x, y), (x, y + 1)),
        ((x, y), (x, if y > 0 { y - 1 } else { usize::MAX })),
    ]
}
