mod utils;

use std::{collections::VecDeque, fmt::Display, str::FromStr};

use itertools::Itertools;

fn main() -> utils::MyResult<()> {
    day5_1()?;
    day5_2()?;

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let words = str.split(' ').collect_vec();
        Ok(Instruction {
            count: words[1].parse()?,
            source: words[3].parse::<usize>()? - 1usize,
            dest: words[5].parse::<usize>()? - 1usize,
        })
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "move {} from {} to {}", self.count, self.source + 1, self.dest + 1)
    }
}

type Stack = VecDeque<char>; //< crates, top to bottom

#[derive(Debug, Default)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    fn parse(lines: impl Iterator<Item = String>) -> utils::MyResult<Stacks> {
        let mut lines = lines.peekable();
        let mut stacks = Stacks::default();
        stacks.stacks.resize(lines.peek().ok_or_else(||"Bad stacks input")?.len() / 4 + 1, Stack::new());
        for l in lines {
            for stack in 0..stacks.stacks.len() {
                let c = l.chars().nth(4*stack+1).ok_or_else(||"Bad stacks input")?;
                if !c.is_numeric() && !c.is_whitespace() {
                    stacks.stacks[stack].push_back(c);
                }
            }
        }
        Ok(stacks)
    }

    /// Get the top of the stacks as a String
    fn top(&self) -> String {
        (&self.stacks).into_iter().filter_map(Stack::front).collect()
    }

    /// Apply a given instruction to the stacks using CrateMover 9000
    fn apply_9000(&mut self, instruction: Instruction) -> utils::MyResult<()> {
        for _ in 0..instruction.count {
            let a = self.stacks[instruction.source].pop_front().ok_or(":(")?;
            self.stacks[instruction.dest].push_front(a);
        }
        Ok(())
    }

    /// Apply a given instruction to the stacks using CrateMover 9001
    fn apply_9001(&mut self, instruction: Instruction) -> utils::MyResult<()> {
        let mut new_dest: VecDeque<char> = self.stacks[instruction.source]
            .drain(0..instruction.count)
            .collect();
        new_dest.append(&mut self.stacks[instruction.dest]);
        self.stacks[instruction.dest] = new_dest;
        Ok(())
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.stacks {
            writeln!(f, "{}", s.into_iter().collect::<String>())?;
        }
        Ok(())
    }
}

fn crate_mover(
    apply_instructions: impl Fn(&mut Stacks, Instruction) -> utils::MyResult<()>
) -> utils::MyResult<()> {
    let mut lines = utils::read_lines("inputs/input5.txt")?;

    let mut stacks = Stacks::parse((&mut lines).take_while(|s| !s.is_empty()))?;

    let instructions = lines.filter_map(|l| Instruction::from_str(&l).ok());

    for instruction in instructions {
        apply_instructions(&mut stacks, instruction)?;
    }

    println!("{}\n", stacks.top());

    Ok(())
}

fn day5_1() -> utils::MyResult<()> {
    crate_mover(Stacks::apply_9000)
}

fn day5_2() -> utils::MyResult<()> {
    crate_mover(Stacks::apply_9001)
}
