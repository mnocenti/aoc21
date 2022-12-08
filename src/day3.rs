use std::collections::HashSet;

use itertools::Itertools;

fn main() -> aoc22::MyResult<()> {
    day3_1()?;
    day3_2()?;

    Ok(())
}

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

fn day3_1() -> aoc22::MyResult<()> {
    let lines = aoc22::read_lines("inputs/input3.txt")?;
    let res: u32 = lines
        .map(|s| -> (HashSet<char>, HashSet<char>) {
            let hs = s.len() / 2;
            (s[..hs].chars().collect(), s[hs..].chars().collect())
        })
        .map(|(left, right)| -> char { *left.intersection(&right).next().unwrap() })
        .map(item_priority)
        .sum();

    println!("{}", res);

    Ok(())
}

fn day3_2() -> aoc22::MyResult<()> {
    let lines = aoc22::read_lines("inputs/input3.txt")?;
    let res: u32 = lines
        .map(|s| s.chars().collect::<HashSet<char>>())
        .chunks(3)
        .into_iter()
        .map(|group| -> char {
            let common_chars =
                group.reduce(|accum, item| accum.intersection(&item).copied().collect());
            common_chars.unwrap().into_iter().next().unwrap()
        })
        .map(item_priority)
        .sum();

    println!("{}", res);

    Ok(())
}
