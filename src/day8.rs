use ndarray::{s, Array2};

use take_until::TakeUntilExt;

aoc22::main!(day8, "../inputs/input8.txt");

aoc22::test_with_example!(day8, "../inputs/example8.txt", 21, 8);

pub fn day8(input: &str) -> aoc22::MyResult<(u32, u32)> {
    let forest = parse_input(input)?;

    let part1: u32 = forest
        .indexed_iter()
        .map(|(coord, _)| is_visible(coord, &forest) as u32)
        .sum();

    let part2: u32 = forest
        .indexed_iter()
        .map(|(coord, _)| scenic_score(coord, &forest))
        .max()
        .unwrap();

    Ok((part1, part2))
}

fn parse_input(input: &str) -> Result<Array2<u32>, ndarray::ShapeError> {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines.len();
    let height = lines[0].len();

    let flat: Vec<u32> = lines
        .into_iter()
        .flat_map(|line| line.chars().map(|c| char::to_digit(c, 10).unwrap()))
        .collect();

    Array2::from_shape_vec((width, height), flat)
}

fn get_trees_in_all_directions((r, c): (usize, usize), forest: &Array2<u32>) -> [Vec<u32>; 4] {
    let column_slice = |col, slice| forest.column(col).slice(slice).iter().copied().collect();
    let row_slice = |row, slice| forest.row(row).slice(slice).iter().copied().collect();
    let mut directions: [Vec<u32>; 4] = [
        column_slice(c, s![..r]),     //up
        column_slice(c, s![r + 1..]), //down
        row_slice(r, s![..c]),        //left
        row_slice(r, s![c + 1..]),    //right
    ];
    directions[0].reverse();
    directions[2].reverse();

    directions
}

fn is_visible(coord: (usize, usize), forest: &Array2<u32>) -> bool {
    let tree_height = forest[coord];
    get_trees_in_all_directions(coord, forest)
        .iter()
        .any(|direction| direction.iter().all(|&tree| tree < tree_height))
}

fn scenic_score(coord: (usize, usize), forest: &Array2<u32>) -> u32 {
    let treehouse_height = forest[coord];
    get_trees_in_all_directions(coord, forest)
        .into_iter()
        .map(|direction| {
            direction
                .iter()
                .take_until(|&&tree| tree >= treehouse_height) // stop at first high tree
                .count() as u32
        })
        .product()
}
