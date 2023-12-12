use std::collections::HashMap;

use crate::day::Day;

pub struct Day1;

impl Day for Day1 {
    fn one(&self, input: &str) -> String {
        return get_sum(input, false);
    }

    fn two(&self, input: &str) -> String {
        return get_sum(input, true);
    }
}

fn get_sum(input: &str, check_words: bool) -> String {
    let mut sum = 0;

    for line in input.lines() {
        let line_length = line.len();
        let first_digit = find_digit(line, check_words, 0, line_length as i32);
        let last_digit = find_digit(line, check_words, (line_length as i32) - 1, -1);

        sum += format!("{}{}", first_digit, last_digit)
            .parse::<i32>()
            .unwrap();
    }

    return sum.to_string();
}

fn find_digit(line: &str, check_words: bool, start_value: i32, end_value: i32) -> char {
    let mut numbers_dictionary: HashMap<&str, char> = HashMap::new();
    let line_chars: Vec<char> = line.chars().collect();
    let mut i: i32 = start_value;
    let add_value = if start_value > end_value { -1 } else { 1 };

    if check_words {
        if add_value == 1 {
            numbers_dictionary.insert("one", '1');
            numbers_dictionary.insert("two", '2');
            numbers_dictionary.insert("three", '3');
            numbers_dictionary.insert("four", '4');
            numbers_dictionary.insert("five", '5');
            numbers_dictionary.insert("six", '6');
            numbers_dictionary.insert("seven", '7');
            numbers_dictionary.insert("eight", '8');
            numbers_dictionary.insert("nine", '9');
        } else {
            numbers_dictionary.insert("eno", '1');
            numbers_dictionary.insert("owt", '2');
            numbers_dictionary.insert("eerht", '3');
            numbers_dictionary.insert("ruof", '4');
            numbers_dictionary.insert("evif", '5');
            numbers_dictionary.insert("xis", '6');
            numbers_dictionary.insert("neves", '7');
            numbers_dictionary.insert("thgie", '8');
            numbers_dictionary.insert("enin", '9');
        }
    }

    while i != end_value {
        let c = line_chars[i as usize];

        if c.is_numeric() {
            return c;
        } else if check_words {
            for (key, value) in numbers_dictionary.iter() {
                let key_chars: Vec<char> = key.chars().collect();
                let mut j = i;
                let mut k = 0;
                let mut possible_match = true;

                while j != end_value && k < key_chars.len() {
                    if line_chars[j as usize] != key_chars[k] {
                        possible_match = false;
                        break;
                    }

                    j += add_value;
                    k += 1;
                }

                if possible_match && k == key_chars.len() {
                    return *value;
                }
            }
        }

        i += add_value;
    }

    return '0';
}
