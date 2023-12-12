use std::collections::HashMap;

use crate::day::Day;

pub struct Day4;

impl Day for Day4 {
    fn one(&self, input: &str) -> String {
        return input
            .lines()
            .fold(0, |acc, line| {
                let common_count = get_common_count(line);

                acc + if common_count == 0 {
                    0
                } else {
                    2u32.pow(common_count - 1)
                }
            })
            .to_string();
    }

    fn two(&self, input: &str) -> String {
        let lines = input.lines().collect::<Vec<&str>>();
        return get_card_number(
            &lines,
            0,
            lines.len(),
            &mut HashMap::new(),
            &mut HashMap::new(),
        )
        .to_string();
    }
}

fn get_card_number(
    lines: &[&str],
    start: usize,
    end: usize,
    common_counts: &mut HashMap<usize, u32>,
    card_numbers_per_indexes: &mut HashMap<(usize, usize), u32>,
) -> u32 {
    let mut card_number = 0;
    let mut i = start;

    while i < end {
        let common_count;

        if common_counts.contains_key(&i) {
            common_count = common_counts[&i];
        } else {
            common_count = get_common_count(lines[i]);
            common_counts.insert(i, common_count);
        }

        if common_count > 0 {
            let new_start = i + 1;
            let mut new_end = i + common_count as usize + 1;

            if new_end > lines.len() {
                new_end = lines.len();
            }

            if card_numbers_per_indexes.contains_key(&(new_start, new_end)) {
                card_number += card_numbers_per_indexes[&(new_start, new_end)];
            } else {
                let card_number_indexes = get_card_number(
                    lines,
                    new_start,
                    new_end,
                    common_counts,
                    card_numbers_per_indexes,
                );
                card_numbers_per_indexes.insert((new_start, new_end), card_number_indexes);

                card_number += card_number_indexes;
            }
        }

        i += 1;
        card_number += 1;
    }

    return card_number;
}

fn get_common_count(line: &str) -> u32 {
    let sides: Vec<Vec<i32>> = line
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .split('|')
        .map(|x| x.trim())
        .collect::<Vec<&str>>()
        .iter()
        .map(|&x| {
            x.split(' ')
                .filter(|&x| !x.trim().is_empty())
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    return sides[0].iter().filter(|&&x| sides[1].contains(&x)).count() as u32;
}
