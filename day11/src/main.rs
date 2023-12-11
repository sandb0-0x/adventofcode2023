use itertools::Itertools;
use std::{collections::HashSet, error::Error, fmt, fs, result};

type Result<T> = result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct FileParseError;

impl fmt::Display for FileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Could not parse file contents properly")
    }
}

impl Error for FileParseError {}

fn count_vec_in_range(vec: &Vec<usize>, lower_bound: usize, upper_bound: usize) -> usize {
    vec.partition_point(|x| x < &upper_bound) - vec.partition_point(|x| x <= &lower_bound)
}

#[derive(Debug)]
struct UniverseImage {
    num_rows: usize,
    num_cols: usize,
    galaxy_locations: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl UniverseImage {
    fn galaxy_distance(
        &self,
        galaxy_a: (usize, usize),
        galaxy_b: (usize, usize),
        expansion_factor: usize,
    ) -> usize {
        galaxy_a.0.abs_diff(galaxy_b.0)
            + galaxy_a.1.abs_diff(galaxy_b.1)
            + (expansion_factor - 1)
                * count_vec_in_range(
                    &self.empty_rows,
                    usize::min(galaxy_a.0, galaxy_b.0),
                    usize::max(galaxy_a.0, galaxy_b.0),
                )
            + (expansion_factor - 1)
                * count_vec_in_range(
                    &self.empty_cols,
                    usize::min(galaxy_a.1, galaxy_b.1),
                    usize::max(galaxy_a.1, galaxy_b.1),
                )
    }
}

fn part_one(universe_image: &UniverseImage) -> usize {
    println!("{universe_image:?}");
    universe_image
        .galaxy_locations
        .iter()
        .cartesian_product(universe_image.galaxy_locations.iter())
        .map(|(&galaxy_a, &galaxy_b)| universe_image.galaxy_distance(galaxy_a, galaxy_b, 1))
        .sum::<usize>()
        / 2
}

fn part_two(universe_image: &UniverseImage) -> usize {
    println!("{universe_image:?}");
    universe_image
        .galaxy_locations
        .iter()
        .cartesian_product(universe_image.galaxy_locations.iter())
        .map(|(&galaxy_a, &galaxy_b)| universe_image.galaxy_distance(galaxy_a, galaxy_b, 1_000_000))
        .sum::<usize>()
        / 2
}

fn parse_file_contents(file_contents: &str) -> Result<UniverseImage> {
    let file_chars = file_contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let num_rows = file_chars.len();
    let num_cols = file_chars
        .iter()
        .map(|line| line.len())
        .max()
        .ok_or(FileParseError)?;

    let galaxy_locations = file_chars
        .into_iter()
        .zip(0usize..)
        .map(|(line_chars, row)| {
            line_chars
                .into_iter()
                .zip(0usize..)
                .map(|(char, col)| match char {
                    '#' => Ok(Some((row, col))),
                    '.' => Ok(None),
                    _ => Err(FileParseError),
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter_map(result::Result::transpose)
        .collect::<result::Result<Vec<_>, _>>()?;

    let mut empty_rows = galaxy_locations
        .iter()
        .map(|(row, _col)| *row)
        .collect::<HashSet<usize>>()
        .symmetric_difference(&HashSet::from_iter(0..num_rows))
        .copied()
        .collect::<Vec<_>>();
    empty_rows.sort();

    let mut empty_cols = galaxy_locations
        .iter()
        .map(|(_row, col)| *col)
        .collect::<HashSet<usize>>()
        .symmetric_difference(&HashSet::from_iter(0..num_cols))
        .copied()
        .collect::<Vec<_>>();
    empty_cols.sort();

    Ok(UniverseImage {
        num_rows,
        num_cols,
        galaxy_locations,
        empty_rows,
        empty_cols,
    })
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let universe_image = match parse_file_contents(&file_contents) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Could not parse file contents: {err}");
            return;
        }
    };

    let sum_of_galaxy_distances = part_one(&universe_image);
    println!("Part One -- Sum of Galaxy Distances: {sum_of_galaxy_distances}");

    let sum_of_galaxy_distances_expanded = part_two(&universe_image);
    println!("Part One -- Sum of Galaxy Distances: {sum_of_galaxy_distances_expanded}");
}
