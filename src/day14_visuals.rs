use std::{collections::VecDeque, fmt::Display, ops::Index};

use itertools::Itertools;

use pixel_canvas::{input::MouseState, Canvas, Color, Image, RC};

aoc22::main!(day14_visuals, "../inputs/input14.txt");

const TILE_SIZE: usize = 4;

pub fn day14_visuals(input: &str) -> aoc22::MyResult<(usize, usize)> {
    let (mut cave, sand_entry) = parse_cave(input)?;
    add_bedrock(&mut cave);
    let mut count = 0usize;
    let mut sand_grains = VecDeque::new();
    let mut speed = 1.0;
    let canvas = Canvas::new(cave.dimensions.0 * TILE_SIZE, cave.dimensions.1 * TILE_SIZE)
        .title("I don't like sand")
        .state(MouseState::new())
        .show_ms(true);
    canvas.render(move |_, image| {
        if count == 0 {
            cave.draw(image);
            add_sand(&cave, sand_entry, &mut sand_grains);
        }
        speed *= 1.005;
        for _ in 0..(speed as usize) {
            process_sand(&mut cave, &mut sand_grains, image);
            count += 1;
        }
    });

    Ok((0, 0))
}

type Coord = (usize, usize);
type Path = Vec<Coord>;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
    FlowingSand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Air => '.',
            Tile::Rock => '#',
            Tile::Sand => 'o',
            Tile::FlowingSand => 'o',
        }
        .fmt(f)
    }
}

struct Cave {
    dimensions: (usize, usize),
    tiles: Vec<Vec<Tile>>,
}

impl Index<Coord> for Cave {
    type Output = Tile;

    fn index(&self, (x, y): Coord) -> &Self::Output {
        &self.tiles[x][y]
    }
}

impl Cave {
    fn new((width, height): (usize, usize)) -> Cave {
        Cave {
            dimensions: (width, height),
            tiles: vec![vec![Tile::Air; height]; width],
        }
    }

    fn width(&self) -> usize {
        self.dimensions.0
    }
    fn height(&self) -> usize {
        self.dimensions.1
    }

    fn draw(&self, image: &mut Image) {
        for point in (0..self.width()).cartesian_product(0..self.height()) {
            Self::draw_tile(point, self[point], image);
        }
    }

    fn draw_tile((x, y): Coord, tile: Tile, image: &mut Image) {
        let color = Self::get_color(tile);
        let (x, y) = (x * TILE_SIZE, y * TILE_SIZE);
        draw_square(image, (x, y), TILE_SIZE, color);
    }

    fn set_tile(&mut self, (x, y): Coord, tile: Tile) {
        self.tiles[x][y] = tile;
    }

    fn set_and_draw_tile(&mut self, (x, y): Coord, tile: Tile, image: &mut Image) {
        self.tiles[x][y] = tile;
        Self::draw_tile((x, y), tile, image);
    }

    const AIR_COLOR: Color = Color {
        r: 0x7C,
        g: 0xCC,
        b: 0xCA,
    };
    const ROCK_COLOR: Color = Color {
        r: 0x33,
        g: 0x1F,
        b: 0x1F,
    };
    const SAND_COLOR: Color = Color {
        r: 0xDC,
        g: 0xCD,
        b: 0x79,
    };
    const FLOWING_SAND_COLOR: Color = Color {
        r: 0xAC,
        g: 0xCC,
        b: 0xA1,
    };
    fn get_color(tile: Tile) -> Color {
        match tile {
            Tile::Air => Self::AIR_COLOR,
            Tile::Rock => Self::ROCK_COLOR,
            Tile::Sand => Self::SAND_COLOR,
            Tile::FlowingSand => Self::FLOWING_SAND_COLOR,
        }
    }
}

fn draw_square(image: &mut Image, (topx, topy): (usize, usize), size: usize, color: Color) {
    let height = image.height();
    for (x, y) in (topx..topx + size).cartesian_product(topy..topy + size) {
        image[RC(height - y - 1, x)] = color;
    }
}

#[derive(Debug, PartialEq)]
enum SandMovement {
    Move(Coord),
    Rest,
    EndlessAbyss,
}

fn add_sand(cave: &Cave, source_pos: Coord, sand_grains: &mut VecDeque<Coord>) {
    if cave[source_pos] == Tile::Air {
        sand_grains.push_back(source_pos);
    }
}

fn process_sand(cave: &mut Cave, sand_grains: &mut VecDeque<Coord>, image: &mut Image) {
    if let Some(&grain) = sand_grains.front() {
        let result = sand_physics(cave, grain);
        if let SandMovement::Move(new_pos) = result {
            Cave::draw_tile(new_pos, Tile::FlowingSand, image);
            sand_grains.push_front(new_pos);
        }
        if let SandMovement::Rest = result {
            cave.set_and_draw_tile(grain, Tile::Sand, image);
        }
        if result == SandMovement::EndlessAbyss || result == SandMovement::Rest {
            sand_grains.pop_front();
        }
    }
}

fn sand_physics(cave: &Cave, (x, y): Coord) -> SandMovement {
    if y == cave.height() - 1 {
        SandMovement::EndlessAbyss
    } else if cave[(x, y + 1)] == Tile::Air {
        SandMovement::Move((x, y + 1))
    } else if x > 0 && cave[(x - 1, y + 1)] == Tile::Air {
        SandMovement::Move((x - 1, y + 1))
    } else if x < cave.width() - 1 && cave[(x + 1, y + 1)] == Tile::Air {
        SandMovement::Move((x + 1, y + 1))
    } else {
        SandMovement::Rest
    }
}

fn parse_cave(input: &str) -> aoc22::MyResult<(Cave, Coord)> {
    let paths = input
        .lines()
        .map(parse_path)
        .collect::<Result<Vec<_>, _>>()?;
    let (min_x, max_x) = paths
        .iter()
        .flatten()
        .map(|(x, _)| x)
        .minmax()
        .into_option()
        .ok_or("No paths")?;
    let max_y = paths
        .iter()
        .flatten()
        .map(|(_, y)| y)
        .max()
        .ok_or("No paths")?;

    // height : account for the added rock at the bottom in part 2
    let max_height = max_y + 3;
    // width : add 2 for the side holes needed to guarantee a path to the abyss
    let max_starting_width = (max_x - min_x) + 2;
    // width that is big enough for any possible pile of sand
    let max_final_width = max_height * 2 + max_starting_width;
    // shift every x coord to put the sand source in the middle
    let x_shift = (max_final_width / 2) as isize - 500;

    let mut cave = Cave::new((max_final_width, max_height));

    // add rocks from paths
    let mut add_rock =
        |x: usize, y: usize| cave.set_tile(((x as isize + x_shift) as usize, y), Tile::Rock);
    for p in paths {
        for (&(start_x, start_y), &(end_x, end_y)) in p.iter().tuple_windows() {
            if start_x == end_x {
                (start_y.min(end_y)..=start_y.max(end_y)).for_each(|y| add_rock(start_x, y));
            } else {
                (start_x.min(end_x)..=start_x.max(end_x)).for_each(|x| add_rock(x, start_y));
            }
        }
    }
    Ok((cave, ((500 + x_shift) as usize, 0)))
}

fn add_bedrock(cave: &mut Cave) {
    let max_y = cave.height() - 1;
    for x in 0..cave.width() {
        cave.set_tile((x, max_y), Tile::Rock);
    }
}

fn parse_path(line: &str) -> aoc22::MyResult<Path> {
    line.split(" -> ")
        .map(|points| {
            let (x, y) = points.split_once(',').ok_or("parse error")?;
            Ok((x.parse::<usize>()?, y.parse::<usize>()?))
        })
        .collect()
}
