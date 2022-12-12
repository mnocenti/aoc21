use std::{collections::VecDeque, fmt::Debug, str::FromStr};

use itertools::Itertools;

aoc22::main!(day11, "../inputs/input11.txt");

aoc22::test_with_example!(day11, "../inputs/example11.txt", 10605, 0);

pub fn day11(input: &str) -> aoc22::MyResult<(u64, i32)> {
    let mut monkeys = input
        .split("\r\n\r\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    for round in 1..=20 {
        for monkey_index in 0..monkeys.len() {
            while let Some((item, target)) = monkeys[monkey_index].inspect() {
                monkeys[target].catch(item);
            }
        }
        round_recap(round, &monkeys);
    }

    let monkey_business = monkeys
        .iter()
        .sorted_by(|monmon, keykey| Ord::cmp(&monmon.inspections, &keykey.inspections))
        .rev()
        .tuple_windows()
        .map(|(first, second)| first.inspections * second.inspections)
        .next()
        .ok_or("No monke ?")?;

    let part2 = 0;

    Ok((monkey_business, part2))
}

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divider: u64,
    target: (usize, usize),
    inspections: u64,
}

impl Monkey {
    fn inspect(&mut self) -> Option<(u64, usize)> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;
        item = (self.operation)(item);
        item /= 3;
        if item % self.divider == 0 {
            Some((item, self.target.0))
        } else {
            Some((item, self.target.1))
        }
    }

    fn catch(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            //.field("operation", &self.operation)
            .field("divider", &self.divider)
            .field("target", &self.target)
            .finish()
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn std::error::Error>;

    fn from_str(description: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = description.trim().lines().collect();

        let items = match lines[1].trim().split(&[' ', ',']).collect::<Vec<_>>()[..] {
            ["Starting", "items:", ref items_str @ ..] => items_str
                .iter()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse())
                .collect::<Result<VecDeque<_>, _>>()?,
            ref bad => panic!("Can't parse items : {:?}", bad),
        };
        let operation = match lines[2].trim().split(&[' ', ',']).collect::<Vec<_>>()[..] {
            ["Operation:", "new", "=", a, op, b] => parse_operation(a, op, b)?,
            ref bad => panic!("Can't parse operation : {:?}", bad),
        };
        let divider = match lines[3].trim().split(&[' ', ',']).collect::<Vec<_>>()[..] {
            ["Test:", "divisible", "by", n] => n.parse()?,
            ref bad => panic!("Can't parse operation : {:?}", bad),
        };
        let monkey_true: usize = lines[4]
            .trim()
            .split(&[' ', ','])
            .last()
            .ok_or("If true parsing error")?
            .parse()?;
        let monkey_false: usize = lines[5]
            .split(&[' ', ','])
            .last()
            .ok_or("If false parsing error")?
            .parse()?;

        Ok(Monkey {
            items,
            operation,
            divider,
            target: (monkey_true, monkey_false),
            inspections: 0,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(u64),
    Old,
}

impl Operand {
    fn get(self, old: u64) -> u64 {
        match self {
            Self::Literal(n) => n,
            Self::Old => old,
        }
    }
}

impl FromStr for Operand {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Operand::Old),
            n => Ok(Operand::Literal(n.parse()?)),
        }
    }
}

fn parse_operation(a: &str, op: &str, b: &str) -> aoc22::MyResult<Box<dyn Fn(u64) -> u64>> {
    let a: Operand = a.parse()?;
    let b: Operand = b.parse()?;
    match op {
        "+" => Ok(Box::new(move |old| a.get(old) + b.get(old))),
        "*" => Ok(Box::new(move |old| a.get(old) * b.get(old))),
        _ => panic!("Unknown operation"),
    }
}

fn round_recap(round: usize, monkeys: &[Monkey]) {
    println!("After round {}:", round);
    for (index, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", index, monkey.items);
    }
    println!();
}
