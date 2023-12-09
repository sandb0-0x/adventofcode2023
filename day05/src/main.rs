use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::slice::Iter;
use std::str::FromStr;

#[derive(Debug)]
struct MapEntry {
    source_range_start: usize,
    range_length: usize,
    // The difference between the destination and source values in the map entry
    offset: i64,
}

impl FromStr for MapEntry {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_iter = s.split_whitespace();
        let dest_range_start = split_iter.next().ok_or("No dest range")?.parse::<usize>()?;
        let source_range_start = split_iter
            .next()
            .ok_or("No source range")?
            .parse::<usize>()?;
        let range_length = split_iter
            .next()
            .ok_or("No range length")?
            .parse::<usize>()?;
        assert!(split_iter.next().is_none());

        Ok(MapEntry {
            source_range_start: source_range_start,
            range_length: range_length,
            offset: (dest_range_start as i64) - (source_range_start as i64),
        })
    }
}

#[derive(Debug)]
struct AlmanacMap {
    source_category: String,
    dest_category: String,
    // Assume this is sorted, for efficiency
    map_entries: Vec<MapEntry>,
}

impl AlmanacMap {
    fn apply_mapping(&self, index: usize) -> usize {
        for map_entry in &self.map_entries {
            if index < map_entry.source_range_start {
                return index;
            }
            if index < map_entry.source_range_start + map_entry.range_length {
                return ((index as i64) + map_entry.offset) as usize;
            }
        }
        index
    }
}

fn map_seeds_to_min_location(
    seed_iter: Iter<usize>,
    almanac_maps: &HashMap<(String, String), AlmanacMap>,
) -> usize {
    let seed_soil_map = almanac_maps
        .get(&(String::from("seed"), String::from("soil")))
        .unwrap();
    let soil_fertilizer_map = almanac_maps
        .get(&(String::from("soil"), String::from("fertilizer")))
        .unwrap();
    let fertilizer_water_map = almanac_maps
        .get(&(String::from("fertilizer"), String::from("water")))
        .unwrap();
    let water_light_map = almanac_maps
        .get(&(String::from("water"), String::from("light")))
        .unwrap();
    let light_temp_map = almanac_maps
        .get(&(String::from("light"), String::from("temperature")))
        .unwrap();
    let temp_humidity_map = almanac_maps
        .get(&(String::from("temperature"), String::from("humidity")))
        .unwrap();
    let humidity_location_map = almanac_maps
        .get(&(String::from("humidity"), String::from("location")))
        .unwrap();

    let mut min_location = usize::MAX;
    seed_iter
        .map(|i| seed_soil_map.apply_mapping(*i))
        .map(|i| soil_fertilizer_map.apply_mapping(i))
        .map(|i| fertilizer_water_map.apply_mapping(i))
        .map(|i| water_light_map.apply_mapping(i))
        .map(|i| light_temp_map.apply_mapping(i))
        .map(|i| temp_humidity_map.apply_mapping(i))
        .map(|i| humidity_location_map.apply_mapping(i))
        .for_each(|i| {
            if i <= min_location {
                min_location = i
            }
        });
    min_location
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let almanac_file_regex =
        Regex::new(r"seeds:\s([0-9 ]+)\s+((?:\w+-\w+-\w+ map:\s+[0-9\s]*)*)").unwrap();
    let mapping_regex = Regex::new(r"(\w+)-\w+-(\w+)\s+map:\s+([0-9\s]*)(?:(?:\s+)|\z)").unwrap();

    let almanac_captures = almanac_file_regex
        .captures(&file_contents)
        .expect("Almanac file did not match");
    let seeds = almanac_captures.get(1).unwrap().as_str().to_owned();
    let all_mappings = almanac_captures.get(2).unwrap().as_str().to_owned();

    let seed_vec = seeds
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("{seed_vec:?}");

    let almanac_maps = mapping_regex
        .captures_iter(&all_mappings)
        .map(|m| AlmanacMap {
            source_category: m.get(1).unwrap().as_str().to_owned(),
            dest_category: m.get(2).unwrap().as_str().to_owned(),
            map_entries: m
                .get(3)
                .unwrap()
                .as_str()
                .lines()
                .filter_map(|e| e.parse().ok())
                .collect(),
        })
        .map(|mut m: AlmanacMap| {
            m.map_entries.sort_by_key(|e| e.source_range_start);
            ((m.source_category.clone(), m.dest_category.clone()), m)
        })
        .collect::<HashMap<(String, String), AlmanacMap>>();

    // let part_one_min_location = map_seeds_to_min_location(seed_vec.iter(), &almanac_maps);
    // println!(
    //     "Part One -- Minimum Location Value: {}",
    //     part_one_min_location
    // );

    let mut updated_seed_vec = Vec::<usize>::new();
    seed_vec.chunks(2).into_iter().for_each(|c| {
        let (range_start, range_length) = (c[0], c[1]);
        updated_seed_vec.extend(range_start..range_start + range_length);
    });
    // println!("New Vec Len: {}", updated_seed_vec.len());

    let part_two_min_location = map_seeds_to_min_location(updated_seed_vec.iter(), &almanac_maps);
    println!(
        "Part Two -- Minimum Location Value: {}",
        part_two_min_location
    );
}
