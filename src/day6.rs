use std::collections::HashSet;

fn main() {
    day6_1();
    day6_2();
}

fn find_marker_position(marker_size : usize) -> Option<usize> {
    let s : Vec<char> = include_str!("../inputs/input6.txt").chars().collect();
    for (n, marker) in s.windows(marker_size).enumerate() {
        if marker.iter().map(|c|*c).collect::<HashSet<char>>().len() == marker_size {
            return Some(n+marker_size);
        }
    }
    None
}

fn day6_1() {
    println!("{}", find_marker_position(4).unwrap());
}
fn day6_2() {
    println!("{}", find_marker_position(14).unwrap());
}
