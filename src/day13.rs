use crate::day::Day;

pub struct Day13;

impl Day for Day13 {
    fn one(&self, input: &str) -> String {
        return get_blocks(input)
            .into_iter()
            .filter_map(|block| get_reflection_value(block))
            .sum::<i32>()
            .to_string();
    }

    fn two(&self, input: &str) -> String {
        return get_blocks(input)
            .into_iter()
            .filter_map(|block| get_reflection_value_with_fix(block))
            .sum::<i32>()
            .to_string();
    }
}

fn get_reflection_value(block: Vec<Vec<char>>) -> Option<i32> {
    return vertical_reflection(block.clone(), None)
        .or_else(|| horizontal_reflection(block.clone(), None));
}

fn get_blocks(input: &str) -> Vec<Vec<Vec<char>>> {
    return input
        .split("\n\n")
        .map(|block| block.lines().map(|line| line.chars().collect()).collect())
        .collect();
}

fn vertical_reflection(block: Vec<Vec<char>>, ignore_value_option: Option<i32>) -> Option<i32> {
    let width = block[0].len();
    let height = block.len();
    let mut x = 0;
    let mut vx = 1;

    while x < width && vx < width {
        let mut y = 0;
        let mut found_possible_symmetry = true;

        while y < height {
            if block[y][x] != block[y][vx] {
                found_possible_symmetry = false;
                break;
            }
            y += 1;
        }

        if found_possible_symmetry {
            let mut sx: i32 = x as i32 - 1;
            let mut svx = vx + 1;
            let mut is_symmetric = true;

            while sx >= 0 && svx < width && is_symmetric {
                let mut sy = 0;

                while sy < height {
                    if block[sy][sx as usize] != block[sy][svx] {
                        is_symmetric = false;
                        break;
                    }
                    sy += 1;
                }

                sx -= 1;
                svx += 1;
            }

            if is_symmetric {
                if let Some(ignore_value) = ignore_value_option {
                    if ignore_value != vx as i32 {
                        return Some(vx as i32);
                    }
                } else {
                    return Some(vx as i32);
                }
            }
        }

        x += 1;
        vx = x + 1;
    }

    return None;
}

fn horizontal_reflection(block: Vec<Vec<char>>, ignore_value_option: Option<i32>) -> Option<i32> {
    let width = block[0].len();
    let height = block.len();
    let mut y = 0;
    let mut vy = 1;

    while y < height && vy < height {
        let mut x = 0;
        let mut found_possible_symmetry = true;

        while x < width {
            if block[y][x] != block[vy][x] {
                found_possible_symmetry = false;
                break;
            }
            x += 1;
        }

        if found_possible_symmetry {
            let mut sy: i32 = y as i32 - 1;
            let mut svy = vy + 1;
            let mut is_symmetric = true;

            while sy >= 0 && svy < height && is_symmetric {
                let mut sx = 0;

                while sx < width {
                    if block[sy as usize][sx] != block[svy][sx] {
                        is_symmetric = false;
                        break;
                    }
                    sx += 1;
                }

                sy -= 1;
                svy += 1;
            }

            if is_symmetric {
                if let Some(ignore_value) = ignore_value_option {
                    if ignore_value != vy as i32 * 100 {
                        return Some(vy as i32 * 100);
                    }
                } else {
                    return Some(vy as i32 * 100);
                }
            }
        }

        y += 1;
        vy = y + 1;
    }

    return None;
}

fn get_reflection_value_with_fix(block: Vec<Vec<char>>) -> Option<i32> {
    let width = block[0].len();
    let height = block.len();
    let mut y = 0;
    let current_reflection_value = get_reflection_value(block.clone()).unwrap();

    while y < height {
        let mut x = 0;

        while x < width {
            let mut new_block = block.clone();

			new_block[y][x] = if new_block[y][x] == '.' { '#' } else { '.' };

            if let Some(new_reflection_value) =
                horizontal_reflection(new_block.clone(), Some(current_reflection_value))
            {
                if new_reflection_value != current_reflection_value {
                    return Some(new_reflection_value);
                }
            }

            if let Some(new_reflection_value) =
                vertical_reflection(new_block.clone(), Some(current_reflection_value))
            {
                if new_reflection_value != current_reflection_value {
                    return Some(new_reflection_value);
                }
            }

            x += 1;
        }

        y += 1;
    }

    return Some(current_reflection_value);
}
