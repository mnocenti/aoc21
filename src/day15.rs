use std::{collections::HashSet, ops::RangeInclusive};

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
    let beacons: HashSet<_> = sensors.iter().map(|s| s.closest_beacon).collect();

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.position.0 - sensor.usable_range)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|sensor| sensor.position.0 + sensor.usable_range)
        .max()
        .unwrap();

    let row = area_size / 2;
    let covered_positions = (min_x..=max_x)
        .filter(|&x| sensors.iter().any(|sensor| sensor.covers((x, row))))
        .count();
    let beacons_in_row = beacons.iter().filter(|b| b.1 == row).count();
    let part1 = covered_positions - beacons_in_row;

    let hidden_beacon_pos =
        find_uncovered_tile(area_size, &sensors).ok_or("Couldn't find beacon")?;

    let part2 = hidden_beacon_pos.0 * 4000000 + hidden_beacon_pos.1;

    Ok((part1, part2))
}

type Coord = (isize, isize);

fn manhattan_distance((ax, ay): Coord, (bx, by): Coord) -> isize {
    (ax.abs_diff(bx) + ay.abs_diff(by)) as isize
}

#[derive(Debug)]
struct Sensor {
    position: Coord,
    closest_beacon: Coord,
    usable_range: isize,
}

impl Sensor {
    fn new(position: Coord, closest_beacon: Coord) -> Sensor {
        Sensor {
            position,
            closest_beacon,
            usable_range: manhattan_distance(position, closest_beacon),
        }
    }

    fn covers(&self, point: Coord) -> bool {
        manhattan_distance(self.position, point) <= self.usable_range
    }

    fn range_for_row(&self, y: isize) -> Option<RangeInclusive<isize>> {
        let y_diff = self.position.1.abs_diff(y) as isize;
        if y_diff > self.usable_range {
            None
        } else {
            let remaining_distance = self.usable_range - y_diff;
            Some((self.position.0 - remaining_distance)..=(self.position.0 + remaining_distance))
        }
    }
}

fn parse_sensors(input: &str) -> aoc22::MyResult<Vec<Sensor>> {
    input
        .lines()
        .flat_map(|l| l.split(&[':', ',']))
        .tuples()
        .map(|(sx, sy, bx, by)| {
            let get_val = |s: &str| {
                let val_str = s.split_once('=').unwrap().1;
                val_str.parse::<isize>()
            };
            Ok(Sensor::new(
                (get_val(sx)?, get_val(sy)?),
                (get_val(bx)?, get_val(by)?),
            ))
        })
        .collect()
}

fn find_uncovered_tile(area_size: isize, sensors: &Vec<Sensor>) -> Option<Coord> {
    for y in 0..=area_size {
        let not_covered = remove_covered_ranges(area_size, y, sensors);
        if let Some(r) = not_covered.first() {
            return Some((*r.start(), y));
        }
        if y % 100000 == 0 {
            println!("{}", y);
        }
    }
    None
}

fn remove_covered_ranges(
    area_size: isize,
    y: isize,
    sensors: &Vec<Sensor>,
) -> Vec<RangeInclusive<isize>> {
    let mut row = vec![(0..=area_size)];
    for s in sensors {
        if let Some(range) = s.range_for_row(y) {
            remove_range(&mut row, range);
        }
    }
    row
}

fn remove_range(row: &mut Vec<RangeInclusive<isize>>, range: RangeInclusive<isize>) {
    if range.is_empty() {
        return;
    }
    let start = *range.start();
    let end = *range.end();
    let mut i = 0usize;
    while i < row.len() {
        let r = &mut row[i];
        let r_start = *r.start();
        let r_end = *r.end();
        if r_start >= start && r_end <= end {
            row.remove(i);
            continue; // don't increase i
        } else if start <= r_end {
            if start > r_start && end > r_end {
                *r = r_start..=(start - 1);
            } else {
                if start > r_start {
                    *r = r_start..=(start - 1);
                    if end < r_end {
                        row.insert(i + 1, (end + 1)..=r_end);
                    }
                } else if end >= r_start {
                    *r = (end + 1)..=r_end;
                }
                break;
            }
        }
        i += 1;
    }
}
