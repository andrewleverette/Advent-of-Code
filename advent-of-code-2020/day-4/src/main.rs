use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lazy_static::lazy_static;
use regex::Regex;

/// Creates an iterator over the lines of the puzzle input
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Parses input into a vector of vector of tuples
fn parse_input(filename: &str) -> Vec<Vec<(String, String)>> {
    let mut results = Vec::new();

    match read_lines(filename) {
        Ok(lines) => {
            let mut fields: Vec<(String, String)> = Vec::new();
            for line in lines {
                let line = line.unwrap();

                if line == "" {
                    results.push(fields.to_owned());
                    fields = Vec::new();
                } else {
                    let chunk: Vec<String> = line.split(' ').map(|s| s.to_owned()).collect();
                    for pair in chunk {
                        let pair: Vec<&str> = pair.split(':').collect();
                        fields.push((pair[0].to_owned(), pair[1].to_owned()))
                    }
                }
            }

            results.push(fields.to_owned())
        },
        Err(e) => panic!("Error reading file: {}", e),
    }

    results
}

/// Tests if a function is valid based on the number of fields
/// If the strict parameter is set to true, then it validates the fields as well
fn is_passport_valid(fields: &[(String, String)], strict: bool) -> bool {
    if fields.len() < 7 {
        return false;
    }

    if strict {
        count_fields(fields) == 7 && validate_fields(fields)
    } else {
        count_fields(fields) == 7
    }
}

/// Simple counts the number of fields excluding "cid"
fn count_fields(fields: &[(String, String)]) -> i32 {
    let mut validation_count = 0;

    for (key, _) in fields {
        if key == "cid" {
            continue;
        } else {
            validation_count += 1;
        }
    }

    validation_count
}

/// Validates the data to ensure that it matches the requirements
fn validate_fields(fields: &[(String, String)]) -> bool {
    lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref ECL_REGEX: Regex =Regex::new(r"^amb$|^blu$|^brn$|^gry$|^grn$|^hzl$|^oth$").unwrap();
        static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }

    for (key, value) in fields {
        match key.as_ref() {
            "byr" => {
                let year = value.parse::<u16>().unwrap();
                if year < 1920 || year > 2002 {
                    return false;
                }
            },
            "iyr" => {
                let year = value.parse::<u16>().unwrap();
                if year < 2010 || year > 2020 {
                    return false;
                }
            },
            "eyr" => {
                let year = value.parse::<u16>().unwrap();
                if year < 2020 || year > 2030 {
                    return false;
                }
            },
            "hgt" => {
                if !(value.contains("cm") || value.contains("in")) {
                    return false;
                }

                let unit = &value[value.len() - 2..];
                let height = value[..value.len() - 2].parse::<u8>().unwrap();

                if unit == "cm" && (height < 150 || height > 193) {
                    return false;
                }

                if unit == "in" && (height < 59 || height > 76) {
                    return false
                }
            },
            "hcl" => {
                if !HCL_REGEX.is_match(value) {
                    return false;
                }
            },
            "ecl" => {
                if !ECL_REGEX.is_match(value) {
                    return false;
                }
            },
            "pid" => {
                if !PID_REGEX.is_match(value) {
                    return false;
                }
            },
            _ => {
                continue;
            }
        }
    }

    true
}

pub fn puzzle_one_solution(batch: &[Vec<(String, String)>]) -> i32 {
    let mut valid_count = 0;

    for fields in batch {
        if is_passport_valid(fields, false) {
            valid_count += 1;
        }
    }

    valid_count
}

pub fn puzzle_two_solution(batch: &[Vec<(String, String)>]) -> i32 {
    let mut valid_count = 0;

    for fields in batch {
        if is_passport_valid(fields, true) {
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

    fn get_example_input() -> Vec<Vec<(String, String)>> {
        vec![
            vec![("ecl".to_owned(), "gry".to_owned()), ("pid".to_owned(), "860033327".to_owned()), ("eyr".to_owned(), "2020".to_owned()), ("hcl".to_owned(), "#fffffd".to_owned()), ("byr".to_owned(), "1937".to_owned()), ("iyr".to_owned(), "2017".to_owned()), ("cid".to_owned(), "147".to_owned()), ("hgt".to_owned(), "183cm".to_owned())], 
            vec![("iyr".to_owned(), "2013".to_owned()), ("ecl".to_owned(), "amb".to_owned()), ("cid".to_owned(), "350".to_owned()), ("eyr".to_owned(), "2023".to_owned()), ("pid".to_owned(), "028048884".to_owned()), ("hcl".to_owned(), "#cfa07d".to_owned()), ("byr".to_owned(), "1929".to_owned())], 
            vec![("hcl".to_owned(), "#ae17e1".to_owned()), ("iyr".to_owned(), "2013".to_owned()), ("eyr".to_owned(), "2024".to_owned()), ("ecl".to_owned(), "brn".to_owned()), ("pid".to_owned(), "760753108".to_owned()), ("byr".to_owned(), "1931".to_owned()), ("hgt".to_owned(), "179cm".to_owned())], 
            vec![("hcl".to_owned(), "#cfa07d".to_owned()), ("eyr".to_owned(), "2025".to_owned()), ("pid".to_owned(), "166559648".to_owned()), ("iyr".to_owned(), "2011".to_owned()), ("ecl".to_owned(), "brn".to_owned()), ("hgt".to_owned(), "59in".to_owned())]
        ]
    }

    fn get_example_invalid_input() -> Vec<Vec<(String, String)>> {
        vec![
            vec![("eyr".to_owned(), "1972".to_owned()), ("cid".to_owned(), "100".to_owned()), ("hcl".to_owned(), "#18171d".to_owned()), ("ecl".to_owned(), "amb".to_owned()), ("hgt".to_owned(), "170".to_owned()), ("pid".to_owned(), "186cm".to_owned()), ("iyr".to_owned(), "2018".to_owned()), ("byr".to_owned(), "1926".to_owned())], 
            vec![("iyr".to_owned(), "2019".to_owned()), ("hcl".to_owned(), "#602927".to_owned()), ("eyr".to_owned(), "1967".to_owned()), ("hgt".to_owned(), "170cm".to_owned()), ("ecl".to_owned(), "grn".to_owned()), ("pid".to_owned(), "012533040".to_owned()), ("byr".to_owned(), "1946".to_owned())], 
            vec![("hcl".to_owned(), "dab227".to_owned()), ("iyr".to_owned(), "2012".to_owned()), ("ecl".to_owned(), "brn".to_owned()), ("hgt".to_owned(), "182cm".to_owned()), ("pid".to_owned(), "021572410".to_owned()), ("eyr".to_owned(), "2020".to_owned()), ("byr".to_owned(), "1992".to_owned()), ("cid".to_owned(), "277".to_owned())], 
            vec![("hgt".to_owned(), "59cm".to_owned()), ("ecl".to_owned(), "zzz".to_owned()), ("eyr".to_owned(), "2038".to_owned()), ("hcl".to_owned(), "74454a".to_owned()), ("iyr".to_owned(), "2023".to_owned()), ("pid".to_owned(), "3556412378".to_owned()), ("byr".to_owned(), "2007".to_owned())]]
    }

    fn get_example_all_valid_input() -> Vec<Vec<(String, String)>> {
        vec![
            vec![("pid".to_owned(), "087499704".to_owned()), ("hgt".to_owned(), "74in".to_owned()), ("ecl".to_owned(), "grn".to_owned()), ("iyr".to_owned(), "2012".to_owned()), ("eyr".to_owned(), "2030".to_owned()), ("byr".to_owned(), "1980".to_owned()), ("hcl".to_owned(), "#623a2f".to_owned())], 
            vec![("eyr".to_owned(), "2029".to_owned()), ("ecl".to_owned(), "blu".to_owned()), ("cid".to_owned(), "129".to_owned()), ("byr".to_owned(), "1989".to_owned()), ("iyr".to_owned(), "2014".to_owned()), ("pid".to_owned(), "896056539".to_owned()), ("hcl".to_owned(), "#a97842".to_owned()), ("hgt".to_owned(), "165cm".to_owned())], 
            vec![("hcl".to_owned(), "#888785".to_owned()), ("hgt".to_owned(), "164cm".to_owned()), ("byr".to_owned(), "2001".to_owned()), ("iyr".to_owned(), "2015".to_owned()), ("cid".to_owned(), "88".to_owned()), ("pid".to_owned(), "545766238".to_owned()), ("ecl".to_owned(), "hzl".to_owned()), ("eyr".to_owned(), "2022".to_owned())], 
            vec![("iyr".to_owned(), "2010".to_owned()), ("hgt".to_owned(), "158cm".to_owned()), ("hcl".to_owned(), "#b6652a".to_owned()), ("ecl".to_owned(), "blu".to_owned()), ("byr".to_owned(), "1944".to_owned()), ("eyr".to_owned(), "2021".to_owned()), ("pid".to_owned(), "093154719".to_owned())]
        ]
    }
    #[test]
    fn test_puzzle_one_example() {
        let input = get_example_input();

        assert_eq!(puzzle_one_solution(&input), 2);
    }

    #[test]
    fn test_puzzle_two_invalid_input() {
        let input = get_example_invalid_input();

        assert_eq!(puzzle_two_solution(&input), 0);
    }

    #[test]
    fn test_puzzle_two_all_valid_input() {
        let input = get_example_all_valid_input();

        assert_eq!(puzzle_two_solution(&input), 4);
    }
}