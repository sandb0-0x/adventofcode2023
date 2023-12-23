#[allow(unused)]
mod parse_file_error;

use parse_file_error::ParseFileError;
#[allow(unused_imports)]
use std::{collections::HashMap, error, fs, str::FromStr};

#[allow(unused)]
const PART_ONE_DESCRIPTION: &str = "Total Load on North:";
#[allow(unused)]
const PART_TWO_DESCRIPTION: &str = "Part Two Description";

type Answer = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RockType {
    Rounded,
    Square,
    Empty,
}

#[derive(Debug)]
struct PuzzleInput {
    rock_grid: Vec<Vec<RockType>>,
}

impl FromStr for PuzzleInput {
    type Err = ParseFileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rock_grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'O' => Ok(RockType::Rounded),
                        '#' => Ok(RockType::Square),
                        '.' => Ok(RockType::Empty),
                        _ => Err(ParseFileError::from_str(format!("Invalid rock type: {c}"))),
                    })
                    .collect::<Result<Vec<RockType>, ParseFileError>>()
            })
            .collect::<Result<Vec<Vec<RockType>>, ParseFileError>>()?;

        Ok(PuzzleInput { rock_grid })
    }
}

fn pretty_print(rock_grid: &Vec<Vec<RockType>>) -> String {
    rock_grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|rock_type| match rock_type {
                    RockType::Rounded => 'O',
                    RockType::Square => '#',
                    RockType::Empty => '.',
                })
                .collect::<String>()
        })
        .reduce(|acc, e| acc + "\n" + &e)
        .unwrap_or(String::from(""))
}

#[allow(unused)]
fn part_one(puzzle_input: &PuzzleInput) -> Answer {
    let mut total_load = 0;

    for col in 0..puzzle_input.rock_grid[0].len() {
        let mut new_rows = Vec::new();
        let mut next_row = 0usize;
        for row in 0..puzzle_input.rock_grid.len() {
            match puzzle_input.rock_grid[row][col] {
                RockType::Rounded => {
                    new_rows.push(next_row);
                    next_row += 1;
                }
                RockType::Square => {
                    next_row = row + 1;
                }
                RockType::Empty => {}
            }
        }
        total_load += new_rows
            .into_iter()
            .map(|row| puzzle_input.rock_grid.len() - row)
            .sum::<usize>();
    }

    total_load
}

fn compute_load(rock_grid: &Vec<Vec<RockType>>) -> Answer {
    let row_count = rock_grid.len();
    rock_grid
        .iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.iter()
                .filter(|&&rock| rock == RockType::Rounded)
                .count()
                * (row_count - row_index)
        })
        .sum()
}

// Extracted from part_one for part_two:
fn tilt_north_and_rotate(rock_grid: &Vec<Vec<RockType>>) -> Vec<Vec<RockType>> {
    let mut new_grid = Vec::new();
    for col in 0..rock_grid[0].len() {
        let mut new_square_indices = Vec::new();
        let mut new_round_indices = Vec::new();
        let mut next_index = (rock_grid.len() - 1) as isize;
        for row in 0..rock_grid.len() {
            match rock_grid[row][col] {
                RockType::Rounded => {
                    new_round_indices.push(next_index as usize);
                    next_index -= 1;
                }
                RockType::Square => {
                    new_square_indices.push((rock_grid.len() - 1) - row);
                    next_index = (rock_grid.len() - 1) as isize - (row + 1) as isize;
                }
                RockType::Empty => {}
            }
        }
        let mut new_row = vec![RockType::Empty; rock_grid.len()];
        new_square_indices
            .into_iter()
            .for_each(|i| new_row[i] = RockType::Square);
        new_round_indices
            .into_iter()
            .for_each(|i| new_row[i] = RockType::Rounded);
        new_grid.push(new_row);
    }
    new_grid
}

#[allow(unused)]
fn part_two(puzzle_input: &PuzzleInput) -> Answer {
    let cycles = 1_000_000_000;
    let mut memo_map_str = HashMap::<String, String>::new();
    let mut rock_grid_str = pretty_print(&puzzle_input.rock_grid);
    for i in 0..cycles {
        if i % (cycles / 1000) == 0 {
            println!("i: {i}");
        }

        rock_grid_str = memo_map_str
            .entry(rock_grid_str.clone())
            .or_insert_with(|| {
                let rock_grid = rock_grid_str.parse::<PuzzleInput>().unwrap().rock_grid;
                let tilt_north = tilt_north_and_rotate(&rock_grid);
                let tilt_west = tilt_north_and_rotate(&tilt_north);
                let tilt_south = tilt_north_and_rotate(&tilt_west);
                let tilt_east = tilt_north_and_rotate(&tilt_south);
                pretty_print(&tilt_east)
            })
            .clone();
    }
    let rock_grid = rock_grid_str.parse::<PuzzleInput>().unwrap().rock_grid;

    compute_load(&rock_grid)
}

#[allow(unused)]
fn part_two_cycle_detection(puzzle_input: &PuzzleInput) -> Answer {
    let cycles = 1_000_000_000usize;
    // Todo: Use a bimap instead of two hashmaps, although it's the same underlying data
    let mut previous_grids = HashMap::<String, usize>::new();
    let mut previous_grids_by_index = HashMap::<usize, String>::new();
    let mut rock_grid_str = pretty_print(&puzzle_input.rock_grid);
    for i in 0..cycles {
        if i % (cycles / 1000) == 0 {
            println!("i: {i}");
        }

        if let Some(existing_index) = previous_grids.get(&rock_grid_str) {
            // Cycle detected, determine the grid at i=cycles
            let cycle_length = i - existing_index;
            let offset = (cycles - existing_index) % cycle_length;
            let final_grid_index = offset + existing_index;
            rock_grid_str = previous_grids_by_index
                .get(&final_grid_index)
                .unwrap()
                .clone();
            break;
        } else {
            let rock_grid = rock_grid_str.parse::<PuzzleInput>().unwrap().rock_grid;
            let tilt_north = tilt_north_and_rotate(&rock_grid);
            let tilt_west = tilt_north_and_rotate(&tilt_north);
            let tilt_south = tilt_north_and_rotate(&tilt_west);
            let tilt_east = tilt_north_and_rotate(&tilt_south);

            previous_grids.insert(rock_grid_str.clone(), i);
            previous_grids_by_index.insert(i, rock_grid_str);

            rock_grid_str = pretty_print(&tilt_east);
        }
    }

    let rock_grid = rock_grid_str.parse::<PuzzleInput>().unwrap().rock_grid;
    compute_load(&rock_grid)
}

fn main() {
    let start_time = std::time::SystemTime::now();
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let puzzle_input = match file_contents.parse::<PuzzleInput>() {
        Ok(puzzle_input) => puzzle_input,
        Err(err) => {
            println!("Error parsing file contents: {err}");
            return;
        }
    };

    let part_one_answer = part_one(&puzzle_input);
    println!("Part One -- {PART_ONE_DESCRIPTION}\n{part_one_answer}");

    let part_two_answer = part_two_cycle_detection(&puzzle_input);
    println!("Part Two -- {PART_TWO_DESCRIPTION}, {part_two_answer}");

    match start_time.elapsed() {
        Ok(elapsed) => {
            println!("Elapsed secs: {}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error recording time: {e:?}");
        }
    }
}
