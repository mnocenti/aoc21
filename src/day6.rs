use std::collections::HashSet;

fn main() {
    day6_1();
    day6_2();
}

fn find_marker_position(marker_size: usize) -> Option<usize> {
    let s: Vec<char> = include_str!("../inputs/input6.txt").chars().collect();
    s.windows(marker_size)
        .enumerate()
        .map(|(n, marker)| (n, marker.to_owned().into_iter().collect::<HashSet<char>>()))
        .filter(|(_, marker_set)| marker_set.len() == marker_size)
        .map(|(n,_)| n+marker_size)
        .next()
}

fn day6_1() {
    println!("{}", find_marker_position(4).unwrap());
}
fn day6_2() {
    println!("{}", find_marker_position(14).unwrap());
}
