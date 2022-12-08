use std::str::FromStr;

fn main() -> aoc22::MyResult<()> {
    day2_1()?;
    day2_2()?;

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for RPS {
    type Err = ();

    fn from_str(input: &str) -> Result<RPS, Self::Err> {
        match input {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissor),
            _ => Err(()),
        }
    }
}

impl RPS {
    fn from_int(val: i32) -> Option<RPS> {
        match val {
            1 => Some(RPS::Rock),
            2 => Some(RPS::Paper),
            3 => Some(RPS::Scissor),
            4 => Some(RPS::Rock),
            0 => Some(RPS::Scissor),
            _ => None,
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(input: &str) -> Result<Outcome, Self::Err> {
        match input {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

fn score_v1(opponent_play: RPS, instructions: &str) -> i32 {
    let my_play = RPS::from_str(instructions).unwrap();

    let outcome = match (opponent_play, my_play) {
        (a, b) if a == b => Outcome::Draw,
        (RPS::Scissor, RPS::Rock) => Outcome::Win,
        (RPS::Rock, RPS::Scissor) => Outcome::Lose,
        (a, b) if b as i32 > a as i32 => Outcome::Win,
        _ => Outcome::Lose,
    };
    my_play as i32 + outcome as i32
}

fn score_v2(opponent_play: RPS, instructions: &str) -> i32 {
    let outcome = Outcome::from_str(instructions).unwrap();
    let my_play = match (opponent_play, outcome) {
        (o, Outcome::Draw) => Some(o),
        (o, Outcome::Win) => RPS::from_int(o as i32 + 1),
        (o, Outcome::Lose) => RPS::from_int(o as i32 - 1),
    }
    .unwrap();

    my_play as i32 + outcome as i32
}

fn compute_score(score_fn: impl Fn(RPS, &str) -> i32) -> aoc22::MyResult<i32> {
    let lines = aoc22::read_lines("inputs/input2.txt")?;
    let res = lines
        .map(|s| {
            let parts: Vec<_> = s.split(' ').collect();
            let opponent_play = RPS::from_str(parts[0]).unwrap();
            score_fn(opponent_play, parts[1])
        })
        .sum::<i32>();
    Ok(res)
}

fn day2_1() -> aoc22::MyResult<()> {
    let score = compute_score(score_v1)?;
    println!("{}", score);

    Ok(())
}

fn day2_2() -> aoc22::MyResult<()> {
    let score = compute_score(score_v2)?;
    println!("{}", score);

    Ok(())
}
