use crate::day::Day;

pub struct Day5;

pub struct Mapping {
	pub destination_range_start: u64,
	pub source_range_start: u64,
	pub range_length: u64
}

pub struct Almanac {
	pub seeds: Vec<u64>,
	pub mappings: Vec<Vec<Mapping>>
}

impl Day for Day5 {
	fn one(&self, input: &str) -> String {
		let almanac = get_almanac(input);
		let mut min_location = std::u64::MAX;

		for seed in almanac.seeds {
			let mut in_evaluation = seed;

			for internal_mappings in &almanac.mappings {
				for internal_mapping in internal_mappings {
					if  in_evaluation >= internal_mapping.source_range_start &&
						in_evaluation <= internal_mapping.source_range_start + internal_mapping.range_length {
						in_evaluation = internal_mapping.destination_range_start + (in_evaluation - internal_mapping.source_range_start);
						break;
					}
				}	
			}

			if in_evaluation < min_location {
				min_location = in_evaluation;
			}
		}

		return min_location.to_string();
	}

	fn two(&self, input: &str) -> String {
		return "".to_string();
	}
}

fn get_almanac(input: &str) -> Almanac
{
	let mut seeds: Vec<u64> = Vec::new();
	let mut mappings: Vec<Vec<Mapping>> = Vec::new();
	let lines = input.lines();
	let mut i = 0;

	for line in lines {
		if i == 0 {
			seeds = line.split(':').skip(1).next().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
		}
		else if i > 1 {
			if !line.trim().is_empty() {
				if !line.chars().next().unwrap().is_numeric() {
					mappings.push(Vec::new());
				}
				else {
					let tokens = line.split(' ').map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>();

					mappings.last_mut().unwrap().push(Mapping { 
						destination_range_start: tokens[0],
						source_range_start: tokens[1],
						range_length: tokens[2]
					});
				}
			}
		}

		i+=1;
	}

	return Almanac {
		seeds: seeds,
		mappings: mappings
	};
}