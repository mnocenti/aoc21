use std::collections::HashSet;

use itertools::Itertools;


aoc22::main!(day15_input, "../inputs/input15.txt");

aoc22::test_with_example!(day15_example, "../inputs/example15.txt", 26, 56000011);

pub fn day15_input(input: &str) -> aoc22::MyResult<(usize, isize)> {
    day15(input, 4000000)
}
pub fn day15_example(input: &str) -> aoc22::MyResult<(usize, isize)> {
    day15(input, 20)
}

pub fn day15(input: &str, area_size: isize) -> aoc22::MyResult<(usize, isize)> {
    let sensors = parse_sensors(input)?;
    let beacons : HashSet<_> = sensors.iter().map(|s|s.closest_beacon).collect();

    let min_x = sensors.iter().map(|sensor| sensor.position.0-sensor.usable_range).min().unwrap();
    let max_x = sensors.iter().map(|sensor| sensor.position.0+sensor.usable_range).max().unwrap();

    let row = area_size/2;
    let covered_positions= (min_x..=max_x).filter(|&x| sensors.iter().any(|sensor|sensor.covers((x, row)))).count();
    let beacons_in_row = beacons.iter().filter(|b| b.1 == row).count();
    let part1 = covered_positions - beacons_in_row;

    // this might give a result in a few years... 😴
    let hidden_beacon_pos = (0..=area_size).cartesian_product(0..=area_size).find(|(x,y)| !sensors.iter().any(|sensor|sensor.covers((*x, *y)))).unwrap();

    let part2 = hidden_beacon_pos.0*4000000+hidden_beacon_pos.1;

    // TODO change space coordinates to use (u,v) where u = x+y and v = x-y

    Ok((part1, part2))
}

type Coord = (isize,isize);

fn manhattan_distance((ax,ay) : Coord, (bx,by) : Coord) -> isize {
    (ax.abs_diff(bx) + ay.abs_diff(by)) as isize
}

#[derive(Debug)]
struct Sensor {
    position : Coord,
    closest_beacon : Coord,
    usable_range: isize,
}

impl Sensor {
    fn new (position : Coord, closest_beacon : Coord) -> Sensor {
        Sensor { position, closest_beacon, usable_range: manhattan_distance(position,closest_beacon)}
    }

    fn covers(&self, point : Coord) -> bool {
        manhattan_distance(self.position, point) <= self.usable_range
    }
}

fn parse_sensors(input: &str) -> aoc22::MyResult<Vec<Sensor>> {
    input.lines().flat_map(|l| l.split(&[':',','])).tuples().map(|(sx,sy,bx,by)| {
        let get_val = |s:&str| { let val_str = s.split_once('=').unwrap().1;
        val_str.parse::<isize>()};
        Ok(Sensor::new((get_val(sx)?, get_val(sy)?),(get_val(bx)?, get_val(by)?)))
    }).collect()
}