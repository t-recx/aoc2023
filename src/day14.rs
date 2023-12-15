use crate::day::Day;

pub struct Day14;

impl Day for Day14 {
    fn one(&self, input: &str) -> String {
		let mut matrix = get_matrix(input);
		let mut rocks: Vec<(i32, i32)> = Vec::new();
		let mut y = 0;
		
		while y < matrix.len() {
			let mut x = 0;

			while x < matrix[y].len() {
				if matrix[y][x] == 'O' {
					rocks.push((x as i32, y as i32));
					matrix[y][x] = '.';
				}

				x+=1;
			}	

			y+=1;
		}

		for rock in &mut rocks {
			let mut y = rock.1;

			loop {
				if y == 0 || matrix[(y-1) as usize][rock.0 as usize] != '.' {
					break;
				}

				y-=1;
			}

			rock.1 = y;
			matrix[rock.1 as usize][rock.0 as usize] = 'O';
		}

		let mut sum = 0;

		y = 0;
		while y < matrix.len() {
			sum += rocks.iter().filter(|(_, iy)| *iy as usize == y).count() * (matrix.len() - y);
			y+=1;
		}

		return sum.to_string();
    }

    fn two(&self, input: &str) -> String {
		return "".to_string();
    }
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
    return input.lines().map(|line| line.chars().collect()).collect();
}