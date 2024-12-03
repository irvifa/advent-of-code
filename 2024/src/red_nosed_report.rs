use crate::utils::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Checks if a report is safe by verifying that levels are either consistently
/// increasing or consistently decreasing with a difference of 1 to 3.
fn is_safe_report(levels: &Vec<i32>) -> bool {
    let increasing = levels
        .windows(2)
        .all(|window| window[1] - window[0] >= 1 && window[1] - window[0] <= 3);

    let decreasing = levels
        .windows(2)
        .all(|window| window[0] - window[1] >= 1 && window[0] - window[1] <= 3);

    increasing || decreasing
}

/// Handles the Problem Dampener by checking if removing one level makes the report safe.
fn is_safe_with_dampener(levels: &Vec<i32>) -> bool {
    if is_safe_report(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);

        if is_safe_report(&modified_levels) {
            return true;
        }
    }

    false
}

/// Counts the number of safe reports in the provided list for Part 1.
fn count_safe_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count()
}

/// Counts the number of safe reports in the provided list for Part 2.
fn count_safe_reports_with_dampener(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_with_dampener(report))
        .count()
}

fn read_reports_from_file(filename: &str) -> Vec<Vec<i32>> {
    let path = Path::new(filename);
    let file = File::open(&path).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("Failed to parse number"))
                .collect()
        })
        .collect()
}

pub fn run_part1() {
    let args = Args::parse();
    let reports = read_reports_from_file(&args.file_path);
    let safe_reports_count = count_safe_reports(&reports);

    println!("Part 1 - Number of safe reports: {}", safe_reports_count);
}

pub fn run_part2() {
    let args = Args::parse();
    let reports = read_reports_from_file(&args.file_path);
    let safe_reports_count = count_safe_reports_with_dampener(&reports);

    println!(
        "Part 2 - Number of safe reports with Problem Dampener: {}",
        safe_reports_count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_red_nosed_safe_reports_count() {
        let input_path = "input/2-red_nosed_report/test-input-1.txt";
        let reports = read_reports_from_file(input_path);
        let safe_reports_count = count_safe_reports(&reports);

        let expected_safe_reports_count = 2;
        assert_eq!(safe_reports_count, expected_safe_reports_count);
    }

    #[test]
    fn test_red_nosed_safe_reports_with_dampener_count() {
        let input_path = "input/2-red_nosed_report/test-input-2.txt";
        let reports = read_reports_from_file(input_path);
        let safe_reports_count = count_safe_reports_with_dampener(&reports);

        let expected_safe_reports_count = 4;
        assert_eq!(safe_reports_count, expected_safe_reports_count);
    }
}
