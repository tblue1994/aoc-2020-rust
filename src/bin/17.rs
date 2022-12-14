use std::collections::HashSet;

type Pos = (i32, i32, i32, i32);
type State = HashSet<Pos>;
pub fn parse_start(input: &str) -> State {
    let mut starting = State::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                starting.insert((i as i32, j as i32, 0, 0));
            }
        }
    }
    starting
}

pub fn step(state: &State, use4d: bool) -> State {
    let mut next_state = State::new();
    let all_pos_next = get_full_next_state(state, use4d);
    for point in all_pos_next {
        let active = state.contains(&point);
        let mut count = 0;
        for n in neighbors(point, use4d, false) {
            if state.contains(&n) {
                count += 1;
                if count >= 4 {
                    break;
                }
            }
        }
        if count == 3 || (active && count == 2) {
            next_state.insert(point);
        }
    }
    next_state
}

pub fn neighbors(point: Pos, use4d: bool, include_center: bool) -> Vec<Pos> {
    let mut neighbors: Vec<Pos> = vec![];
    for x in -1..2 {
        for y in -1..2 {
            for z in -1..2 {
                if use4d {
                    for w in -1..2 {
                        if include_center || !(x == 0 && y == 0 && z == 0 && w == 0) {
                            neighbors.push((point.0 + x, point.1 + y, point.2 + z, point.3 + w));
                        }
                    }
                } else if include_center || !(x == 0 && y == 0 && z == 0) {
                    neighbors.push((point.0 + x, point.1 + y, point.2 + z, 0));
                }
            }
        }
    }
    neighbors
}

pub fn get_full_next_state(state: &State, use4d: bool) -> State {
    let mut next_state = State::new();
    for key in state {
        let neighbors = neighbors(*key, use4d, true);
        for n in neighbors {
            next_state.insert(n);
        }
    }
    next_state
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut state = parse_start(input);
    for _ in 0..6 {
        state = step(&state, false);
    }
    Some(state.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut state = parse_start(input);
    for _ in 0..6 {
        state = step(&state, true);
    }
    Some(state.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(112));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(848));
    }
}
