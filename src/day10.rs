use itertools::Itertools;

aoc22::main!(day10, "../inputs/input10.txt");

const EXAMPLE2_EXPECTED: &str = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
aoc22::test_with_example!(day10, "../inputs/example10.txt", 13140, EXAMPLE2_EXPECTED);

#[derive(Debug, Default, Copy, Clone)]
enum Instruction {
    #[default]
    NoOp,
    AddX(i32),
}

impl Instruction {
    fn apply(&self, x: i32) -> i32 {
        match self {
            Self::NoOp => x,
            Self::AddX(v) => x + v,
        }
    }
    fn cycle_count(&self) -> i32 {
        match self {
            Self::NoOp => 1,
            Self::AddX(_) => 2,
        }
    }
}

pub fn day10(input: &str) -> aoc22::MyResult<(i32, String)> {
    let instructions = parse_instructions(input);

    let mut states: Vec<i32> = Vec::new();
    let mut x = 1;
    states.push(x); // cycle 0
    for instr in instructions {
        for _ in 0..instr.cycle_count() {
            states.push(x)
        }
        x = instr.apply(x);
    }
    for (cycle, x) in states.iter().enumerate() {
        println!("{}: {}", cycle, x);
    }

    let part1 = states
        .iter()
        .enumerate()
        .filter_map(|(cycle, x)| {
            if is_interesting(cycle) {
                println!("{}th cycle, x={}", cycle, x);
                Some((cycle as i32) * x)
            } else {
                None
            }
        })
        .sum();

    let part2 = render_screen(&states);

    Ok((part1, part2))
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| match l.split_once(' ') {
            Some(("addx", val)) => Instruction::AddX(val.parse().unwrap()),
            _ => Instruction::NoOp,
        })
        .collect()
}

const INTERESTING_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn is_interesting(cycle: usize) -> bool {
    INTERESTING_CYCLES.contains(&cycle)
}

fn render_screen(states: &[i32]) -> String {
    states
        .iter()
        .enumerate()
        .skip(1)
        .map(|(cycle, x)| {
            let pos = ((cycle - 1) % 40) as i32;
            if (pos - x).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .chunks(40)
        .into_iter()
        .map(|a| a.collect::<String>() + "\n")
        .fold(String::new(), |acc, elem| acc + &elem)
}
