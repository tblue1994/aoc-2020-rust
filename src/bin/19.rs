use std::collections::HashMap;

type Ruleset = HashMap<u32, String>;

pub fn build_ruleset(input: &str) -> Ruleset {
    let mut ruleset = Ruleset::new();
    input.lines().for_each(|l| {
        let mut s = l.split(": ");
        ruleset.insert(
            s.next().unwrap().parse().unwrap(),
            s.next().unwrap().to_string(),
        );
    });
    ruleset
}

pub fn handle_num_rule(input: String, rule: String, ruleset: &Ruleset) -> Vec<u32> {
    let mut states = vec![(input, 0)];
    for sub_rule in rule.split(' ').map(|s| s.parse::<u32>().unwrap()) {
        let mut new_states: Vec<(String, u32)> = vec![];
        for (str, consumed) in states {
            let consumed_by_rule = validate_message(str.to_string(), sub_rule, ruleset);
            for v in consumed_by_rule {
                if v == 0 {
                    break;
                } else {
                    new_states.push((str[(v as usize)..str.len()].to_string(), consumed + v))
                }
            }
        }
        states = new_states;
        if states.is_empty() {
            break;
        }
    }
    if states.is_empty() {
        return vec![0];
    }
    states.iter().map(|(_, x)| *x).collect()
}

pub fn validate_message(input: String, rule: u32, ruleset: &Ruleset) -> Vec<u32> {
    let rule = ruleset.get(&rule).unwrap();
    if rule.contains('"') {
        vec![u32::from(
            rule.to_string().chars().nth(1).unwrap() == input.chars().next().unwrap_or_default(),
        )]
    } else if rule.contains('|') {
        rule.split(" | ")
            .flat_map(|s| handle_num_rule(input.to_string(), s.to_string(), ruleset))
            .filter(|r| *r > 0)
            .collect::<Vec<u32>>()
    } else {
        handle_num_rule(input, rule.to_string(), ruleset)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut parts = input.split("\n\n");
    let ruleset = build_ruleset(parts.next().unwrap());
    Some(
        parts
            .next()
            .unwrap()
            .lines()
            .filter(|l| validate_message(l.to_string(), 0, &ruleset)[0] == l.len() as u32)
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut parts = input.split("\n\n");
    let mut ruleset = build_ruleset(parts.next().unwrap());
    ruleset
        .entry(8)
        .and_modify(|rule| *rule = "42 | 42 8".to_string());
    ruleset
        .entry(11)
        .and_modify(|rule| *rule = "42 31 | 42 11 31".to_string());

    Some(
        parts
            .next()
            .unwrap()
            .lines()
            .filter(|l| validate_message(l.to_string(), 0, &ruleset)[0] == l.len() as u32)
            .count(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(12));
    }
}
