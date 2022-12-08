use itertools::Itertools;

fn main() -> aoc22::MyResult<()> {
    day1_1()?;

    Ok(())
}

fn day1_1() -> aoc22::MyResult<()> {
    let lines = aoc22::read_lines("inputs/input1.txt")?;
    let res = lines
        .map(|s| {
            if s.is_empty() {
                0
            } else {
                s.parse::<i32>().unwrap()
            }
        })
        .group_by(|i| *i != 0)
        .into_iter()
        .filter_map(
            |(key, group)| {
                if !key {
                    None
                } else {
                    Some(group.sum::<i32>())
                }
            },
        )
        .sorted();

    //for i in res {
    //    println!("{}", i);
    //}
    println!("{}", res.last().unwrap());
    Ok(())
}
