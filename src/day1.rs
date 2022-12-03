use itertools::Itertools;

mod utils;

fn main() -> utils::MyResult<()> {
    day1_1()?;

    Ok(())
}

fn day1_1() -> utils::MyResult<()> {
    let lines = utils::read_lines("inputs/input1.txt")?;
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
        .filter_map(|(key, group)| {
            if !key {
                return None;
            } else {
                Some(group.sum::<i32>())
            }
        })
        .sorted();

    //for i in res {
    //    println!("{}", i);
    //}
    println!("{}", res.last().unwrap());
    Ok(())
}
