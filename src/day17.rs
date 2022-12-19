use std::{fmt::Display, ops::Add};

aoc22::main!(day17, "../inputs/input17.txt");

aoc22::test_with_example!(day17, "../inputs/example17.txt", 3068, 1514285714288);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

macro_rules! c {
    ($x: expr, $y: expr) => {
        Coord { x: $x, y: $y }
    };
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Wind {
    Left,
    Right,
}

impl Wind {
    fn offset(&self) -> Coord {
        match *self {
            Self::Left => c!(-1, 0),
            Self::Right => c!(1, 0),
        }
    }
}
type Shape = Vec<Coord>;
type Row = [bool; TOWER_WIDTH as usize];
struct Tower {
    rows: Vec<Row>,
}

impl Tower {
    fn with_height_capacity(capacity: usize) -> Tower {
        Tower {
            rows: Vec::with_capacity(capacity),
        }
    }

    fn is_rock(&self, c: &Coord) -> bool {
        (c.y as usize) < self.rows.len() && self.rows[c.y as usize][c.x as usize]
    }

    fn height(&self) -> isize {
        self.rows.len() as isize
    }

    fn insert(&mut self, c: &Coord) {
        let rows_to_insert = c.y - self.height() + 1;
        if rows_to_insert > 0 {
            self.rows.push(Row::default());
        }
        self.rows[c.y as usize][c.x as usize] = true;
    }
}

impl Display for Tower {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter().rev() {
            '|'.fmt(f)?;
            row.iter()
                .map(|rock| if *rock { '#' } else { '.' })
                .for_each(|c| c.fmt(f).unwrap());
            "|\n".fmt(f)?;
        }
        "+-------+\n".fmt(f)
    }
}

const TOWER_WIDTH: isize = 7;

pub fn day17(input: &str) -> aoc22::MyResult<(isize, isize)> {
    let winds: Vec<_> = input
        .chars()
        .filter_map(|s| match s {
            '<' => Some(Wind::Left),
            '>' => Some(Wind::Right),
            _ => None,
        })
        .collect();
    let shapes: [Shape; 5] = [
        // ####
        vec![c!(2, 3), c!(3, 3), c!(4, 3), c!(5, 3)],
        // .#.
        // ###
        // .#.
        vec![c!(3, 3), c!(2, 4), c!(3, 4), c!(4, 4), c!(3, 5)],
        // ..#
        // ..#
        // ###
        vec![c!(2, 3), c!(3, 3), c!(4, 3), c!(4, 4), c!(4, 5)],
        // #
        // #
        // #
        // #
        vec![c!(2, 3), c!(2, 4), c!(2, 5), c!(2, 6)],
        // ##
        // ##
        vec![c!(2, 3), c!(2, 4), c!(3, 3), c!(3, 4)],
    ];

    let mut tower = Tower::with_height_capacity(5000);

    let mut wind_index: usize = 0;
    for i in 0..2022 {
        let mut shape = spawn(&shapes[i % shapes.len()], tower.height());
        apply_wind(&mut shape, &tower, &mut wind_index, &winds);
        while fall(&mut shape, &tower) {
            apply_wind(&mut shape, &tower, &mut wind_index, &winds);
        }
        stop_shape(&shape, &mut tower);
        //println!("{}", tower);
    }
    let part1 = tower.height();

    let part2 = 0;

    Ok((part1, part2))
}

fn spawn(shape: &Shape, height: isize) -> Shape {
    shape.iter().map(|coord| *coord + c!(0, height)).collect()
}

fn apply_wind(shape: &mut Shape, tower: &Tower, wind_index: &mut usize, winds: &[Wind]) {
    let wind = winds[*wind_index];

    let offset = wind.offset();
    if !collides(shape, offset, tower) {
        move_shape(shape, offset);
    }

    *wind_index = (*wind_index + 1) % winds.len();
}

fn fall(shape: &mut Shape, tower: &Tower) -> bool {
    let offset = c!(0, -1);
    let falls = !collides(shape, offset, tower);
    if falls {
        move_shape(shape, offset);
    }
    falls
}

fn collides(shape: &Shape, offset: Coord, tower: &Tower) -> bool {
    let collides_wall = |c: &Coord| c.y < 0 || c.x < 0 || c.x > TOWER_WIDTH - 1;
    shape
        .iter()
        .map(|c| *c + offset)
        .any(|c| collides_wall(&c) || tower.is_rock(&c))
}

fn move_shape(shape: &mut Shape, offset: Coord) {
    for c in shape.iter_mut() {
        *c = *c + offset;
    }
}

fn stop_shape(shape: &Shape, tower: &mut Tower) {
    for c in shape {
        tower.insert(c);
    }
}
