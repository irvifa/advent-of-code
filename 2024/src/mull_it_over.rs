use crate::utils::Args;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run_part1() {
    let args = Args::parse();
    let sum = process_memory(&args.file_path);
    println!("Sum of all valid mul instructions: {}", sum);
}

/// Processes the corrupted memory and returns the sum of valid `mul` instructions.
fn process_memory(file_path: &str) -> i32 {
    let file_content = fs::read_to_string(file_path).expect("Failed to read the input file");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile regex");

    // Borrow file_content as a &str
    re.captures_iter(&file_content)
        .map(|cap| {
            let x: i32 = cap[1].parse().expect("Failed to parse X");
            let y: i32 = cap[2].parse().expect("Failed to parse Y");
            x * y
        })
        .sum()
}

/// Entry point for Part 2: Processes valid `mul` instructions with conditional handling.
pub fn run_part2() {
    let args = Args::parse();
    let sum = process_memory_with_conditionals(&args.file_path);
    println!("Sum of all enabled mul instructions: {}", sum);
}

/// Processes the corrupted memory with conditionals and returns the sum of enabled `mul` instructions.
fn process_memory_with_conditionals(file_path: &str) -> i32 {
    let file_content = fs::read_to_string(file_path).expect("Failed to read the input file");

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Failed to compile mul regex");
    let do_re = Regex::new(r"do\(\)").expect("Failed to compile do regex");
    let dont_re = Regex::new(r"don't\(\)").expect("Failed to compile don't regex");

    let mut enabled = true; // At the beginning, `mul` instructions are enabled.
    let mut sum = 0;

    for line in file_content.split_inclusive([';', '\n', '&', '+', ')', ']']) {
        if do_re.is_match(line) {
            enabled = true;
        } else if dont_re.is_match(line) {
            enabled = false;
        }

        if enabled {
            for cap in mul_re.captures_iter(line) {
                let x: i32 = cap[1].parse().expect("Failed to parse X");
                let y: i32 = cap[2].parse().expect("Failed to parse Y");
                sum += x * y;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_memory() {
        let input_path = "input/3-mull-it-over/test-input-1.txt";
        let actual_memory = process_memory(input_path);
        let expected_memory = 161;
        assert_eq!(actual_memory, expected_memory);
    }

    #[test]
    fn test_process_memory_with_conditionals() {
        let input_path = "input/3-mull-it-over/test-input-2.txt";
        let actual_memory = process_memory_with_conditionals(input_path);
        let expected_memory = 48;
        assert_eq!(actual_memory, expected_memory);
    }
}
