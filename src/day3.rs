use crate::day::Day;

pub struct Day3;

impl Day for Day3 {
	fn one(&self, input: &str) -> String {
		let matrix: Vec<Vec<char>> = get_matrix(input);
		let mut y = 0;
		let mut part_number: Vec<char> = Vec::new();
		let mut parts: Vec<i32> = Vec::new();

		while y < matrix.len() {
			let mut x = 0;
			let mut add_to_parts = false;
			part_number.clear();

			while x < matrix[y].len() {
				let c = matrix[y][x];

				if c.is_numeric() {
					part_number.push(c);

					if is_touching_symbol(&matrix, x, y) {
						add_to_parts = true;
					}
				}
				
				if !c.is_numeric() || x == matrix[y].len() - 1 {
					if add_to_parts && part_number.len() > 0 {
						parts.push(part_number.clone().into_iter().collect::<String>().parse::<i32>().unwrap());
						add_to_parts = false;
					}

					part_number.clear();
				}

				x+=1;
			}

			y+=1;
		}

		return parts.iter().sum::<i32>().to_string();
	}

	fn two(&self, input: &str) -> String {
		let matrix: Vec<Vec<char>> = get_matrix(input);
		let mut y = 0;
		let mut gear_ratios: Vec<i32> = Vec::new();

		while y < matrix.len() {
			let mut x = 0;

			while x < matrix[y].len() {
				let c = matrix[y][x];

				if c == '*' {
					let gear_ratio = get_gear_ratio(&matrix, x, y);

					if gear_ratio > 0 {
						gear_ratios.push(gear_ratio);
					}
				}

				x+=1;
			}

			y+=1;
		}

		return gear_ratios.iter().sum::<i32>().to_string();
	}
}

fn get_gear_ratio(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> i32 {
	let mut places_to_check: Vec<(usize, usize)> = Vec::new();
	let mut number_of_adjacent_parts = 0;

	if x > 0 {
		if matrix[y][x - 1].is_numeric() {
			number_of_adjacent_parts += 1;
			places_to_check.push((x-1, y));
		}
	}

	if x + 1 < matrix[y].len() {
		if matrix[y][x + 1].is_numeric() {
			number_of_adjacent_parts += 1;
			places_to_check.push((x+1, y));
		}
	}

	if y > 0 {
		if matrix[y - 1][x].is_numeric() {
			number_of_adjacent_parts += 1;
			places_to_check.push((x, y-1));
		}
		else { 
			if x > 0 {
				if matrix[y - 1][x - 1].is_numeric() {
					number_of_adjacent_parts += 1;
					places_to_check.push((x-1, y-1));
				}
			}

			if x + 1 < matrix[y - 1].len() {
				if matrix[y - 1][x + 1].is_numeric() {
					number_of_adjacent_parts += 1;
					places_to_check.push((x+1, y-1));
				}
			}
		}
	}

	if y + 1 < matrix.len() {
		if matrix[y + 1][x].is_numeric() {
			number_of_adjacent_parts += 1;
			places_to_check.push((x, y+1));
		}
		else { 
			if x > 0 {
				if matrix[y + 1][x - 1].is_numeric() {
					number_of_adjacent_parts += 1;
					places_to_check.push((x-1, y+1));
				}
			}

			if x + 1 < matrix[y + 1].len() {
				if matrix[y + 1][x + 1].is_numeric() {
					number_of_adjacent_parts += 1;
					places_to_check.push((x+1, y+1));
				}
			}
		}
	}

	if number_of_adjacent_parts != 2 {
		return 0;
	}

	let mut parts: Vec<i32> = Vec::new();

	for place_to_check in places_to_check {
		let mut part_number: Vec<char> = Vec::new();
		let mut x = place_to_check.0;

		loop {
			let c = matrix[place_to_check.1][x];

			if !c.is_numeric() {
				break;
			}

			part_number.insert(0, c);

			if x == 0 {
				break;
			}

			x-=1;
		}

		x = place_to_check.0 + 1;

		while x < matrix[place_to_check.1].len() {
			let c = matrix[place_to_check.1][x];

			if !c.is_numeric() {
				break;
			}

			part_number.push(c);
			x+=1;
		}

		parts.push(part_number.clone().into_iter().collect::<String>().parse::<i32>().unwrap());
	}

	return parts[0] * parts[1];
}

fn is_touching_symbol(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
	let directions: Vec<(i32, i32)> = vec![
		(-1, 0), (1, 0), 
		(-1, -1), (-1, 1), 
		(1, -1), (1, 1),
		(0, -1), (0, 1)
	];

	for direction in directions {
		let vx = x as i32 + direction.0;
		let vy = y as i32 + direction.1;
		if vx >= 0 && vx < matrix[y].len() as i32 {
			if vy >= 0 && vy < matrix.len() as i32 {
				if is_symbol(matrix[vy as usize][vx as usize]) {
					return true;
				}
			}
		}
	}

	return false;
}

fn is_symbol(c: char) -> bool {
	return !c.is_numeric() && c != '.';
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
	let mut matrix: Vec<Vec<char>> = Vec::new();

	for line in input.lines() {
		let mut line_vec: Vec<char> = Vec::new();

		for c in line.chars() {
			line_vec.push(c);
		}

		matrix.push(line_vec);
	}

	return matrix;
}