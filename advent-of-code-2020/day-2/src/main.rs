use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Creates an iterator over the lines of the puzzle input
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_input(filename: &str) -> Vec<((u32, u32, char), String)> {
    let mut results = Vec::new();

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                let line = line.unwrap();
                let line: Vec<&str> = line.split(':').collect();

                let policy: Vec<&str> = line[0].split(' ').collect();

                let frequencies: Vec<u32> = policy[0]
                    .split('-')
                    .map(|val| val.parse::<u32>().unwrap())
                    .collect();

                let character = policy[1].chars().next().unwrap();

                let password = line[1].trim().to_owned();

                results.push(((frequencies[0], frequencies[1], character), password));
            }
        }
        Err(e) => panic!("Error reading file: {}", e),
    };

    results
}

enum Strategy {
    Frequency,
    Position,
}

fn validate_by_frequency(policy: &(u32, u32, char), password: &str) -> bool {
    let mut character_count = 0;

    for c in password.chars() {
        if c == policy.2 {
            character_count += 1;
        }
    }

    policy.0 <= character_count && character_count <= policy.1
}

fn validate_by_position(policy: &(u32, u32, char), password: &str) -> bool {
    let characters: Vec<char> = password.chars().collect();

    (characters[policy.0 as usize - 1] == policy.2) ^ (characters[policy.1 as usize - 1] == policy.2)
}

/// Determines if a password is valid given a policy and a validation strategy
/// If the strategy is `Frequency` then the policy is interpreted as frequency ranges.
/// If the policy is `Position` then the policy is treated is indices into the password
fn is_password_valid(policy: &(u32, u32, char), password: &str, strategy: Strategy) -> bool {    
    match strategy {
        Strategy::Frequency => validate_by_frequency(policy, password),
        Strategy::Position => validate_by_position(policy, password),
    }
}

pub fn puzzle_one_solution(input: &[((u32, u32, char), String)]) -> u32 {
    let mut valid_count = 0;

    for (policy, password) in input {
        if is_password_valid(policy, password, Strategy::Frequency) {
            valid_count += 1;
        }
    }

    valid_count
}

pub fn puzzle_two_solution(input: &[((u32, u32, char), String)]) -> u32 {
    let mut valid_count = 0;

    for (policy, password) in input {
        if is_password_valid(policy, password, Strategy::Position) {
            valid_count += 1;
        }
    }

    valid_count
}

fn main() {
    let input = parse_input("./puzzle_input.txt");

    println!("Puzzle 1 Solution -> {}", puzzle_one_solution(&input));
    println!("Puzzle 2 Solution -> {}", puzzle_two_solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_one_example_input() {
        let input = vec![
            ((1, 3, 'a'), "abcde".to_owned()),
            ((1, 3, 'b'), "cdefg".to_owned()),
            ((2, 9, 'c'), "ccccccccc".to_owned()),
        ];

        assert_eq!(puzzle_one_solution(&input), 2);
    }

    #[test]
    fn test_puzzle_one_example_bad_input() {
        let input = vec![];

        assert_eq!(puzzle_one_solution(&input), 0);
    }

    #[test]
    fn test_puzzle_two_example_input() {
        let input = vec![
            ((1, 3, 'a'), "abcde".to_owned()),
            ((1, 3, 'b'), "cdefg".to_owned()),
            ((2, 9, 'c'), "ccccccccc".to_owned()),
        ];

        assert_eq!(puzzle_two_solution(&input), 1);
    }

    #[test]
    fn test_puzzle_two_example_bad_input() {
        let input = vec![];

        assert_eq!(puzzle_two_solution(&input), 0);
    }
}
