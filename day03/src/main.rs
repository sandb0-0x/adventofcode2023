use itertools::iproduct;
use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct PartNum {
    value: u32,
    loc: (usize, usize),
    len: usize,
}

impl PartNum {
    fn adjacent_to_symbol(&self, symbol_map: &HashMap<(usize, usize), &str>) -> bool {
        iproduct!(
            (self.loc.0.saturating_sub(1))..(self.loc.0 + 2),
            (self.loc.1.saturating_sub(1))..(self.loc.1 + self.len + 1)
        )
        .any(|x| symbol_map.contains_key(&x))
    }

    fn all_adjacent_symbols(
        &self,
        symbol_map: &HashMap<(usize, usize), &str>,
        symbol_matcher: &dyn Fn(&str) -> bool,
    ) -> Vec<(usize, usize)> {
        iproduct!(
            (self.loc.0.saturating_sub(1))..(self.loc.0 + 2),
            (self.loc.1.saturating_sub(1))..(self.loc.1 + self.len + 1)
        )
        .filter(|x| {
            if let Some(symbol) = symbol_map.get(&x) {
                symbol_matcher(&symbol)
            } else {
                false
            }
        })
        .collect()
    }
}

fn part_one(part_num_vec: &Vec<PartNum>, symbol_map: &HashMap<(usize, usize), &str>) -> u32 {
    part_num_vec
        .iter()
        .filter(|part_num| part_num.adjacent_to_symbol(symbol_map))
        .map(|part_num| part_num.value)
        .sum()
}

fn part_two(part_num_vec: &Vec<PartNum>, symbol_map: &HashMap<(usize, usize), &str>) -> u32 {
    // Map from (location) -> (All part numbers adjacent), for each gear
    let mut gears_to_part_numbers: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    // Populate gears_to_part_numbers
    part_num_vec
        .iter()
        .flat_map(|part_num: &PartNum| {
            part_num
                .all_adjacent_symbols(symbol_map, &|s| s == "*")
                .into_iter()
                .map(|x| (x, part_num.value))
        })
        .for_each(|(loc, part_value)| {
            if !gears_to_part_numbers.contains_key(&loc) {
                gears_to_part_numbers.insert(loc, Vec::new());
            };
            gears_to_part_numbers
                .get_mut(&loc)
                .unwrap()
                .push(part_value);
        });
    // Compute sum of products of gear part numbers
    gears_to_part_numbers
        .into_iter()
        .map(|(_, part_value_vec)| {
            if part_value_vec.len() == 2 {
                part_value_vec[0] * part_value_vec[1]
            } else {
                0
            }
        })
        .sum()
}

fn main() {
    let digits_re = Regex::new(r"(\d+)").unwrap();
    let symbol_re = Regex::new(r"[^\d.]").unwrap();

    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");
    
    let part_num_vec: Vec<PartNum> = file_contents
        .lines()
        .zip(0..)
        .flat_map(|(line, line_index)| {
            digits_re
                .captures_iter(line)
                .map(|c| c.get(0).unwrap())
                .map(move |m| PartNum {
                    value: m.as_str().parse().unwrap(),
                    loc: (line_index, m.start()),
                    len: m.len(),
                })
        })
        .collect();
    // println!("{:?}", part_num_vec);

    let symbol_map: HashMap<(usize, usize), &str> = file_contents
        .lines()
        .zip(0..)
        .flat_map(|(line, line_index)| {
            symbol_re
                .captures_iter(line)
                .map(|c| c.get(0).unwrap())
                .map(move |m| ((line_index, m.start()), m.as_str()))
        })
        .collect();
    // println!("{:?}", symbol_map);

    let part_one_solution = part_one(&part_num_vec, &symbol_map);
    println!("Part One -- Sum of Part Numbers: {part_one_solution}");
    let part_two_solution = part_two(&part_num_vec, &symbol_map);
    println!("Part Two -- SumProduct of Gear-Adjacent parts: {part_two_solution}");
}
