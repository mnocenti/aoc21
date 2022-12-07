mod utils;

use std::ops::RangeInclusive;

fn main() -> utils::MyResult<()> {
    day4_1()?;
    day4_2()?;

    Ok(())
}

fn get_pairs_of_sections(
) -> utils::MyResult<impl Iterator<Item = (RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let lines = utils::read_lines("inputs/input4.txt")?;
    let res = lines.filter_map(|s: String| {
        let (first, second) = s.split_once(',')?;
        let ((f1, f2), (s1, s2)) = (first.split_once('-')?, second.split_once('-')?);
        match (
            f1.parse::<u32>(),
            f2.parse::<u32>(),
            s1.parse::<u32>(),
            s2.parse::<u32>(),
        ) {
            (Ok(f1), Ok(f2), Ok(s1), Ok(s2)) => Some((f1..=f2, s1..=s2)),
            _ => None,
        }
    });
    Ok(res)
}

fn day4_1() -> utils::MyResult<()> {
    let pairs_of_sections = get_pairs_of_sections()?;
    let res = pairs_of_sections
        .filter_map(|(range1, range2)| {
            let fully_contains = |r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>| {
                r1.start() <= r2.start() && r1.end() >= r2.end()
            };
            if fully_contains(&range1, &range2) || fully_contains(&range2, &range1) {
                Some(1)
            } else {
                None
            }
        })
        .count();
    println!("{}", res);

    Ok(())
}

fn day4_2() -> utils::MyResult<()> {
    let pairs_of_sections = get_pairs_of_sections()?;
    let res = pairs_of_sections
        .filter_map(|(range1, range2)| {
            let overlap = |r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>| {
                r1.start() <= r2.start() && r1.end() >= r2.start()
            };
            if overlap(&range1, &range2) || overlap(&range2, &range1) {
                Some(1)
            } else {
                None
            }
        })
        .count();
    println!("{}", res);

    Ok(())
}
