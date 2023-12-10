use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

#[derive(Debug)]
enum NodeInstructionResult {
    ResultNode(String),
    Success(u64),
}

fn compute_instruction_result<F: Fn(&str) -> bool>(
    start_node: &str,
    end_condition: F,
    instructions: &str,
    network: &HashMap<String, Node>,
) -> NodeInstructionResult {
    let mut current_node = network
        .get(start_node)
        .expect("Start node not found in network: {start_node}");
    for (c, index) in instructions.chars().into_iter().zip(1..) {
        let new_node = match c {
            'L' => network
                .get(&current_node.left)
                .expect("Unable to follow {current_node} left"),
            'R' => network
                .get(&current_node.right)
                .expect("Unable to follow {current_node} right"),
            x => {
                panic!("Unable to read instruction: {x}");
            }
        };

        if end_condition(&new_node.label) {
            return NodeInstructionResult::Success(index);
        } else {
            current_node = new_node;
        }
    }

    NodeInstructionResult::ResultNode(current_node.label.clone())
}

fn part_one(instructions: &str, network: &HashMap<String, Node>) -> u64 {
    // First, compute each node's instruction result,
    // ie the result of following the given instructions starting at that node
    let node_instruction_results = network.keys().fold(HashMap::new(), |mut map, node| {
        map.insert(
            node,
            compute_instruction_result(node, |s| s == "ZZZ", instructions, network),
        );
        map
    });
    // println!("{node_instruction_results:?}");

    let instruction_length = instructions.len() as u64;
    let mut steps = 0u64;
    let mut current_node = network.get("AAA").expect("AAA node not found");

    loop {
        match node_instruction_results.get(&current_node.label) {
            Some(NodeInstructionResult::Success(step_count)) => {
                break steps + step_count;
            }
            Some(NodeInstructionResult::ResultNode(node)) => {
                steps += instruction_length;
                current_node = network
                    .get(node)
                    .expect("Unable to find node in network: {node}");
            }
            None => {
                panic!(
                    "Could not find note in instruction results: {}",
                    current_node.label
                );
            }
        }
    }
}

fn part_two(instructions: &str, network: &HashMap<String, Node>) -> u64 {
    // First, compute each node's instruction result,
    // ie the result of following the given instructions starting at that node
    let node_instruction_results = network.keys().fold(HashMap::new(), |mut map, node| {
        map.insert(
            node,
            compute_instruction_result(node, |s| s.ends_with("Z"), instructions, network),
        );
        map
    });
    // println!("{node_instruction_results:?}");

    let instruction_length = instructions.len() as u64;
    let start_nodes = network
        .keys()
        .filter(|s| s.ends_with("A"))
        .map(|s| network.get(s).ok_or("Unable to find node: {s}"))
        .collect::<Result<Vec<_>, _>>()
        .expect("Could not parse starting nodes");

    start_nodes.into_iter().map(|node| {
        let mut steps = 0u64;
        let mut current_node = node;
        loop {
            match node_instruction_results.get(&current_node.label) {
                Some(NodeInstructionResult::Success(step_count)) => {
                    break steps + step_count;
                },
                Some(NodeInstructionResult::ResultNode(result_node)) => {
                    steps += instruction_length;
                    current_node = network
                        .get(result_node)
                        .expect("Unable to find node in network: {result_node}");
                },
                None => {
                    panic!("Could not find node in instruction results: {}", current_node.label);
                }
            }
        }
    }).reduce(|lcm, n| num::integer::lcm(lcm, n)).expect("Empty starting node set")
}

fn parse_file_contents(
    file_contents: &str,
) -> Result<(&str, HashMap<String, Node>), Box<dyn std::error::Error>> {
    let mut lines = file_contents.lines();
    let instructions = lines.next().ok_or("No instructions")?;
    lines.next();

    let network_re = Regex::new(r"([0-9A-Z]{3}) = \(([0-9A-Z]{3}), ([0-9A-Z]{3})\)").unwrap();
    let network = lines
        .map(|line| {
            let network_capture = network_re
                .captures(line)
                .ok_or("Could not parse network definion")?;
            if let (Some(label), Some(left), Some(right)) = (
                network_capture.get(1),
                network_capture.get(2),
                network_capture.get(3),
            ) {
                Ok((
                    label.as_str().to_owned(),
                    Node {
                        label: label.as_str().to_owned(),
                        left: left.as_str().to_owned(),
                        right: right.as_str().to_owned(),
                    },
                ))
            } else {
                Err("Could not parse node: {line}")
            }
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    Ok((instructions, network))
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let (instructions, network) = match parse_file_contents(&file_contents) {
        Ok(result) => result,
        Err(error) => {
            println!("Unable to parse file contents: {error}");
            return;
        }
    };
    // println!("{instructions:?}");
    // println!("{network:?}");

    // let part_one_step_count = part_one(instructions, &network);
    // println!("Part One -- Step Count: {part_one_step_count}");

    let part_two_step_count = part_two(instructions, &network);
    println!("Part Two -- Step Count: {part_two_step_count}");
}
