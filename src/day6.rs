use crate::day::Day;

pub struct Day6;

impl Day for Day6 {
    fn one(&self, input: &str) -> String {
        let numbers: Vec<Vec<u64>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|x| x.parse().ok())
                    .collect()
            })
            .collect();

        return compute(numbers);
    }

    fn two(&self, input: &str) -> String {
        let numbers: Vec<u64> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .skip(1)
                    .flat_map(|x| x.chars().filter_map(|c| c.to_digit(10)))
                    .map(u64::from)
                    .fold(0, |acc, digit| acc * 10 + digit)
            })
            .collect();

        return compute(vec![vec![numbers[0]], vec![numbers[1]]]);
    }
}

fn compute(numbers: Vec<Vec<u64>>) -> String {
    let mut i = 0;
    let mut possibilities: Vec<u64> = Vec::new();
    while i < numbers[0].len() {
        let time = numbers[0][i];
        let distance = numbers[1][i];
        let mut k = 0;
        let mut number_winning_conditions = 0;

        while k < time {
            let speed = k;
            let distance_travelled = speed * (time - k);

            if distance_travelled > distance {
                number_winning_conditions += 1;
            } else {
                if number_winning_conditions > 0 {
                    break;
                }
            }

            k += 1;
        }

        i += 1;
        possibilities.push(number_winning_conditions);
    }

    return possibilities.iter().fold(1, |acc, &x| acc * x).to_string();
}
