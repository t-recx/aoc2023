use std::fs;
use std::env;

mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use day::Day;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;

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
		5 => {
			let day = Day5 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		6 => {
			let day = Day6 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		7 => {
			let day = Day7 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		8 => {
			let day = Day8 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		9 => {
			let day = Day9 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		10 => {
			let day = Day10 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		11 => {
			let day = Day11 {};

			println!("{}", day.one(&input));
			println!("{}", day.two(&input));
		},
		_ => println!("Day not available"),
	}
}
