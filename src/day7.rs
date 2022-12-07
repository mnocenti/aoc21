use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Default)]
struct DirPos {
    full_name: String,
    line: usize,
}

const CD_CMD: &str = "$ cd ";

fn main() {
    let lines = include_str!("../inputs/input7.txt").lines().collect_vec();
    let dir_index = create_dir_index(&lines);
    let dir_sizes = compute_sizes(&lines, dir_index);

    let part1 = dir_sizes
        .iter()
        .filter_map(|(_, &size)| if size <= 100000 { Some(size) } else { None })
        .sum::<usize>();
    println!("{}", part1);

    let free_space = 70000000 - dir_sizes["/"];
    let to_free = 30000000 - free_space;
    let part2 = dir_sizes
        .iter()
        .filter_map(|(_, &size)| if size >= to_free { Some(size) } else { None })
        .min()
        .unwrap();
    println!("{}", part2);
}

/// Create an index storing the starting line of each directories
fn create_dir_index(lines: &Vec<&str>) -> Vec<DirPos> {
    let cd_lines = lines
        .iter()
        .enumerate()
        .filter(|(_, line)| line.starts_with(CD_CMD));
    let mut dir_index = Vec::new();
    let mut add_dir_to_index = |full_name: &String, line| {
        dir_index.push(DirPos {
            full_name: full_name.clone(),
            line,
        })
    };
    let mut cur_dir = String::new();
    for (i, cd_line) in cd_lines {
        match &cd_line[CD_CMD.len()..] {
            ".." => {
                cur_dir = match cur_dir.rsplit_once("/") {
                    Some((prefix, _)) => String::from(prefix),
                    None => String::new(),
                }
            }
            "/" => {
                cur_dir = String::from("/");
                add_dir_to_index(&cur_dir, i);
            }
            x => {
                cur_dir = join_dir(&cur_dir, x);
                add_dir_to_index(&cur_dir, i);
            }
        }
    }
    dir_index
}

/// Compute the size of each directory and return it in a HashMap
fn compute_sizes(lines: &Vec<&str>, dir_index: Vec<DirPos>) -> HashMap<String, usize> {
    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    dir_index
        .into_iter()
        // compute sizes in reserve to be sure that the size of an inner dir is computed before the outer dir
        .rev()
        .for_each(|current_dir| {
            // the line just after cd should be a 'ls'
            assert_eq!(lines[current_dir.line + 1], "$ ls");
            // for the current dir, just read the output of its direct 'ls' command
            // the size of all its subdirs are already computed
            let ls_output = lines[current_dir.line + 2..]
                .iter()
                .take_while(|l| !l.starts_with("$"));
            let current_size: usize = ls_output
                .filter_map(|l| match l.split_once(" ") {
                    Some(("dir", d)) => {
                        let sub_dir = join_dir(&current_dir.full_name, d);
                        Some(dir_sizes[&sub_dir])
                    }
                    Some((file_size, _)) => file_size.parse::<usize>().ok(),
                    _ => None,
                })
                .sum();
            dir_sizes.insert(current_dir.full_name, current_size);
        });
    dir_sizes
}

fn join_dir(parent: &str, child: &str) -> String {
    if parent == "/" {
        parent.to_owned() + child
    } else {
        parent.to_owned() + "/" + child
    }
}
