use std::{collections::VecDeque, fmt::Debug, rc::Rc, str::FromStr};

use itertools::Itertools;

aoc22::main!(day11, "../inputs/input11.txt");

aoc22::test_with_example!(day11, "../inputs/example11.txt", 10605, 2713310158);

pub fn day11(input: &str) -> aoc22::MyResult<(u64, u64)> {
    let monkeys = input
        .split("\r\n\r\n")
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let part1 = monkey_business(monkeys.clone(), 20, &|worry| worry / 3)?;

    let ppcm = monkeys.iter().map(|m| m.prime_factor).product();
    let part2 = monkey_business(monkeys, 10000, &|worry| {
        decrease_worry_with_ppcm(worry, ppcm)
    })?;

    Ok((part1, part2))
}

/// use prime numbers to fight anxiety
fn decrease_worry_with_ppcm(worry: u64, ppcm: u64) -> u64 {
    if worry > ppcm {
        worry % ppcm + ppcm
    } else {
        worry
    }
}

fn monkey_business(
    mut monkeys: Vec<Monkey>,
    round_count: usize,
    decrease_worry: &impl Fn(u64) -> u64,
) -> aoc22::MyResult<u64> {
    for round in 1..=round_count {
        for monkey_index in 0..monkeys.len() {
            while let Some((item, target)) = monkeys[monkey_index].inspect(decrease_worry) {
                monkeys[target].catch(item);
            }
        }
        round_recap(round, &monkeys);
    }

    let business = monkeys
        .iter()
        .sorted_by(|monmon, keykey| Ord::cmp(&monmon.inspections, &keykey.inspections))
        .rev()
        .tuple_windows()
        .map(|(first, second)| first.inspections * second.inspections)
        .next()
        .ok_or("No monke ?")?;

    Ok(business)
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Rc<dyn Fn(u64) -> u64>,
    prime_factor: u64,
    target: (usize, usize),
    inspections: u64,
}

impl Monkey {
    fn inspect(&mut self, decrease_worry: impl Fn(u64) -> u64) -> Option<(u64, usize)> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;
        item = (self.operation)(item);
        item = decrease_worry(item);
        if item % self.prime_factor == 0 {
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
            .field("prime_factor", &self.prime_factor)
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
        let prime_factor = match lines[3].trim().split(&[' ', ',']).collect::<Vec<_>>()[..] {
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
            prime_factor,
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

fn parse_operation(a: &str, op: &str, b: &str) -> aoc22::MyResult<Rc<dyn Fn(u64) -> u64>> {
    let a: Operand = a.parse()?;
    let b: Operand = b.parse()?;
    match op {
        "+" => Ok(Rc::new(move |old| a.get(old) + b.get(old))),
        "*" => Ok(Rc::new(move |old| a.get(old) * b.get(old))),
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
