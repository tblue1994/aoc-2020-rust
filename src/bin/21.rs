use std::collections::{HashMap, HashSet, VecDeque};

use regex::Regex;

type IngAlg = (Vec<String>, Vec<String>);
type Allergens = HashSet<String>;

pub fn parse_input(input: &str) -> (Vec<IngAlg>, Allergens) {
    let l_regex = Regex::new(r"(.*) \(contains (.*)\)").unwrap();
    let mut allergens_list = Allergens::new();
    let ing_alg = input
        .lines()
        .map(|l| {
            let caps = l_regex.captures(l).unwrap();
            let ingredients: Vec<String> = caps[1].split(' ').map(|s| s.to_owned()).collect();
            let allergens: Vec<String> = caps[2].split(", ").map(|s| s.to_owned()).collect();
            allergens.iter().for_each(|a| {
                allergens_list.insert(a.to_owned());
            });
            (ingredients, allergens)
        })
        .collect();
    (ing_alg, allergens_list)
}

fn get_bad_ing(ing_alg: &[IngAlg], a_list: Allergens) -> HashSet<String> {
    let mut bad_ing: HashSet<String> = HashSet::new();
    for a in a_list {
        let sets: Vec<&IngAlg> = ing_alg.iter().filter(|(_, alg)| alg.contains(&a)).collect();
        let mut common: HashSet<String> = HashSet::new();
        for (ing, _) in sets {
            let mut new_common: HashSet<String> = HashSet::new();
            for i in ing {
                if common.is_empty() || common.contains(i) {
                    new_common.insert(i.to_owned());
                }
            }
            common = new_common;
        }
        for c in common {
            bad_ing.insert(c);
        }
    }
    bad_ing
}

pub fn part_one(input: &str) -> Option<u32> {
    let (ing_alg, a_list) = parse_input(input);

    let bad_ing = get_bad_ing(&ing_alg, a_list);

    let mut count = 0;

    for (ing, _) in ing_alg {
        for i in ing {
            if !bad_ing.contains(&i) {
                count += 1;
            }
        }
    }

    Some(count)
}

// pub fn part_two(input: &str) -> Option<String> {
//     let (ing_alg, a_list) = parse_input(input);
//     let bad_ing = get_bad_ing(&ing_alg, a_list);
//     let mut q: VecDeque<String> = VecDeque::new();
//     let mut map: HashMap<String, String> = HashMap::new();
//     for i in bad_ing {
//         q.push_back(i);
//     }
//     while let Some(i) = q.pop_front() {
//         let sets: Vec<&IngAlg> = ing_alg.iter().filter(|(ing, _)| ing.contains(&i)).collect();
//         let mut common: HashSet<String> = HashSet::new();
//         for (_, alg) in sets {
//             let mut new_common: HashSet<String> = HashSet::new();
//             for a in alg {
//                 if (common.is_empty() || common.contains(a)) && !map.contains_key(a) {
//                     new_common.insert(a.to_owned());
//                 }
//             }
//             common = new_common;
//         }
//         if common.len() == 1 {
//             map.insert(common.iter().next().unwrap().to_string(), i);
//         } else {
//             q.push_back(i)
//         }
//     }
//     let mut kvps = map.iter().collect::<Vec<(&String, &String)>>();
//     kvps.sort_by(|a, b| a.0.cmp(b.0));
//     Some(
//         kvps.iter()
//             .map(|(_, v)| v.to_string())
//             .collect::<Vec<String>>()
//             .join(","),
//     )
// }

pub fn part_two(input: &str) -> Option<String> {
    let (ing_alg, a_list) = parse_input(input);
    let mut q: VecDeque<String> = VecDeque::new();
    let mut map: HashMap<String, String> = HashMap::new();
    let mut visited: HashSet<String> = HashSet::new();
    for a in a_list {
        q.push_back(a);
    }
    while let Some(a) = q.pop_front() {
        let sets: Vec<&IngAlg> = ing_alg.iter().filter(|(_, alg)| alg.contains(&a)).collect();
        let mut common: HashSet<String> = HashSet::new();
        for (ing, _) in sets {
            let mut new_common: HashSet<String> = HashSet::new();
            for i in ing {
                if (common.is_empty() || common.contains(i)) && !visited.contains(i) {
                    new_common.insert(i.to_owned());
                }
            }
            common = new_common;
        }
        if common.len() == 1 {
            let i = common.iter().next().unwrap().to_string();
            map.insert(a, i.to_string());
            visited.insert(i.to_string());
        } else {
            q.push_back(a)
        }
    }
    let mut kvps = map.iter().collect::<Vec<(&String, &String)>>();
    kvps.sort_by(|a, b| a.0.cmp(b.0));
    Some(
        kvps.iter()
            .map(|(_, v)| v.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some("mxmxvkd,sqjhc,fvjkl".to_owned()));
    }
}
