use std::fs;

#[derive(Debug)]
struct Scratchcard {
    index: usize,
    winning_number: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Scratchcard {
    fn matches(&self) -> usize {
        self
            .card_numbers
            .iter()
            .filter(|n| self.winning_number.contains(n))
            .count()
    }

    fn score(&self) -> u32 {
        let matches = self.matches() as u32;
        if matches == 0 {
            0
        } else {
            2u32.pow(matches - 1)
        }
    }
}

fn parse_scratchcard(line: &str) -> Scratchcard {
    let line_split = line.split(&[':', '|']).collect::<Vec<&str>>();
    Scratchcard {
        index: line_split[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap(),
        winning_number: line_split[1]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
        card_numbers: line_split[2]
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect(),
    }
}

fn part_one(scratchcard_vec: &Vec<Scratchcard>) -> u32 {
    scratchcard_vec.into_iter().map(|sc| sc.score()).sum()
}

fn part_two(scratchcard_vec: &Vec<Scratchcard>) -> u32 {
    // Prepare scratchcard counts vector
    let mut scratchcard_counts: Vec<u32> = vec![1u32; scratchcard_vec.len() + 1];
    scratchcard_counts[0] = 0;

    for scratchcard in scratchcard_vec {
        for i in 0usize..scratchcard.matches() {
            let update_index = scratchcard.index + i + 1;
            if update_index < scratchcard_counts.len() {
                scratchcard_counts[update_index] += scratchcard_counts[scratchcard.index];
            }
        }
    }
    
    scratchcard_counts.into_iter().sum()
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    let scratchcard_vec: Vec<Scratchcard> = file_contents.lines().map(parse_scratchcard).collect();

    println!("Part One -- Sum of Score: {}", part_one(&scratchcard_vec));
    println!("Part Two -- Total Number of Tickets: {}", part_two(&scratchcard_vec));
}
