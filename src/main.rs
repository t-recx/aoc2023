use std::fs;
use std::env;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;

use day::Day;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;

fn main() {
	let args: Vec<String> = env::args().skip(1).collect();
	let day_number: i8 = args[0].parse().unwrap();

	let input = fs::read_to_string(format!("./input/day{}.input", day_number)).unwrap();

	match day_number {
		1 => {
			let day = Day1 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		2 => {
			let day = Day2 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		3 => {
			let day = Day3 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		4 => {
			let day = Day4 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		_ => println!("Day not available"),
	}
}
