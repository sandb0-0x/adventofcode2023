mod parse_file_error;

use parse_file_error::ParseFileError;
use std::{error, fs, str::FromStr};

const PART_ONE_DESCRIPTION: &str = "Part One Description";
const PART_TWO_DESCRIPTION: &str = "Part Two Description";

type Answer = usize;

struct PuzzleInput;

impl FromStr for PuzzleInput {
    type Err = ParseFileError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        !unimplemented!()
    }
}

fn part_one(puzzle_input: &PuzzleInput) -> Answer {
    !unimplemented!()
}

fn part_two(puzzle_input: &PuzzleInput) -> Answer {
    !unimplemented!()
}

fn main() {
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
    println!("Part One -- {PART_ONE_DESCRIPTION}, {part_one_answer}");

    let part_two_answer = part_two(&puzzle_input);
    println!("Part Two -- {PART_TWO_DESCRIPTION}, {part_two_answer}");
}
