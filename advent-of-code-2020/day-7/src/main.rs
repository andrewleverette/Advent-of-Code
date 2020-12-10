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

/// Parses input into a HashMap where the 
/// key is the bag color and the values are vectors
/// of tuples that contain the bag color and count.
fn parse_input(filename: &str) -> HashMap<String, Vec<(String, u32)>> {
    let mut rules = HashMap::new();
    
    match read_lines(filename) {
        Ok(lines) => {
            for line in lines {
                let line = line.unwrap();

                let rule: Vec<&str> = line.split(" bags contain ").collect();
                let outer_bag = rule[0].to_owned();
                let inner_bags = {
                    if rule[1].contains("no") {
                        vec![]
                    } else {
                        let bag_rules: Vec<&str> = rule[1].split(", ").collect();

                        let mut values = Vec::new();
                        for bag_rule in bag_rules {
                            let bag_rule: Vec<&str> = bag_rule.split(' ').collect();
                            
                            let bag_count = bag_rule[0].parse::<u32>().unwrap();
                            let bag_name = format!("{} {}", bag_rule[1], bag_rule[2]);

                            values.push((bag_name.to_owned(), bag_count));
                        }
                        values.to_owned()
                    }
                };
                rules.insert(outer_bag, inner_bags);
            }
        },
        Err(e) => panic!("Error reading file: {}", e),
    }

    rules
}

/// Recursively finds out if the `target` bag could be nested
/// in at least one other bag
fn can_contain_bag(target: &String, key: &String, rules: &HashMap<String, Vec<(String, u32)>>) -> bool {
    if let Some(rule) = rules.get(key) {
        if rule.is_empty() {
            return false;
        }

        if rule.iter().any(|(bag, _)| bag == target) {
            return true;
        }

        for (bag, _) in rule {
           if can_contain_bag(target, bag, rules) {
               return true;
           }
        }

        false
    } else {
        false
    }
}

/// Recursively calculates the total number of bags that are required
/// to cary the `target` bag
fn count_nested_bags(target: &String, rules: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    if let Some(rule) = rules.get(target) {
        if rule.is_empty() {
            0
        } else {
            let mut bag_count = 0;
            for (bag, count) in rule {
                bag_count += count + count * count_nested_bags(bag, rules);
            }
            bag_count
        }
    } else {
        0
    }
}

/// This solution finds out how many bags could contain a the target bag
fn puzzle_one_solution(input: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let target = "shiny gold".to_owned();
    let mut options = 0;

    for key in input.keys() {
        if can_contain_bag(&target, key, input) {
            options += 1;
        }
    }

    options
}

/// This solution calculates exactly how many bags are required given a target
fn puzzle_two_solution(input: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let target = "shiny gold".to_owned();
    count_nested_bags(&target, input)
}

fn main() {
    let input = parse_input("./puzzle_input.txt");

    println!("Puzzle 1 Solution -> {}", puzzle_one_solution(&input));
    println!("Puzzle 2 Solution -> {}", puzzle_two_solution(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> HashMap<String, Vec<(String, u32)>> {
        let mut rules = HashMap::new();

        rules.insert("light red".to_owned(), vec![("bright white".to_owned(), 1), ("muted yellow".to_owned(), 2)]);
        rules.insert("dark orange".to_owned(), vec![("bright white".to_owned(), 3), ("muted yellow".to_owned(), 4)]);
        rules.insert("bright white".to_owned(), vec![("shiny gold".to_owned(), 1)]);
        rules.insert("muted yellow".to_owned(), vec![("shiny gold".to_owned(), 2), ("faded blue".to_owned(), 9)]);
        rules.insert("shiny gold".to_owned(), vec![("dark olive".to_owned(), 1), ("vibrant plum".to_owned(), 2)]);
        rules.insert("dark olive".to_owned(), vec![("faded blue".to_owned(), 3), ("dotted black".to_owned(), 4)]);
        rules.insert("vibrant plum".to_owned(), vec![("faded blue".to_owned(), 5), ("dotted black".to_owned(), 6)]);
        rules.insert("faded blue".to_owned(), vec![]);
        rules.insert("dotted black".to_owned(), vec![]);


        rules
    }

    #[test]
    fn test_puzzle_one_example() {
        let input = get_example_input();

        assert_eq!(puzzle_one_solution(&input), 4)
    }

    #[test]
    fn test_puzzle_two_example() {
        let input = get_example_input();

        assert_eq!(puzzle_two_solution(&input), 32);
    }
}
