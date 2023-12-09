use std::fs;

fn count_record_strategies(time: u64, distance: u64) -> u64 {
    let record_cutoff: u64 = Vec::from_iter(0..time / 2)
        .as_slice()
        .partition_point(|&t| t * (time - t) <= distance)
        .try_into()
        .expect("Could not calculate record cutoff");

    time - 2 * record_cutoff + 1
}

fn part_one(file_contents: &str) -> u64 {
    let mut file_lines = file_contents.lines();

    let times_iter = file_lines
        .next()
        .expect("Unable to read line")
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok());
    let dist_iter = file_lines
        .next()
        .expect("Unable to read line")
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok());

    let races_iter = times_iter.zip(dist_iter);
    races_iter.map(|(t, d)| count_record_strategies(t, d)).product()
}

fn part_two(file_contents: &str) -> u64 {
    let mut file_lines = file_contents.lines();
    
    let time = file_lines
        .next()
        .expect("Unable to read line")
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u64>().expect("Unable to parse line into time value");
    let dist = file_lines
        .next()
        .expect("Unable to read line")
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<u64>().expect("Unable to parse line into dist value");

    count_record_strategies(time, dist)
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    
    let part_one_record_product = part_one(&file_contents);
    println!("Part One -- Product of Record Stragies: {}", part_one_record_product);

    let part_two_record = part_two(&file_contents);
    println!("Part Two -- Record Strategies: {}", part_two_record);
}
