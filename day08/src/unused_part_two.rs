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

#[derive(Debug)]
struct NodeInstructionResultForGhosts {
    result_node: String,
    success_step_counts: HashSet<u64>,
}

fn compute_instruction_result_part_two(
    start_node: &str,
    instructions: &str,
    network: &HashMap<String, Node>,
) -> NodeInstructionResultForGhosts {
    let mut current_node = network
        .get(start_node)
        .expect("Start node not found in network: {start_node}");
    let mut success_step_counts = HashSet::<u64>::new();

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

        if new_node.label.ends_with("Z") {
            success_step_counts.insert(index);
        }
        current_node = new_node;
    }

    NodeInstructionResultForGhosts {
        result_node: current_node.label.clone(),
        success_step_counts: success_step_counts,
    }
}

fn part_two(instructions: &str, network: &HashMap<String, Node>) -> u64 {
    // First, compute each node's instruction result,
    // ie the result of following the given instructions starting at that node
    let node_instruction_results = network.keys().fold(HashMap::new(), |mut map, node| {
        map.insert(
            node,
            compute_instruction_result_part_two(node, instructions, network),
        );
        map
    });
    // println!("{node_instruction_results:?}");

    let instruction_length = instructions.len() as u64;
    let mut steps = 0u64;
    let mut current_nodes = network
        .keys()
        .filter(|s| s.ends_with("A"))
        .map(|s| network.get(s).ok_or("Unable to find node: {s}"))
        .collect::<Result<Vec<_>, _>>()
        .expect("Could not parse starting nodes");
    // println!("{starting_nodes:?}");

    loop {
        let instruction_results = current_nodes
            .iter()
            .map(|node| {
                node_instruction_results
                    .get(&node.label)
                    .expect("Unable to find node: {node:?}")
            })
            .collect::<Vec<_>>();
        let success_step_intersection = instruction_results.iter().fold(
            (0..instruction_length+1)
                .into_iter()
                .collect::<HashSet<u64>>(),
            |set, result| {
                set.intersection(&result.success_step_counts)
                    .map(|n| *n)
                    .collect()
            },
        );
        if success_step_intersection.is_empty() {
            steps += instruction_length;
            println!("Taken Steps: {steps}");
            if steps > 1_000_000_000_000_000 { break 0; }
            current_nodes = instruction_results
                .into_iter()
                .map(|result| {
                    network
                        .get(&result.result_node)
                        .expect("Unable to find node: {result.result_node}")
                })
                .collect();
        } else {
            break steps + success_step_intersection.iter().min().unwrap();
        }
    }
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

    let part_two_step_count = part_two(instructions, &network);
    println!("Part Two -- Step Count: {part_two_step_count}");
}