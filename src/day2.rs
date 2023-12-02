use std::collections::HashMap;

use crate::day::Day;

pub struct Day2;

impl Day for Day2 {
	fn one(&self, input: &str) -> String {
		let games: HashMap<i32, Vec<HashMap<&str, i32>>> = get_games(input);
		let mut cubes_in_bag_per_color: HashMap<&str, i32> = HashMap::new();
		let mut sum = 0;

		cubes_in_bag_per_color.insert("red", 12);
		cubes_in_bag_per_color.insert("green", 13);
		cubes_in_bag_per_color.insert("blue", 14);

		for (id, rounds) in games.iter() {
			let mut possible_game = true;

			for round in rounds {
				for (bag_color, bag_count) in &cubes_in_bag_per_color {
					match round.get(bag_color) {
						Some(value) => {
							if value > &bag_count {
								possible_game = false;
								break;
							}
						},
						None => {}
					}
				}

				if !possible_game {
					break;
				}
			}

			if !possible_game {
				continue;
			}

			sum += id;
		}

		return sum.to_string();
	}

	fn two(&self, input: &str) -> String {
		let games: HashMap<i32, Vec<HashMap<&str, i32>>> = get_games(input);
		let mut sum = 0;

		for (_, rounds) in games {
			let mut max_cubes_per_color: HashMap<&str, i32> = HashMap::new();
			let mut cubes_power = 1;

			for round in rounds {
				for (color, cubes) in round {
					match max_cubes_per_color.get(color) {
						Some(value) => { 
							if cubes > *value {
								max_cubes_per_color.entry(color).and_modify(|v| *v = cubes);
							}
						},
						None => {
							max_cubes_per_color.insert(color, cubes);
						}
					}
				} 
			}

			for (_, cubes) in max_cubes_per_color {
				cubes_power *= cubes;
			}

			sum += cubes_power;
		}

		return sum.to_string();
	}
}

fn get_games(input: &str) -> HashMap<i32, Vec<HashMap<&str, i32>>> {
	let mut games: HashMap<i32, Vec<HashMap<&str, i32>>> = HashMap::new();

	for line in input.lines() {
		let tokens: Vec<&str> = line.split(':').collect();
		let game_id = tokens[0].split(' ').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

		let rounds_tokens: Vec<&str> = tokens[1].split(';').collect();
		let mut rounds: Vec<HashMap<&str, i32>> = Vec::new();

		for round_token in rounds_tokens {
			let mut cubes_by_color: HashMap<&str, i32> = HashMap::new ();
			let colors_tokens: Vec<&str> = round_token.split(',').collect();

			for color_token in colors_tokens {
				let colors_times: Vec<&str> = color_token.trim().split(' ').collect();

				for chunk in colors_times.chunks(2) {
					match chunk {
						[times, color] => {
							cubes_by_color.insert(color, times.parse::<i32>().unwrap());
						}
						_ => {}
					}
				}
			}

			rounds.push(cubes_by_color);
		}

		games.insert(game_id, rounds);
	}

	return games;
}