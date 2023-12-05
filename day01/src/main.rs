use std::collections::HashMap;
use std::fs;

fn calibration_value_part1(line: &str) -> u32 {
    let filtered: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();
    10 * filtered.first().unwrap() + filtered.last().unwrap()
}

// INCORRECT SOLUTION
// const REPLACEMENTS: [(&str, &str); 9] = [
//     ("one", "1"),
//     ("two", "2"),
//     ("three", "3"),
//     ("four", "4"),
//     ("five", "5"),
//     ("six", "6"),
//     ("seven", "7"),
//     ("eight", "8"),
//     ("nine", "9"),
// ];
// fn calibration_value_part2(input_line: &str) -> u32 {
//     let mut line = input_line.to_string();
//     for (spelled, digit) in REPLACEMENTS {
//         line = line.replace(spelled, digit);
//     }
//     let filtered: Vec<u32> = line.chars().filter_map(|char| char.to_digit(10)).collect();
//     println!("Line {input_line} produced filtered {filtered:?}");
//     10 * filtered.first().unwrap() + filtered.last().unwrap()
// }

const DIGIT_STRINGS: [(u32, &str, &str); 9] = [
    (1, "one", "1"),
    (2, "two", "2"),
    (3, "three", "3"),
    (4, "four", "4"),
    (5, "five", "5"),
    (6, "six", "6"),
    (7, "seven", "7"),
    (8, "eight", "8"),
    (9, "nine", "9"),
];

fn calibration_value_part2(line: &str) -> u32 {
    let (mut first_digit, mut last_digit): (u32, u32) = (0, 0);
    let (mut first_digit_index, mut last_digit_index): (usize, usize) = (line.chars().count(), 0);

    for (digit, spelled_str, digit_str) in DIGIT_STRINGS {
        let left_find = match (line.find(spelled_str), line.find(digit_str)) {
            (Some(spelled_index), Some(digit_index)) => {
                Some(std::cmp::min(spelled_index, digit_index))
            }
            (Some(spelled_index), None) => Some(spelled_index),
            (None, Some(digit_index)) => Some(digit_index),
            (None, None) => None,
        };

        match left_find {
            Some(left_index) if left_index <= first_digit_index => {
                first_digit = digit;
                first_digit_index = left_index;
            }
            _ => (),
        };

        let right_find = match (line.rfind(spelled_str), line.rfind(digit_str)) {
            (Some(spelled_index), Some(digit_index)) => {
                Some(std::cmp::max(spelled_index, digit_index))
            }
            (Some(spelled_index), None) => Some(spelled_index),
            (None, Some(digit_index)) => Some(digit_index),
            (None, None) => None,
        };

        match right_find {
            Some(right_index) if right_index >= last_digit_index => {
                last_digit = digit;
                last_digit_index = right_index;
            }
            _ => (),
        };
    }
    // println!("Line {line} produced first value {first_digit} and last digit {last_digit}");
    10 * first_digit + last_digit
}

fn main() {
    let file_path = "input.txt";
    // let file_path = "sample_input_pt2.txt";
    let contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    // let calibration_value_part1_sum: u32 = contents.lines().map(calibration_value_part1).sum();
    // println!(
    //     "Part One: calibration value sum: {}",
    //     calibration_value_part1_sum
    // );

    let calibration_value_part2_sum: u32 = contents.lines().map(calibration_value_part2).sum();
    println!(
        "Part Two: calibration value sum: {:?}",
        calibration_value_part2_sum
    );
}