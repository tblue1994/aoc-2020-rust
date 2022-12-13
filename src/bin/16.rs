use regex::Regex;
use std::{collections::HashMap, ops::Range};

#[derive(Debug)]
pub struct Property {
    ranges: Vec<Range<u32>>,
    name: String,
}

impl Property {
    fn new(name: String, ranges: Vec<Range<u32>>) -> Self {
        Self { name, ranges }
    }
}
pub fn build_properties(input: &str) -> Vec<Property> {
    let r_regex = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let params: Vec<&str> = input.lines().collect();
    params
        .iter()
        .map(|p| {
            let caps = r_regex.captures(p).unwrap();

            Property::new(
                caps[1].to_string(),
                vec![
                    Range {
                        start: caps[2].parse::<u32>().unwrap(),
                        end: caps[3].parse::<u32>().unwrap() + 1,
                    },
                    Range {
                        start: caps[4].parse::<u32>().unwrap(),
                        end: caps[5].parse::<u32>().unwrap() + 1,
                    },
                ],
            )
        })
        .collect()
}

pub fn get_valid_tickets(input: &str, props: &Vec<Property>) -> (Vec<Vec<u32>>, u32) {
    let mut valid_tickets: Vec<Vec<u32>> = vec![];
    let mut total = 0;
    for nearby_ticket in input.lines().skip(1) {
        let nums: Vec<u32> = nearby_ticket
            .split(',')
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let mut valid = true;
        for n in &nums {
            let mut found = false;
            for prop in props {
                for range in &prop.ranges {
                    if range.contains(n) {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                total += n;
                valid = false;
                break;
            }
        }
        if valid {
            valid_tickets.push(nums)
        }
    }
    (valid_tickets, total)
}

pub fn part_one(input: &str) -> Option<u32> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let props: Vec<Property> = build_properties(parts[0]);

    let (_, total) = get_valid_tickets(parts[2], &props);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u128> {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let props: Vec<Property> = build_properties(parts[0]);

    let (valid_tickets, _) = get_valid_tickets(parts[2], &props);
    let my_ticket: Vec<u128> = parts[1]
        .lines()
        .skip(1)
        .take(1)
        .last()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u128>().unwrap())
        .collect();

    let mut available_indexes: Vec<usize> = (0..valid_tickets[0].len()).collect();
    let mut hash: HashMap<String, usize> = HashMap::new();
    while !available_indexes.is_empty() {
        for prop in &props {
            if hash.contains_key(&prop.name) {
                continue;
            }
            let mut potential_indexes: Vec<usize> = vec![];
            for i in &available_indexes {
                let mut index_values = valid_tickets.iter().map(|v| v[*i]);
                if index_values.all(|v| prop.ranges.iter().any(|r| r.contains(&v))) {
                    potential_indexes.push(*i)
                }
            }
            if potential_indexes.len() == 1 {
                available_indexes.retain(|v| *v != potential_indexes[0]);
                hash.insert(prop.name.to_string(), potential_indexes[0]);
            }
        }
    }

    if hash.contains_key("departure track") {
        Some(
            my_ticket[*hash.get("departure location").unwrap()]
                * my_ticket[*hash.get("departure station").unwrap()]
                * my_ticket[*hash.get("departure platform").unwrap()]
                * my_ticket[*hash.get("departure track").unwrap()]
                * my_ticket[*hash.get("departure date").unwrap()]
                * my_ticket[*hash.get("departure time").unwrap()],
        )
    } else {
        Some(
            my_ticket[*hash.get("class").unwrap()]
                * my_ticket[*hash.get("row").unwrap()]
                * my_ticket[*hash.get("seat").unwrap()],
        )
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(71));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(98));
    }
}
