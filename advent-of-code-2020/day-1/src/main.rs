use std::collections::HashMap;
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

/// Parses the input into a vector of unsigned integers.
/// Assumes that all data in file can be parsed correctly.
/// Panics if file doesn't exist.
fn parse_input(filename: &str) -> Vec<u32> {
    match read_lines(filename) {
        Ok(lines) => {
            lines.into_iter()
                .map(|line| {
                    let value = line.unwrap();
                    value.parse::<u32>().unwrap()
                })
                .collect()
        },
        Err(e) => panic!("Error reading file: {}", e)
    }
}

const TARGET: u32 = 2020;

/// Helper function to calculate the product of two numbers
/// that sum to a specific target. This function is used in both
/// puzzle solutions.
/// 
/// # Approach
/// 
/// Makes use of a HashMap to store complements and indices. If a complement is seen
/// later in the input then the product of the current value and its complement are returned.
fn two_sum_product(input: &[u32], target: u32, skip: Option<usize>) -> Option<u32> {
    let mut complements = HashMap::new();
    
    for i in 0..input.len() {
        if skip.is_some() && skip.unwrap() == i {
            continue;
        }

        if input[i] > target {
            continue;
        }

        match complements.get(&input[i]) {
            Some(&idx) => return Some(input[idx] * input[i]),
            None => complements.insert(target - input[i], i)
        };
    }

    None
}

/// Function to solve part 1
/// This is basically a Two Sum problem.
/// Calls the helper function and returns the result. 
pub fn puzzle_one_solution(input: &[u32]) -> Option<u32> {
    two_sum_product(input, TARGET, None)
}

/// Function to solve part 2
/// This is basically a Three Sum problem.
/// Iterates over the list and passes the `two_sum_target`
/// into the helper function. Returns the product of the current value and
/// its two sum product.
pub fn puzzle_two_solution(input: &[u32]) -> Option<u32> {
    for i in 0..input.len() {
        let two_sum_target = TARGET - input[i];

        if let Some(result) = two_sum_product(input, two_sum_target, Some(i)) {
            return Some(input[i] * result);
        }   
    }

    None
}

fn main() {
    let input = parse_input("./puzzle_input.txt");

    println!("Puzzle 1 Solution -> {}", puzzle_one_solution(&input).unwrap());
    println!("Puzzle 2 Solution -> {}", puzzle_two_solution(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puzzle_one_example_input() {
        assert_eq!(puzzle_one_solution(&[1721, 979, 366, 299, 675, 1456]), Some(514579));
    }

    #[test]
    fn test_puzzle_two_example_input() {
        assert_eq!(puzzle_two_solution(&[1721, 979, 366, 299, 675, 1456]), Some(241861950))
    }

    #[test]
    fn test_puzzle_one_bad_input() {
        assert_eq!(puzzle_one_solution(&[]), None);
        assert_eq!(puzzle_one_solution(&[1, 2, 3, 4]), None)
    }

    #[test]
    fn test_puzzle_two_bad_input() {
        assert_eq!(puzzle_two_solution(&[]), None);
        assert_eq!(puzzle_two_solution(&[1, 2, 3, 4]), None)
    }

}