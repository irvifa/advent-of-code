use crate::utils::Args;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

/// Reads the left and right lists from a file.
fn read_lists_from_file(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = fs::File::open(file_path).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line");
        let mut parts = line.split_whitespace();

        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            left_list.push(left.parse::<i32>().expect("Invalid number in left list"));
            right_list.push(right.parse::<i32>().expect("Invalid number in right list"));
        }
    }

    (left_list, right_list)
}

/// Sorts the provided lists in ascending order.
fn sort_lists(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) {
    left_list.sort();
    right_list.sort();
}

/// Calculates the total distance between the two lists.
fn calculate_total_distance(left_list: &[i32], right_list: &[i32]) -> i32 {
    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

/// Calculates the similarity score between the two lists.
fn calculate_similarity_score(left_list: &[i32], right_list: &[i32]) -> i32 {
    // Count occurrences of each number in the right list.
    let mut right_counts = HashMap::new();
    for &num in right_list {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate similarity score for the left list.
    left_list
        .iter()
        .map(|&num| num * right_counts.get(&num).unwrap_or(&0))
        .sum()
}

/// Entry point for Part 1: Total Distance Calculation.
pub fn run_part1() {
    let args = Args::parse();
    let (mut left_list, mut right_list) = read_lists_from_file(&args.file_path);

    sort_lists(&mut left_list, &mut right_list);
    let total_distance = calculate_total_distance(&left_list, &right_list);

    println!("Part 1 - Total distance: {}", total_distance);
}

/// Entry point for Part 2: Similarity Score Calculation.
pub fn run_part2() {
    let args = Args::parse();
    let (left_list, right_list) = read_lists_from_file(&args.file_path);

    let similarity_score = calculate_similarity_score(&left_list, &right_list);

    println!("Part 2 - Similarity score: {}", similarity_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_historian_total_distance() {
        let input_path = "input/1-historian/test-input-1.txt";
        let (mut left_list, mut right_list) = read_lists_from_file(input_path);

        sort_lists(&mut left_list, &mut right_list);
        let total_distance = calculate_total_distance(&left_list, &right_list);

        let expected_total_distance = 11;
        assert_eq!(total_distance, expected_total_distance);
    }

    #[test]
    fn test_historian_similarity_score() {
        let input_path = "input/1-historian/test-input-2.txt";
        let (left_list, right_list) = read_lists_from_file(input_path);

        let similarity_score = calculate_similarity_score(&left_list, &right_list);

        let expected_similarity_score = 31;
        assert_eq!(similarity_score, expected_similarity_score);
    }
}
