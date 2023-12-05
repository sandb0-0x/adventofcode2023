use regex::Regex;
use std::fs;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

fn process_cubes(cubes: &str) -> CubeSet {
    let red_re = Regex::new(r"(\d+) red").unwrap();
    let red_count: u32 = if let Some(red_cap) = red_re.captures(cubes) {
        red_cap.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    };

    let green_re = Regex::new(r"(\d+) green").unwrap();
    let green_count: u32 = if let Some(green_cap) = green_re.captures(cubes) {
        green_cap.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    };

    let blue_re = Regex::new(r"(\d+) blue").unwrap();
    let blue_count: u32 = if let Some(blue_cap) = blue_re.captures(cubes) {
        blue_cap.get(1).unwrap().as_str().parse().unwrap()
    } else {
        0
    };

    CubeSet {
        red: red_count,
        green: green_count,
        blue: blue_count,
    }
}

fn process_game(input_line: &str) -> (u32, Vec<CubeSet>) {
    let re = Regex::new(r"Game (\d+): ([0-9a-z, ;]+)").unwrap();
    let line_cap = re.captures(input_line).unwrap();
    let game_index: u32 = line_cap.get(1).unwrap().as_str().parse().unwrap();
    // println!("{game_index}");
    let game_data = line_cap.get(2).unwrap().as_str().split("; ");
    (game_index, game_data.map(process_cubes).collect())
}

fn part_one(games: &Vec<(u32, Vec<CubeSet>)>) -> u32 {
    let possible_games = games.iter().filter(|(_, cube_set_vec)| {
        cube_set_vec
            .into_iter()
            .all(|cube_set| cube_set.red <= 12 && cube_set.green <= 13 && cube_set.blue <= 14)
    });
    possible_games.map(|(game_index, _)| game_index).sum()
}

fn compute_minimal_cubeset(cube_set_vec: &Vec<CubeSet>) -> CubeSet {
    CubeSet {
        red: cube_set_vec
            .iter()
            .map(|cube_set| cube_set.red)
            .max()
            .unwrap(),
        green: cube_set_vec
            .iter()
            .map(|cube_set| cube_set.green)
            .max()
            .unwrap(),
        blue: cube_set_vec
            .iter()
            .map(|cube_set| cube_set.blue)
            .max()
            .unwrap(),
    }
}

fn part_two(games: &Vec<(u32, Vec<CubeSet>)>) -> u32 {
    games
        .iter()
        .map(|(_, cube_set_vec)| {
            let minimal_cubeset = compute_minimal_cubeset(cube_set_vec);
            minimal_cubeset.red * minimal_cubeset.green * minimal_cubeset.blue
        })
        .sum()
}

fn main() {
    let file_path = "input.txt";
    // let file_path = "sample_input_pt2.txt";
    let contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    let games: Vec<(u32, Vec<CubeSet>)> = contents.lines().map(process_game).collect();

    println!("Part One -- Possible Index Sum: {}", part_one(&games));
    println!("Part Two -- Power Sum of Minimal Sets: {}", part_two(&games));
}
