#[derive(PartialEq, Eq)]
pub enum Mode {
    Addition,
    Multiplication,
}

pub fn handle_value(current: Option<u64>, num: u64, mode: &Option<Mode>) -> Option<u64> {
    if current.is_none() {
        return Some(num);
    } else if *mode == Some(Mode::Multiplication) {
        return Some(current.unwrap() * num);
    } else if *mode == Some(Mode::Addition) {
        return Some(current.unwrap() + num);
    }
    panic!("shits bad");
}

pub fn solve1(problem: String) -> u64 {
    let mut current: Option<u64> = None;
    let mut mode: Option<Mode> = None;
    let mut index: usize = 0;
    loop {
        if index >= problem.len() {
            break;
        }
        let c = problem.chars().nth(index).unwrap();
        match c {
            '*' => {
                mode = Some(Mode::Multiplication);
                index += 1;
            }
            '+' => {
                mode = Some(Mode::Addition);
                index += 1;
            }
            '1'..='9' => {
                let num = c.to_digit(10).unwrap() as u64;
                current = handle_value(current, num, &mode);
                index += 1
            }
            '(' => {
                let mut parens = 0;
                index += 1;
                let mut substring: Vec<char> = vec![];
                loop {
                    let current = problem.chars().nth(index).unwrap();
                    match current {
                        '(' => {
                            parens += 1;
                            substring.push(current);
                        }
                        ')' => {
                            if parens == 0 {
                                break;
                            } else {
                                parens -= 1;
                                substring.push(current)
                            }
                        }
                        _ => substring.push(current),
                    }
                    index += 1;
                }
                let value = solve1(substring.into_iter().collect());
                current = handle_value(current, value, &mode);
                // skips last paren
                index += 1;
            }
            _ => panic!("shits fucked"),
        }
    }

    current.unwrap()
}

pub fn solve2(problem: String) -> u64 {
    let mut current: Option<u64> = None;
    let mut mode: Option<Mode> = None;
    let mut index: usize = 0;
    loop {
        if index >= problem.len() {
            break;
        }
        let c = problem.chars().nth(index).unwrap();
        match c {
            '*' => {
                current = handle_value(
                    current,
                    solve2(problem.chars().skip(index + 1).collect()),
                    &Some(Mode::Multiplication),
                );
                break;
            }
            '+' => {
                mode = Some(Mode::Addition);
                index += 1;
            }
            '1'..='9' => {
                let num = c.to_digit(10).unwrap() as u64;
                current = handle_value(current, num, &mode);
                index += 1
            }
            '(' => {
                let mut parens = 0;
                index += 1;
                let mut substring: Vec<char> = vec![];
                loop {
                    let current = problem.chars().nth(index).unwrap();
                    match current {
                        '(' => {
                            parens += 1;
                            substring.push(current);
                        }
                        ')' => {
                            if parens == 0 {
                                break;
                            } else {
                                parens -= 1;
                                substring.push(current)
                            }
                        }
                        _ => substring.push(current),
                    }
                    index += 1;
                }
                let value = solve2(substring.into_iter().collect());
                current = handle_value(current, value, &mode);
                // skips last paren
                index += 1;
            }
            _ => panic!("shits fucked"),
        }
    }

    current.unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| l.replace(' ', "")).map(solve1).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| l.replace(' ', "")).map(solve2).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(13632));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(23340));
    }
}
