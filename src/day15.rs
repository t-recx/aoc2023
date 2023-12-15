use crate::day::Day;

pub struct Day15;

impl Day for Day15 {
    fn one(&self, input: &str) -> String {
		return input.split(',').map(|x| get_hash(x)).sum::<i32>().to_string();
    }

    fn two(&self, input: &str) -> String {
		return "".to_string();
    }
}

fn get_hash(original: &str) -> i32 {
	return original.chars().fold(0, |acc, c| ((acc + (c as i32)) * 17) % 256);
}