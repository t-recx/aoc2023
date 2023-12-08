use std::collections::HashMap;
use std::cmp::Ordering;

use crate::day::Day;

pub struct Day7;

struct Hand {
	cards: Vec<char>,
	hand_type: i32,
	bid: i32
}

impl Day for Day7 {
	fn one(&self, input: &str) -> String {
		return compute(input, get_number_of_cards_by_type_part_one, compare_hands_part_one);
	}

	fn two(&self, input: &str) -> String {
		return compute(input, get_number_of_cards_by_type_part_two, compare_hands_part_two);
	}
}

fn compute(input: &str, number_of_cards_by_type_fn: fn(&Vec<char>) -> HashMap<char, i32>, compare_hands_fn: fn(a: &Hand, b: &Hand) -> Ordering) -> String {
	let mut hands = get_hands(input, number_of_cards_by_type_fn);
	let mut result = 0;
	let mut i: i32 = 0;

	hands.sort_by(compare_hands_fn);

	for hand in hands {
		result += hand.bid * (i + 1);
		i+=1;
	}

	return result.to_string();
}

fn get_hands(input: &str, number_of_cards_by_type_fn: fn(&Vec<char>) -> HashMap<char, i32>) -> Vec<Hand> {
	let mut hands: Vec<Hand> = Vec::new();

	for line in input.lines() {
		let tokens: Vec<&str> = line.split_whitespace().collect();
		let bid = tokens[1].parse::<i32>().unwrap();
		let cards: Vec<char> = tokens[0].chars().collect();
		let number_of_cards_by_type = number_of_cards_by_type_fn(&cards);
		let hand_type = get_type(number_of_cards_by_type);
		hands.push(Hand {
			cards: cards,
			hand_type: hand_type,
			bid: bid
		});
	}

	return hands;
}

fn get_number_of_cards_by_type_part_one(chars: &Vec<char>) -> HashMap<char, i32> {
    let mut char_counts = HashMap::new();

	for &c in chars {
		if char_counts.contains_key(&c) {
			char_counts.entry(c).and_modify(|v| *v += 1);
		}
		else {
			char_counts.insert(c, 1);
		}
	}

    return char_counts;
}

fn get_number_of_cards_by_type_part_two(chars: &Vec<char>) -> HashMap<char, i32> {
    let char_counts = get_number_of_cards_by_type_part_one(&chars);
	let card_values = get_card_values_part_two();

	let mut jokerless_card_counts: HashMap<char, i32> = char_counts
	 	.clone()
		.into_iter()
		.filter(|&(key, _)| key != 'J')
		.collect();
	let jokers_count: usize = chars.len() - jokerless_card_counts.values().cloned().sum::<i32>() as usize;

	if jokers_count > 0 {
		if jokers_count == 5 {
			jokerless_card_counts.insert('A', 5);
		}
		else {
			let hand_type = get_type(jokerless_card_counts.clone());

			if hand_type == 7 || hand_type == 6 {
				jokerless_card_counts.entry(*jokerless_card_counts.iter().next().unwrap().0).and_modify(|v| *v += jokers_count as i32);
			}	
			else if hand_type == 4 {
				let key = jokerless_card_counts.iter().find(|&(_, v)| *v == 3).unwrap().0;
				jokerless_card_counts.entry(*key).and_modify(|v| *v += jokers_count as i32);
			}
			else if hand_type == 2 {
				let key = jokerless_card_counts.iter().find(|&(_, v)| *v == 2).unwrap().0;
				jokerless_card_counts.entry(*key).and_modify(|v| *v += jokers_count as i32);
			}
			else if hand_type == 0 || hand_type == 3 || hand_type == 5{
				let mut card_counts_vec: Vec<_> = jokerless_card_counts.clone().into_iter().collect();

				card_counts_vec.sort_by(|(k1, _), (k2, _)| card_values[k2].cmp(&card_values[k1]));

				jokerless_card_counts.entry(card_counts_vec[0].0).and_modify(|v| *v += jokers_count as i32);
			}
		}

		return jokerless_card_counts;
	}

    return char_counts;
}

fn compare_hands_part_one(a: &Hand, b: &Hand) -> Ordering {
	return compare_hands(a, b, get_card_values_part_one);
}

fn compare_hands_part_two(a: &Hand, b: &Hand) -> Ordering {
	return compare_hands(a, b, get_card_values_part_two);
}

fn compare_hands(a: &Hand, b: &Hand, card_values_fn: fn() -> HashMap<char, i32>) -> Ordering {
	if a.hand_type > b.hand_type {
		return Ordering::Greater;
	}
	else if a.hand_type < b.hand_type {
		return Ordering::Less;
	}
	else {
		let mut i = 0;
		let card_values = card_values_fn();
		
		while i < a.cards.len() {
			let card_value_a = card_values[&a.cards[i]];
			let card_value_b = card_values[&b.cards[i]];

			if card_value_a > card_value_b {
				return Ordering::Greater;
			}
			else if card_value_a < card_value_b {
				return Ordering::Less;
			}

			i+=1;
		}
	}

	return Ordering::Equal;
}

fn get_type(number_of_cards_by_type: HashMap<char, i32>) -> i32 {
	if number_of_cards_by_type.len() == 1 {
		return 7;
	}
	else if number_of_cards_by_type.values().any(|&value| value == 4) {
		return 6;
	}
	else if number_of_cards_by_type.values().filter(|&&value| value == 3).count() == 1 && number_of_cards_by_type.values().filter(|&&value| value == 2).count() == 1 {
		return 5;
	}
	else if number_of_cards_by_type.values().filter(|&&value| value == 3).count() == 1 {
		return 4;
	}
	else if number_of_cards_by_type.values().filter(|&&value| value == 2).count() == 2 {
		return 3;
	}
	else if number_of_cards_by_type.values().filter(|&&value| value == 2).count() == 1 {
		return 2;
	}
	else if number_of_cards_by_type.len() == 5 {
		return 1;
	}

	return 0;
}

fn get_card_values(j_value: i32) -> HashMap<char, i32> {
	let card_values: HashMap<char, i32> = vec![
			('A', 14), ('K', 13), ('Q', 12), ('J', j_value), ('T', 10),
			('9', 9), ('8', 8), ('7', 7), ('6', 6), ('5', 5),
			('4', 4), ('3', 3), ('2', 2)
		]
		.into_iter()
		.collect();

	return card_values;
}

fn get_card_values_part_one() -> HashMap<char, i32> {
	return get_card_values(11);
}

fn get_card_values_part_two() -> HashMap<char, i32> {
	return get_card_values(1);
}