use std::{collections::HashMap, error::Error, fmt, fs, result};

#[derive(Debug)]
struct ParseFileError;

impl fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse file contents into puzzle input")
    }
}

impl Error for ParseFileError {}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct SpringRecord {
    springs: Vec<char>,
    groups: Vec<usize>,
}

fn arrangements_if_operational(
    record: &SpringRecord,
    index: usize,
    current_group_index: usize,
    current_group_size: usize,
) -> usize {
    if current_group_size > 0 && record.groups[current_group_index] != current_group_size {
        0
    } else if current_group_size > 0 {
        count_possible_arrangements_recursive(record, index + 1, current_group_index + 1, 0)
    } else {
        count_possible_arrangements_recursive(
            record,
            index + 1,
            current_group_index,
            current_group_size,
        )
    }
}

fn arrangements_if_damaged(
    record: &SpringRecord,
    index: usize,
    current_group_index: usize,
    current_group_size: usize,
) -> usize {
    if current_group_size >= record.groups[current_group_index] {
        0
    } else {
        count_possible_arrangements_recursive(
            record,
            index + 1,
            current_group_index,
            current_group_size + 1,
        )
    }
}

fn count_possible_arrangements_recursive(
    record: &SpringRecord,
    index: usize,
    current_group_index: usize,
    current_group_size: usize,
) -> usize {
    if current_group_index >= record.groups.len() {
        // This is fine, so long as no remaining springs are damaged
        return if record.springs[index..].contains(&'#') {
            0
        } else {
            1
        };
    }

    if index >= record.springs.len() {
        return if current_group_index == record.groups.len() - 1
            && current_group_size == record.groups[current_group_index]
        {
            1
        } else {
            0
        };
    }

    match record.springs[index] {
        '.' => arrangements_if_operational(record, index, current_group_index, current_group_size),
        '#' => arrangements_if_damaged(record, index, current_group_index, current_group_size),
        '?' => {
            let damaged =
                arrangements_if_damaged(record, index, current_group_index, current_group_size);
            let operational =
                arrangements_if_operational(record, index, current_group_index, current_group_size);
            damaged + operational
        }
        c @ _ => panic!("Encountered unexpected character: {c}"),
    }
}

fn count_possible_arrangements(record: &SpringRecord) -> usize {
    count_possible_arrangements_memoized(record, 0, 0, 0, &mut HashMap::new())
}

fn arrangements_if_operational_memoized(
    record: &SpringRecord,
    index: usize,
    current_group_index: usize,
    current_group_size: usize,
    memoized: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if current_group_size > 0 && record.groups[current_group_index] != current_group_size {
        0
    } else if current_group_size > 0 {
        count_possible_arrangements_memoized(
            record,
            index + 1,
            current_group_index + 1,
            0,
            memoized,
        )
    } else {
        count_possible_arrangements_memoized(
            record,
            index + 1,
            current_group_index,
            current_group_size,
            memoized,
        )
    }
}

fn arrangements_if_damaged_memoized(
    record: &SpringRecord,
    index: usize,
    current_group_index: usize,
    current_group_size: usize,
    memoized: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if current_group_size >= record.groups[current_group_index] {
        0
    } else {
        count_possible_arrangements_memoized(
            record,
            index + 1,
            current_group_index,
            current_group_size + 1,
            memoized,
        )
    }
}
fn count_possible_arrangements_memoized(
    record: &SpringRecord,
    index: usize,
    current_group_index: usize,
    current_group_size: usize,
    memoized: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(count) = memoized.get(&(index, current_group_index, current_group_size)) {
        return *count;
    }
    let count = {
        if current_group_index >= record.groups.len() {
            // This is fine, so long as no remaining springs are damaged
            return if record.springs[index..].contains(&'#') {
                0
            } else {
                1
            };
        }

        if index >= record.springs.len() {
            return if current_group_index == record.groups.len() - 1
                && current_group_size == record.groups[current_group_index]
            {
                1
            } else {
                0
            };
        }

        match record.springs[index] {
            '.' => arrangements_if_operational_memoized(
                record,
                index,
                current_group_index,
                current_group_size,
                memoized,
            ),
            '#' => arrangements_if_damaged_memoized(
                record,
                index,
                current_group_index,
                current_group_size,
                memoized,
            ),
            '?' => {
                let damaged = arrangements_if_damaged_memoized(
                    record,
                    index,
                    current_group_index,
                    current_group_size,
                    memoized,
                );
                let operational = arrangements_if_operational_memoized(
                    record,
                    index,
                    current_group_index,
                    current_group_size,
                    memoized,
                );
                damaged + operational
            }
            c @ _ => unreachable!("Encountered unexpected character: {c}"),
        }
    };
    memoized.insert((index, current_group_index, current_group_size), count);
    count
}

fn part_one(spring_records: &Vec<SpringRecord>) -> usize {
    spring_records
        .iter()
        .map(|record| count_possible_arrangements(record))
        .sum()
}

fn part_two(spring_records: &Vec<SpringRecord>) -> usize {
    let unfolded_spring_records = spring_records.iter().map(|record| {
        let mut springs = (record.springs.iter().collect::<String>() + "?").repeat(5);
        springs.truncate(springs.len() - 1);
        let groups = record.groups.repeat(5);
        SpringRecord {
            springs: springs.chars().collect(),
            groups,
        }
    });

    unfolded_spring_records
        .map(|record| count_possible_arrangements(&record))
        .sum()
}

fn parse_file_contents(file_contents: &str) -> Result<Vec<SpringRecord>> {
    file_contents
        .lines()
        .map(|line| {
            let mut split_line = line.split_whitespace();
            let springs = split_line.next().map(|s| s.chars().collect());
            let groups = split_line.next().and_then(|group_input| {
                group_input
                    .split(',')
                    .map(|n| n.parse::<usize>().ok())
                    .collect::<Option<Vec<_>>>()
            });
            if let (Some(springs), Some(groups)) = (springs, groups) {
                Ok(SpringRecord { springs, groups })
            } else {
                Err(ParseFileError.into())
            }
        })
        .collect()
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let Ok(spring_records) = parse_file_contents(&file_contents) else {
        println!("Could not parse file contents");
        return;
    };

    let sum_of_possible_arrangements = part_one(&spring_records);
    println!(
        "Part One -- Sum of Possible Row Arrangments: {}",
        sum_of_possible_arrangements
    );

    let sum_of_possible_arrangements_unfolded = part_two(&spring_records);
    println!(
        "Part Two -- Sum of Possible Row Arrangments Unfolded: {}",
        sum_of_possible_arrangements_unfolded
    );
}
