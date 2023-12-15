use std::{error::Error, fmt, fs, result};

#[derive(Debug)]
struct ParseFileError;

impl fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse file contents properly")
    }
}

impl Error for ParseFileError {}

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct RockPattern {
    row_labels: Vec<u64>,
    col_labels: Vec<u64>,
}

fn reflection_from_labels<F>(labels: &Vec<u64>, comparison_fn: F) -> Option<usize>
where
    F: Fn(Vec<&u64>, Vec<&u64>) -> bool,
{
    // println!("Checking Labels: {labels:?}");
    (1..labels.len()).find(|&i| {
        let left = &labels[..i];
        let right = &labels[i..];
        let size = usize::min(left.len(), right.len());

        let left_compare = left.into_iter().rev().take(size).collect::<Vec<&u64>>();
        let right_compare = right.into_iter().take(size).collect::<Vec<&u64>>();

        // println!("At {i}, left gave {left_compare:?}, right gave {right_compare:?}");
        comparison_fn(left_compare, right_compare)
    })
}

fn part_one(rock_patterns: &Vec<RockPattern>) -> usize {
    rock_patterns
        .iter()
        .map(|pattern| {
            // println!("Checking pattern: {pattern:?}");
            let row_mirror =
                reflection_from_labels(&pattern.row_labels, |left, right| left == right);
            let col_mirror =
                reflection_from_labels(&pattern.col_labels, |left, right| left == right);
            match (row_mirror, col_mirror) {
                (Some(r), _) => 100 * r,
                (_, Some(c)) => c,
                _ => unreachable!("No reflection found"),
            }
        })
        .sum()
}

fn compute_bit_differences(val_a: u64, val_b: u64) -> u64 {
    (0..64)
        .map(|n| {
            let bit_a = (val_a >> n) & 1;
            let bit_b = (val_b >> n) & 1;
            if bit_a == bit_b {
                0
            } else {
                1
            }
        })
        .sum()
}

// Return true if exactly one pair of numbers if off by one bit (but all others are equal)
// This means that the _total_ number of bit difference when comparing element by element
// is exactly one
fn compare_reflections_with_smudge(left: Vec<&u64>, right: Vec<&u64>) -> bool {
    left.iter()
        .zip(right.iter())
        .map(|(left_val, right_val)| compute_bit_differences(**left_val, **right_val))
        .sum::<u64>()
        == 1u64
}

fn part_two(rock_patterns: &Vec<RockPattern>) -> usize {
    rock_patterns
        .iter()
        .map(|pattern| {
            // println!("Checking pattern: {pattern:?}");
            let row_mirror = reflection_from_labels(&pattern.row_labels, |left, right| {
                compare_reflections_with_smudge(left, right)
            });
            let col_mirror = reflection_from_labels(&pattern.col_labels, |left, right| {
                compare_reflections_with_smudge(left, right)
            });
            match (row_mirror, col_mirror) {
                (Some(r), _) => 100 * r,
                (_, Some(c)) => c,
                _ => unreachable!("No reflection found"),
            }
        })
        .sum()
}

fn convert_line_to_label(line: &Vec<char>) -> Result<u64> {
    line.into_iter()
        .map(|c| match c {
            '#' => Ok('1'),
            '.' => Ok('0'),
            _ => Err(ParseFileError.into()),
        })
        .collect::<Result<String>>()
        .and_then(|str| u64::from_str_radix(str.as_str(), 2).map_err(|e| e.into()))
}

fn convert_rock_grid_to_labels(grid: &str) -> Result<RockPattern> {
    let grid_cells = grid
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let row_labels = grid_cells
        .iter()
        .map(convert_line_to_label)
        .collect::<Result<Vec<u64>>>();

    let rows = grid_cells.len();
    let cols = grid_cells[0].len();
    let col_labels = (0..cols)
        .map(|col| {
            (0..rows)
                .map(|row| grid_cells[row][col])
                .collect::<Vec<char>>()
        })
        .map(|line| convert_line_to_label(&line))
        .collect::<Result<Vec<u64>>>();

    match (row_labels, col_labels) {
        (Ok(row_labels), Ok(col_labels)) => Ok(RockPattern {
            row_labels,
            col_labels,
        }),
        (Err(e), _) | (_, Err(e)) => Err(e.into()),
    }
}

fn parse_file_contents(file_contents: &str) -> Result<Vec<RockPattern>> {
    file_contents
        .split("\r\n\r\n")
        .map(convert_rock_grid_to_labels)
        .collect()
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let rock_patterns = match parse_file_contents(&file_contents) {
        Ok(rock_patterns) => rock_patterns,
        Err(err) => {
            println!("Error parsing file contents: {err}");
            return;
        }
    };

    let sum_of_mirror_notes = part_one(&rock_patterns);
    println!("Part One -- Sum of mirror notes: {sum_of_mirror_notes}");

    let sum_of_smudged_mirror_notes = part_two(&rock_patterns);
    println!("Part Two -- Sum of smudged mirror notes: {sum_of_smudged_mirror_notes}");
}
