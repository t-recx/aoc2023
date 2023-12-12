use crate::day::Day;

pub struct Day9;

impl Day for Day9 {
    fn one(&self, input: &str) -> String {
        let histories = get_histories(input);

        return histories
            .iter()
            .flat_map(|x| x.iter().flat_map(|y| y.last()))
            .sum::<i32>()
            .to_string();
    }

    fn two(&self, input: &str) -> String {
        let histories = get_histories(input);

        return histories
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| y.first())
                    .rev()
                    .fold(0, |acc, digit| digit.unwrap() - acc)
            })
            .sum::<i32>()
            .to_string();
    }
}

fn get_histories(input: &str) -> Vec<Vec<Vec<i32>>> {
    let mut histories: Vec<Vec<Vec<i32>>> = Vec::new();

    for line in input.lines() {
        let mut sequences: Vec<Vec<i32>> = Vec::new();

        sequences.push(
            line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect(),
        );

        histories.push(sequences);
    }

    for history in &mut histories {
        let mut sequence = history[0].clone();

        loop {
            let mut new_sequence: Vec<i32> = Vec::new();
            let mut i = 1;

            while i < sequence.len() {
                let new_value = sequence[i] - sequence[i - 1];

                new_sequence.push(new_value);

                i += 1;
            }

            history.push(new_sequence.clone());

            if new_sequence.iter().all(|&x| x == 0) {
                break;
            }

            sequence = new_sequence.clone();
        }
    }

    return histories;
}
