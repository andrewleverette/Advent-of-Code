use std::collections::{HashMap, HashSet};
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

/// Parse the input into a vector of vector of chars
fn parse_input(filename: &str) -> Vec<Vec<char>> {
    let mut results = Vec::new();
    let mut group = Vec::new();
    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                let line = line.unwrap();

                if line == "" {
                    results.push(group.to_owned());
                    group = Vec::new();
                } else {
                    group.extend(line.chars());
                    group.push('\n');
                }
            }
        },
        Err(e) => panic!("Error reading file: {}", e),
    }

    results.push(group.to_owned());

    results
}

/// This solution iterates over each group of responses
/// and collects the unique responses for each group and 
/// returns the total number of unique responses
fn puzzle_one_solution(input: &[Vec<char>]) -> u32 {
    let mut answer_count = 0;

    for group in input {
        let answer_set: HashSet<&char> = group.iter().collect();
        answer_count += answer_set.len() - 1;
    }

    answer_count as u32
}

/// This solution counts the number of responses for each group 
/// and returns the number of responses that are included in every response
fn puzzle_two_solution(input: &[Vec<char>]) -> u32 {
    let mut answer_count = 0;

    for group in input {
        let mut response_count = 0;
        let mut answer_collection: HashMap<&char, u32> = HashMap::new();

        for response in group {
            if *response == '\n' {
                response_count += 1;
            } else {
                let counter = answer_collection.entry(response).or_insert(0);
                *counter += 1;
            }
        }

        answer_count += answer_collection.values().filter(|&&v| v == response_count).count() as u32;
    }

    answer_count
}

fn main() {
    let input = parse_input("./puzzle_input.txt");

    println!("Puzzle 1 Solution -> {}", puzzle_one_solution(&input));
    println!("Puzzle 2 Solution -> {}", puzzle_two_solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> Vec<Vec<char>> {
        vec![
            vec!['a', 'b', 'c', '\n'],
            vec!['a', '\n', 'b', '\n', 'c', '\n'],
            vec!['a', 'b', '\n', 'a', 'c', '\n'],
            vec!['a', '\n', 'a', '\n', 'a', '\n', 'a', '\n'],
            vec!['b', '\n']
        ]
    }

    #[test]
    fn test_puzzle_one_example() {
        let input = get_example_input();

        assert_eq!(puzzle_one_solution(&input), 11);
    }

    #[test]
    fn test_puzzle_two_example() {
        let input = get_example_input();

        assert_eq!(puzzle_two_solution(&input), 6);
    }
}