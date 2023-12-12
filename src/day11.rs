use std::collections::HashSet;

use crate::day::Day;

pub struct Day11;

impl Day for Day11 {
    fn one(&self, input: &str) -> String {
        return compute(input, 1).to_string();
    }

    fn two(&self, input: &str) -> String {
        return compute(input, 1000000).to_string();
    }
}

fn compute(input: &str, expansion_number: usize) -> i128 {
    let galaxies = get_galaxies(input, expansion_number);
    let mut already_computed: HashSet<((usize, usize), (usize, usize))> = HashSet::new();
    let mut distances: Vec<i128> = Vec::new();

    for galaxy in &galaxies {
        for another_galaxy in &galaxies {
            if galaxy == another_galaxy {
                continue;
            }

            if already_computed.contains(&(*galaxy, *another_galaxy)) {
                continue;
            }

            if already_computed.contains(&(*another_galaxy, *galaxy)) {
                continue;
            }

            distances.push(get_distance(*galaxy, *another_galaxy));
            already_computed.insert((*galaxy, *another_galaxy));
        }
    }

    return distances.iter().sum::<i128>();
}

fn get_galaxies(input: &str, expansion_number: usize) -> Vec<(usize, usize)> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut x: usize;
    let mut y: usize = 0;

    for row in &grid {
        let mut found = false;
        x = 0;
        for &cell in row {
            if cell == '#' {
                galaxies.push((x, y));
                found = true;
            }
            x += 1;
        }
        if !found {
            y += expansion_number + (if expansion_number == 1 { 1 } else { 0 });
        } else {
            y += 1;
        }
    }

    x = grid[0].len() - 1;

    loop {
        let mut found = false;
        y = 0;
        while y < grid.len() {
            if grid[y][x] != '.' {
                found = true;
                break;
            }

            y += 1;
        }

        if !found {
            galaxies
                .iter_mut()
                .filter(|&&mut (ex, _)| ex > x)
                .for_each(|&mut (ref mut ex, _)| {
                    *ex += expansion_number - (if expansion_number == 1 { 0 } else { 1 });
                });
        }

        if x > 0 {
            x -= 1;
        } else {
            break;
        }
    }

    return galaxies;
}

fn get_distance(galaxy: (usize, usize), another_galaxy: (usize, usize)) -> i128 {
    return ((galaxy.0 as i128) - (another_galaxy.0 as i128)).abs()
        + ((galaxy.1 as i128) - (another_galaxy.1 as i128)).abs();
}
