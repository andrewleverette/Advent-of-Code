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

/// Parse input into a vector of vector of `Map` objects
fn parse_input(filename: &str) -> Vec<Vec<Map>> {
    let mut results = Vec::new();

    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                let line = line.unwrap();

                let parsed = line
                    .trim()
                    .chars()
                    .map(|c| if c == '.' { Map::Open } else { Map::Tree })
                    .collect();

                results.push(parsed);
            }
        }
        Err(e) => panic!("Error reading file: {}", e),
    }

    results
}

pub enum Map {
    Open,
    Tree,
}

/// Counts the number of trees along a path given a slope
fn count_trees_on_slope(input: &[Vec<Map>], slope: &(usize, usize)) -> u32 {
    let mut tree_count = 0;
    let mut row = slope.1;
    let mut column = slope.0;

    while row < input.len() {
        if let Map::Tree = input[row][column] {
            tree_count += 1;
        }

        row += slope.1;
        column = (column + slope.0) % input[0].len();
    }

    tree_count
}

/// Counts the tree along a single path given the slope of a the path
pub fn puzzle_one_solution(input: &[Vec<Map>], slope: (usize, usize)) -> u32 {
    count_trees_on_slope(input, &slope)
}

/// Evaluates the product of tree counts for multiple slopes
pub fn puzzle_two_solution(input: &[Vec<Map>], slopes: &[(usize, usize)]) -> u32 {
    slopes.iter()
        .fold(1, |acc, slope| {
            acc * count_trees_on_slope(input, slope)
        })
}

fn main() {
    let input = parse_input("./puzzle_input.txt");

    println!("Puzzle 1 Solution -> {}", puzzle_one_solution(&input, (3, 1)));
    println!("Puzzle 2 Solution -> {}", puzzle_two_solution(&input, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_input() -> Vec<Vec<Map>> {
        let input = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

        input
            .split('\n')
            .map(|line| {
                let line = line.trim();
                line.chars()
                    .map(|c| if c == '.' { Map::Open } else { Map::Tree })
                    .collect::<Vec<Map>>()
            })
            .collect::<Vec<Vec<Map>>>()
    }
    #[test]
    fn test_puzzle_one_example_input() {
        let input = example_input();
        let slope = (3, 1);

        assert_eq!(puzzle_one_solution(&input, slope), 7);
    }

    #[test]
    fn test_puzzle_one_different_slopes() {
        let input = example_input();

        assert_eq!(puzzle_one_solution(&input, (1, 1)), 2);
        assert_eq!(puzzle_one_solution(&input, (5, 1)), 3);
        assert_eq!(puzzle_one_solution(&input, (7, 1)), 4);
        assert_eq!(puzzle_one_solution(&input, (1, 2)), 2);
    }

    #[test]
    fn test_puzzle_two_example_input() {
        let input = example_input();
        let slopes = vec![
            (1, 1),
            (3, 1),
            (5, 1),
            (7, 1),
            (1, 2),
        ];

        assert_eq!(puzzle_two_solution(&input, &slopes), 336)
    }
}
