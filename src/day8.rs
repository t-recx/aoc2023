use std::collections::HashMap;

use crate::day::Day;

pub struct Day8;

struct Instructions {
	directions: Vec<usize>,
	nodes: HashMap<String, Vec<String>>
}

impl Day for Day8 {
	fn one(&self, input: &str) -> String {
		let instructions = get_instructions(input);
		let mut node = &instructions.nodes["AAA"];
		let mut step = 0;

		loop {
			let node_id = node[instructions.directions[step % instructions.directions.len()]].to_string();

			step += 1;

			if node_id == "ZZZ" {
				break;
			}

			node = &instructions.nodes[&node_id];
		}

		return step.to_string();
	}

	fn two(&self, input: &str) -> String {
		let instructions = get_instructions(input);
		let mut nodes_ids: Vec<String> = instructions.nodes.keys().into_iter().filter(|x| x.ends_with('A')).map(|x| x.to_string()).collect();
		let mut step = 0;

		loop {
			let mut new_nodes_ids: Vec<String> = Vec::new();

			for node_id in nodes_ids {
				let node = &instructions.nodes[&node_id];
				let new_node_id = node[instructions.directions[step % instructions.directions.len()]].to_string();

				new_nodes_ids.push(new_node_id);
			}

			step += 1;

			if new_nodes_ids.iter().all(|x| x.ends_with('Z')) {
				break;
			}

			nodes_ids = new_nodes_ids.clone();
		}

		return step.to_string();
	}
}

fn get_instructions(input: &str) -> Instructions {
	let mut i = 0;
	let mut directions: Vec<usize> = Vec::new();
	let mut nodes: HashMap<String, Vec<String>> = HashMap::new();
	let streamlined_input: String = input.chars().filter(|&x| x != '(' && x != ')' && x !=' ' ).collect();

	for line in streamlined_input.lines() {
		if i == 0 {
			directions = line.chars().map(|x| if x == 'L' { 0 } else { 1 }).collect::<Vec<usize>>();
		}
		else if i > 1 {
			let tokens: Vec<String> = line.split('=').map(|x| x.to_string()).collect();
			let tokens_nodes: Vec<String> = tokens[1].split(',').map(|x| x.to_string()).collect();
			nodes.insert(tokens[0].clone(), vec![tokens_nodes[0].clone(), tokens_nodes[1].clone()]);
		}

		i+=1;
	}

	return Instructions {
		directions,
		nodes
	};
}