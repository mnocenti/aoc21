mod utils;

use std::{fmt::Display, str::FromStr};

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
        write!(
            f,
            "move {} from {} to {}",
            self.count,
            self.source + 1,
            self.dest + 1
        )
    }
}

type Stack = Vec<char>; //< crates, top to bottom

#[derive(Debug, Default)]
struct Stacks {
    stacks: Vec<Stack>,
}

impl Stacks {
    /// parse the first part of the input as a Stacks object
    fn parse(lines: Vec<String>) -> utils::MyResult<Stacks> {
        let mut stacks = Stacks::default();
        stacks.stacks.resize(lines[0].len() / 4 + 1, Stack::new());
        for l in lines.into_iter().rev() {
            for stack in 0..stacks.stacks.len() {
                let c = l.chars().nth(4 * stack + 1).ok_or("Bad stacks input")?;
                if !c.is_numeric() && !c.is_whitespace() {
                    stacks.stacks[stack].push(c);
                }
            }
        }
        Ok(stacks)
    }

    /// Get the top of the stacks as a String
    fn top(&self) -> String {
        self.stacks.iter().filter_map(|s| s.last()).collect()
    }

    /// Apply a given instruction to the stacks using CrateMover 9000
    fn apply_9000(&mut self, inst: Instruction) -> utils::MyResult<()> {
        for _ in 0..inst.count {
            let a = self.stacks[inst.source].pop().ok_or(":(")?;
            self.stacks[inst.dest].push(a);
        }
        Ok(())
    }

    /// Apply a given instruction to the stacks using CrateMover 9001
    fn apply_9001(&mut self, inst: Instruction) -> utils::MyResult<()> {
        let src = &mut self.stacks[inst.source];
        let top_crates = src.split_off(src.len() - inst.count);
        self.stacks[inst.dest].extend(top_crates);
        Ok(())
    }
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_len = self.stacks.iter().map(Vec::len).max().unwrap();
        (0..=max_len).rev().try_for_each(|i| {
            self.stacks
                .iter()
                .enumerate()
                .try_for_each(|(stack_index, stack)| {
                    if i == 0 {
                        write!(f, " {}  ", stack_index + 1)
                    } else if i - 1 < stack.len() {
                        write!(f, "[{}] ", stack[i - 1])
                    } else {
                        write!(f, "    ")
                    }
                })?;
            writeln!(f)
        })?;
        Ok(())
    }
}

fn crate_mover(
    apply_instructions: impl Fn(&mut Stacks, Instruction) -> utils::MyResult<()>,
) -> utils::MyResult<()> {
    let mut lines = utils::read_lines("inputs/input5.txt")?;
    let mut stacks = Stacks::parse((&mut lines).take_while(|s| !s.is_empty()).collect())?;
    lines
        .filter_map(|l| Instruction::from_str(&l).ok())
        .try_for_each(|instruction| apply_instructions(&mut stacks, instruction))?;
    println!("{}\n", stacks);
    println!("{}\n", stacks.top());
    Ok(())
}

fn day5_1() -> utils::MyResult<()> {
    crate_mover(Stacks::apply_9000)
}

fn day5_2() -> utils::MyResult<()> {
    crate_mover(Stacks::apply_9001)
}
