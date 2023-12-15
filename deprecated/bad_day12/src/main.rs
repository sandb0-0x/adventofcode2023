use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt, fs, result,
};

#[derive(Debug)]
struct ParseFileError;

impl fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse file contents into puzzle input")
    }
}

impl Error for ParseFileError {}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq, Eq)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct SpringRow {
    springs: Vec<Condition>,
    damage_groups: Vec<usize>,
}

impl SpringRow {
    fn to_representation_string(&self) -> String {
        self.springs.iter().map(|cond| match cond {
            Condition::Damaged => '#',
            Condition::Operational => '.',
            Condition::Unknown => '?',
        }).collect()
    }
}

fn arrangement_str_to_damage_groups(arrangement_str: &str) -> Vec<usize> {
    let damaged_re = Regex::new(r"#+").unwrap();
    damaged_re
        .find_iter(&arrangement_str)
        .map(|s| s.len())
        .collect()
}

fn count_possible_arrangements_naive(spring_row: &SpringRow) -> usize {
    // There are a bunch of edge cases, so just try all options given a spring row
    let unknown_positions = spring_row
        .springs
        .iter()
        .enumerate()
        .filter(|(_, condition)| condition == &&Condition::Unknown)
        .map(|(unknown_postion, _)| unknown_postion)
        .enumerate()
        .map(|(unknown_counter, unknown_position)| (unknown_position, unknown_counter))
        .collect::<HashMap<usize, usize>>();

    let mut valid_arrangement_count: usize = 0;
    for option_binary in 0..2usize.pow(unknown_positions.len() as u32) {
        let arrangement_str = spring_row
            .springs
            .iter()
            .enumerate()
            .map(|(i, condition)| match condition {
                Condition::Damaged => '#',
                Condition::Operational => '.',
                Condition::Unknown => {
                    if option_binary & (1 << unknown_positions.get(&i).unwrap()) == 0 {
                        '#'
                    } else {
                        '.'
                    }
                }
            })
            .collect::<String>();
        if arrangement_str_to_damage_groups(&arrangement_str) == spring_row.damage_groups {
            valid_arrangement_count += 1;
        }
    }

    valid_arrangement_count
}

fn count_possible_arrangements_recursive(
    current_row: &[Condition],
    remaining_groups: Vec<usize>,
) -> usize {

    // println!("Current Row: {current_row:?}");
    // println!("Remaining Groups: {remaining_groups:?}");

    // Base cases
    let all_remaining_are_operational = current_row
        .iter()
        .all(|cond| cond == &Condition::Operational);
    let remaining_groups_are_empty = remaining_groups.iter().all(|group| group == &0);
    if all_remaining_are_operational && remaining_groups_are_empty {
        // println!("Returining 1");
        return 1;
    } else if all_remaining_are_operational {
        // println!("Returning 0");
        return 0;
    }

    match current_row[0] {
        Condition::Operational => count_possible_arrangements_recursive(
            &current_row[1..],
            if remaining_groups.len() > 0 && remaining_groups[0] == 0 {
                remaining_groups[1..].to_vec()
            } else {
                remaining_groups
            },
        ),
        Condition::Damaged => {
            if remaining_groups.is_empty() || remaining_groups[0] == 0 {
                0
            } else {
                let mut new_remaining_groups = remaining_groups.clone();
                *(new_remaining_groups.first_mut().unwrap()) -= 1;
                count_possible_arrangements_recursive(&current_row[1..], new_remaining_groups)
            }
        }
        Condition::Unknown => {
            let next_is_damaged = if remaining_groups.is_empty() || remaining_groups[0] == 0 {
                0
            } else {
                let mut new_remaining_groups = remaining_groups.clone();
                *new_remaining_groups.first_mut().unwrap() -= 1;
                count_possible_arrangements_recursive(&current_row[1..], new_remaining_groups)
            };
            let next_is_operational = count_possible_arrangements_recursive(
                &current_row[1..],
                if remaining_groups.len() > 0 && remaining_groups[0] == 0 {
                    remaining_groups[1..].to_vec()
                } else {
                    remaining_groups
                },
            );
            next_is_damaged + next_is_operational
        }
    }
}

fn count_possible_arrangements(spring_row: &SpringRow) -> usize {
    count_possible_arrangements_recursive(
        spring_row.springs.as_slice(),
        spring_row.damage_groups.clone(),
    )
}

fn part_one(spring_rows: &Vec<SpringRow>) -> usize {
    spring_rows
        .into_iter()
        // .map(|spring_row| count_possible_arrangements(spring_row))
        // .sum()
        .for_each(|spring_row| {
            println!("Spring row: {:?}", spring_row.to_representation_string());
            println!("Expectation: {:?}", spring_row.damage_groups);
            let num = count_possible_arrangements(spring_row);
            println!("Arrangements: {num}");
        });
    0
}

fn part_two() -> () {}

fn parse_file_contents(file_contents: &str) -> Result<Vec<SpringRow>> {
    let line_re = Regex::new(r"([#?.]+)\s+([0-9,]+)").unwrap();
    file_contents
        .lines()
        // .take(1)
        .map(|line| {
            let line_capture = line_re.captures(line);
            let springs = line_capture.as_ref().and_then(|cap| cap.get(1)).map_or(
                Err(ParseFileError.into()),
                |springs_match| {
                    springs_match
                        .as_str()
                        .chars()
                        .map(|c| match c {
                            '#' => Ok(Condition::Damaged),
                            '.' => Ok(Condition::Operational),
                            '?' => Ok(Condition::Unknown),
                            _ => Err(ParseFileError.into()),
                        })
                        .collect::<Result<Vec<Condition>>>()
                },
            );
            let damage_groups = line_capture.and_then(|cap| cap.get(2)).map_or(
                Err(ParseFileError.into()),
                |damage_groups_match| {
                    damage_groups_match
                        .as_str()
                        .split(',')
                        .map(|n| n.parse::<usize>().map_err(|e| e.into()))
                        .collect::<Result<Vec<usize>>>()
                },
            );
            if let (Ok(springs), Ok(damage_groups)) = (springs, damage_groups) {
                Ok(SpringRow {
                    springs,
                    damage_groups,
                })
            } else {
                Err(ParseFileError.into())
            }
        })
        .collect()
}

fn main() {
    let file_path = "sample_input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let Ok(spring_rows) = parse_file_contents(&file_contents) else {
        println!("Could not parse file contents");
        return;
    };

    let sum_of_possible_arrangements = part_one(&spring_rows);
    println!(
        "Part One -- Sum of Possible Row Arrangments: {}",
        sum_of_possible_arrangements
    );
}
