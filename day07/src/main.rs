use regex::Regex;
use std::{
    cmp::{self, Ordering},
    collections::HashMap,
    fs,
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

trait CamelCardHandGeneric:
    Clone + Copy + PartialEq + Eq + PartialOrd + FromStr<Err = Box<dyn std::error::Error>>
{
}

impl CamelCardHandGeneric for CamelCardHand {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CamelCardHand {
    hand: [u32; 5],
    hand_type: HandType,
}

impl FromStr for CamelCardHand {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut letter_counts: HashMap<char, usize> = HashMap::new();
        let hand_vec = s
            .chars()
            .map(|c| match c {
                'A' => Ok(14),
                'K' => Ok(13),
                'Q' => Ok(12),
                'J' => Ok(11),
                'T' => Ok(10),
                _ => c.to_digit(10).ok_or("No matching card type"),
            })
            .collect::<Result<Vec<u32>, _>>()?;

        let letter_counts: HashMap<char, u32> = s.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let mut sorted_letter_freqs = letter_counts.into_values().collect::<Vec<u32>>();
        sorted_letter_freqs.sort_by_key(|&v| cmp::Reverse(v));
        let hand_type = match (sorted_letter_freqs.get(0), sorted_letter_freqs.get(1)) {
            (Some(5), _) => HandType::FiveOfAKind,
            (Some(4), _) => HandType::FourOfAKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::ThreeOfAKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::OnePair,
            (Some(1), _) => HandType::HighCard,
            _ => return Err("No matching hand type")?,
        };

        match hand_vec.try_into() {
            Ok(hand) => Ok(CamelCardHand {
                hand: hand,
                hand_type: hand_type,
            }),
            Err(e) => Err("Unable to convert input to Camel Hand")?,
        }
    }
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.hand_type, self.hand).cmp(&(&other.hand_type, other.hand))
    }
}

impl PartialOrd for CamelCardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CamelCardHandWithJokers {
    hand: [u32; 5],
    hand_type: HandType,
}

impl CamelCardHandGeneric for CamelCardHandWithJokers {}

impl FromStr for CamelCardHandWithJokers {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // let mut letter_counts: HashMap<char, usize> = HashMap::new();
        let hand_vec = s
            .chars()
            .map(|c| match c {
                'A' => Ok(14),
                'K' => Ok(13),
                'Q' => Ok(12),
                'J' => Ok(1),
                'T' => Ok(10),
                _ => c.to_digit(10).ok_or("No matching card type"),
            })
            .collect::<Result<Vec<u32>, _>>()?;

        let mut letter_counts: HashMap<char, u32> = s.chars().fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

        let joker_count = if let Some(count) = letter_counts.remove(&'J') {
            count.to_owned()
        } else {
            0
        };

        let mut sorted_letter_freqs = letter_counts.into_values().collect::<Vec<u32>>();
        sorted_letter_freqs.sort_by_key(|&v| cmp::Reverse(v));
        // Add jokers to the most frequent number
        // sorted_letter_freqs[0] += joker_count;
        if let Some(first) = sorted_letter_freqs.first_mut() {
            *first += joker_count;
        } else {
            sorted_letter_freqs.push(joker_count);
        }

        let hand_type = match (sorted_letter_freqs.get(0), sorted_letter_freqs.get(1)) {
            (Some(5), _) => HandType::FiveOfAKind,
            (Some(4), _) => HandType::FourOfAKind,
            (Some(3), Some(2)) => HandType::FullHouse,
            (Some(3), _) => HandType::ThreeOfAKind,
            (Some(2), Some(2)) => HandType::TwoPair,
            (Some(2), _) => HandType::OnePair,
            (Some(1), _) => HandType::HighCard,
            _ => return Err("No matching hand type")?,
        };

        match hand_vec.try_into() {
            Ok(hand) => Ok(CamelCardHandWithJokers {
                hand: hand,
                hand_type: hand_type,
            }),
            Err(e) => Err("Unable to convert input to Camel Hand")?,
        }
    }
}

impl Ord for CamelCardHandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.hand_type, self.hand).cmp(&(&other.hand_type, other.hand))
    }
}

impl PartialOrd for CamelCardHandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_file_contents<H: CamelCardHandGeneric>(
    file_contents: &str,
) -> Result<Vec<(H, u32)>, Box<dyn std::error::Error>> {
    file_contents
        .lines()
        .map(|line| {
            let mut line_split = line.split_whitespace();
            let hand = line_split.next().ok_or("No Camel Hand")?.parse::<H>()?;
            let bid = line_split.next().ok_or("No bid")?.parse::<u32>()?;
            Ok((hand, bid))
        })
        .collect::<Result<Vec<(_, _)>, Box<dyn std::error::Error>>>()
}

fn part_one(hands_with_bids: &Vec<(CamelCardHand, u32)>) -> u32 {
    let mut hands_with_bids_sorted: Vec<(CamelCardHand, u32)> = hands_with_bids.to_vec();
    hands_with_bids_sorted.sort_by_key(|&(hand, _)| hand);
    // println!("Hands with bids sorted: {hands_with_bids_sorted:?}");
    hands_with_bids_sorted
        .into_iter()
        .zip(1..)
        .map(|((hand, bid), rank)| bid * rank)
        .sum()
}

fn part_two(hands_with_bids_and_jokers: &Vec<(CamelCardHandWithJokers, u32)>) -> u32 {
    let mut hands_with_bids_sorted: Vec<(CamelCardHandWithJokers, u32)> =
        hands_with_bids_and_jokers.to_vec();
    hands_with_bids_sorted.sort_by_key(|&(hand, _)| hand);
    // println!("Hands with bids sorted: {hands_with_bids_sorted:?}");
    hands_with_bids_sorted
        .into_iter()
        .zip(1..)
        .map(|((hand, bid), rank)| bid * rank)
        .sum()
}

fn main() {
    let file_path = "input.txt";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read file: {file_path}");

    let hands_with_bids = match parse_file_contents(&file_contents) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Unable to parse file contents: {err}");
            return;
        }
    };

    let part_one_total_winnings = part_one(&hands_with_bids);
    println!("Part One -- Total Winnings: {part_one_total_winnings}");

    let hands_with_bids_and_jokers = match parse_file_contents(&file_contents) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("Unable to parse file contents: {err}");
            return;
        }
    };

    let part_two_total_winnings = part_two(&hands_with_bids_and_jokers);
    println!("Part Two -- Total Winnings: {part_two_total_winnings}");
}
