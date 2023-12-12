use std::collections::HashMap;
use std::collections::HashSet;

use crate::day::Day;

pub struct Day10;

struct Schematics {
    starting_position: (i32, i32),
    pipes: HashMap<(i32, i32), Vec<(i32, i32)>>,
    tiles: HashMap<(i32, i32), char>,
}

impl Day for Day10 {
    fn one(&self, input: &str) -> String {
        return (get_data(input).1.len() / 2).to_string();
    }

    fn two(&self, input: &str) -> String {
        return get_count(input).to_string();
    }
}

fn get_count(input: &str) -> i32 {
    let data = get_data(input);
    let schematics = data.0;
    let main_pipe = data.1;
    let tiles = schematics.tiles;
    let mut y = 0;
    let mut count = 0;
    let height = schematics.pipes.keys().map(|&(_, y)| y).max().unwrap();
    let width = schematics.pipes.keys().map(|&(x, _)| x).max().unwrap();

    while y < height {
        let mut x = 0;
        let mut collect = false;
        let mut last_corner = ' ';
        while x < width {
            let tile = tiles[&(x, y)];
            if main_pipe.contains(&(x, y)) {
                if tile == '|' {
                    collect = !collect;
                } else if tile == 'L' {
                    last_corner = tile;
                } else if tile == 'F' {
                    last_corner = tile;
                } else if tile == 'J' {
                    if last_corner == 'F' {
                        collect = !collect;
                    }
                } else if tile == '7' {
                    if last_corner == 'L' {
                        collect = !collect;
                    }
                }
            } else {
                if collect {
                    count += 1;
                }
            }
            x += 1;
        }
        y += 1;
    }

    return count;
}

fn get_data(input: &str) -> (Schematics, HashSet<(i32, i32)>) {
    let schematics = get_schematics(input);
    let starting_position = schematics.starting_position;
    let mut pipes = schematics.pipes.clone();

    let mut x: i32 = starting_position.0;
    let mut y: i32 = starting_position.1;
    let mut already_visited: HashSet<(i32, i32)> = HashSet::new();

    loop {
        already_visited.insert((x, y));

        let current_pipe = pipes.entry((x, y)).or_insert_with(Vec::new);
        if let Some(offset) = current_pipe.pop() {
            let new_x = x + offset.0;
            let new_y = y + offset.1;
            if !already_visited.contains(&(new_x, new_y)) {
                x = new_x;
                y = new_y;
            }
        } else {
            break;
        }
    }

    return (schematics, already_visited);
}

fn get_schematics(input: &str) -> Schematics {
    let mut x;
    let mut y = 0;
    let mut starting_position: (i32, i32) = (0, 0);
    let mut pipes: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    let mut pipe_types: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut tiles: HashMap<(i32, i32), char> = HashMap::new();
    let directions: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (-1, 0), (1, 0)];
    pipe_types.insert('F', vec![(1, 0), (0, 1)]);
    pipe_types.insert('|', vec![(0, -1), (0, 1)]);
    pipe_types.insert('L', vec![(0, -1), (1, 0)]);
    pipe_types.insert('-', vec![(-1, 0), (1, 0)]);
    pipe_types.insert('7', vec![(-1, 0), (0, 1)]);
    pipe_types.insert('J', vec![(-1, 0), (0, -1)]);

    for line in input.lines() {
        x = 0;

        for c in line.chars() {
            if c != 'S' {
                tiles.insert((x, y), c);
            }

            if c == 'S' {
                starting_position = (x, y);
            } else if c != '.' {
                pipes.insert((x, y), pipe_types[&c].clone());
            }

            x += 1;
        }

        y += 1;
    }

    let mut starting_position_offsets: Vec<(i32, i32)> = Vec::new();
    for direction in directions {
        let vx = starting_position.0 + direction.0;
        let vy = starting_position.1 + direction.1;

        if pipes.contains_key(&(vx, vy)) {
            if pipes[&(vx, vy)].iter().any(|&offset| {
                vx + offset.0 == starting_position.0 && vy + offset.1 == starting_position.1
            }) {
                starting_position_offsets.push((direction.0, direction.1));
            }
        }
    }
    pipes.insert(
        (starting_position.0, starting_position.1),
        starting_position_offsets.clone(),
    );
    let matching_pipe_type = pipe_types.iter().find(|(_, offsets)| {
        offsets
            .iter()
            .all(|offset| starting_position_offsets.contains(offset))
    });

    if let Some((pipe_type, _)) = matching_pipe_type {
        tiles.insert((starting_position.0, starting_position.1), *pipe_type);
    }

    return Schematics {
        starting_position,
        pipes,
        tiles,
    };
}
