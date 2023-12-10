use regex::Regex;
use std::fs;

fn compute_next_value(history: &Vec<i32>) -> i32 {
    let mut successive_differences = Vec::new();
    successive_differences.push(history.clone());
    while !successive_differences
        .last()
        .unwrap()
        .iter()
        .all(|&x| x == 0)
    {
        successive_differences.push(
            successive_differences
                .last()
                .unwrap()
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<i32>>(),
        );
    }
    successive_differences
        .into_iter()
        .fold(0, |acc, diff| acc + diff.last().unwrap_or(&0))
}

fn compute_new_first_value(history: &Vec<i32>) -> i32 {
    let mut successive_differences = Vec::new();
    successive_differences.push(history.clone());
    while !successive_differences
        .last()
        .unwrap()
        .iter()
        .all(|&x| x == 0)
    {
        successive_differences.push(
            successive_differences
                .last()
                .unwrap()
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<i32>>(),
        );
    }
    successive_differences
        .into_iter()
        .rev()
        .fold(0, |acc, diff| diff.first().unwrap_or(&0) - acc)
}

fn part_one(histories: &Vec<Vec<i32>>) -> i32 {
    histories.iter().map(compute_next_value).sum()
}

fn part_two(histories: &Vec<Vec<i32>>) -> i32 {
    histories.iter().map(compute_new_first_value).sum()
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let histories = file_contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let part_one_history_sum = part_one(&histories);
    println!("Part One -- Sum of History Next Values: {part_one_history_sum}");
    
    let part_two_history_sum = part_two(&histories);
    println!("Part Two -- Sum of History New First Values: {part_two_history_sum}");
}
