use std::collections::HashMap;

pub fn build_order(input: &str, size: usize) -> usize {
    let input_nums: Vec<usize> = input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut hash: HashMap<usize, usize> = HashMap::new();
    (0..(input_nums.len() - 1)).for_each(|i| {
        hash.insert(input_nums[i], i);
    });
    let mut current_num = input_nums[input_nums.len() - 1];

    for i in (input_nums.len() - 1)..size - 1 {
        if let std::collections::hash_map::Entry::Vacant(e) = hash.entry(current_num) {
            e.insert(i);
            current_num = 0
        } else {
            let new_num = i - *hash.get(&current_num).unwrap();
            *hash.get_mut(&current_num).unwrap() = i;
            current_num = new_num
        }
    }
    current_num
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(build_order(input, 2020))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(build_order(input, 30000000))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(436));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(175594));
    }
}
