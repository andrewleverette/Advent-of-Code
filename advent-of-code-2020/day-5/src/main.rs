use std::cmp;
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

/// Parse the input into a vector of strings
fn parse_input(filename: &str) -> Vec<String> {
    let mut results = Vec::new();

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                let line = line.unwrap();
                results.push(line);
            }
        },
        Err(e) => panic!("Error reading file: {}", e),
    }

    results
}

/// Maximum row number on plane
const MAX_ROW: u16 = 127;

/// Maximum column number on plane
const MAX_COLUMN: u16 = 7;

/// Calculates the row number given a string of
/// binary directions 'F' or 'B'
fn calculate_row(row_spec: &str) -> u16 {
    let mut start = 0;
    let mut end = MAX_ROW;

    let mut mid = (start + end) / 2;

    for d in row_spec.chars() {
        if d == 'F' {
            end = mid;
        } else {
            start = mid + 1;
        }

        mid = (start + end) / 2;
    }

    mid
}

/// Calculates the column number give a string
/// of binary directions 'L' or 'R'
fn calculate_column(col_spec: &str) -> u16 {
    let mut start = 0;
    let mut end = MAX_COLUMN;

    let mut mid = (start + end) / 2;

    for d in col_spec.chars() {
        if d == 'L' {
            end = mid;
        } else {
            start = mid + 1;
        }

        mid = (start + end) / 2;
    }

    mid
}

/// Calculates the seat id by
/// parsing out the row and column specs and
/// and calculating the row and column number.
fn calculate_seat_id(boarding_pass: &str) -> u16 {
    let row_spec = &boarding_pass[..boarding_pass.len() - 3];
    let col_spec = &boarding_pass[boarding_pass.len() - 3..];

    calculate_row(row_spec) * 8 + calculate_column(col_spec)
}

fn find_empty_seat(map: Vec<bool>) -> Option<u16> {
    for (left, window) in map.windows(3).enumerate() {
        if window[0] && !window[1] && window[2] {
            return Some((left + 1) as u16);
        }
    }

    None
}

/// This solution calculates the seat id for each boarding pass
/// and tracks the maximum seat id seen so far
fn puzzle_one_solution(input: &[String]) -> u16 {
    let mut max_seat_id = 0;

    for boarding_pass in input {
        let seat_id = calculate_seat_id(boarding_pass);

        max_seat_id = cmp::max(max_seat_id, seat_id);
    }

    max_seat_id
}

/// This solution builds an a vector of boolean values
/// that represent if a seat is taken.
/// Once the vector is built, the empty seat is found
fn puzzle_two_solution(input: &[String]) -> u16 {
    let mut seat_map = vec![false; 128 * 8];

    for boarding_pass in input {
        let seat_id = calculate_seat_id(boarding_pass) ;
        seat_map[seat_id as usize] = true;
    }

    match find_empty_seat(seat_map) {
        Some(seat) => seat,
        None => panic!("No solution found for puzzle two!")
    }
}

fn main() {
    let input = parse_input("/Users/andrewleverette/Documents/Workspace/advent-of-code/advent-of-code-2020/day-5//puzzle_input.txt");

    println!("Puzzle 1 Solution -> {}", puzzle_one_solution(&input));
    println!("Puzzle 2 Solution -> {}", puzzle_two_solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<String> {
        vec![
            "BFFFBBFRRR".to_owned(),
            "FFFBBBFRRR".to_owned(),
            "BBFFBBFRLL".to_owned(),
        ]
    }

    #[test]
    fn test_puzzle_one_example() {
        let input = example_input();

        assert_eq!(puzzle_one_solution(&input), 820)
    }

    #[test]
    fn test_calculate_seat_id() {
        let boarding_pass = "FBFBBFFRLR";

        assert_eq!(calculate_seat_id(boarding_pass), 357);
    }
}
