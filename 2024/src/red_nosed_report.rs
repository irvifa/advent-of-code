use crate::utils::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn is_safe_report(levels: &Vec<i32>) -> bool {
    // Check if levels are increasing
    let increasing = levels
        .windows(2)
        .all(|window| window[1] - window[0] >= 1 && window[1] - window[0] <= 3);

    // Check if levels are decreasing
    let decreasing = levels
        .windows(2)
        .all(|window| window[0] - window[1] >= 1 && window[0] - window[1] <= 3);

    increasing || decreasing
}

fn count_safe_reports(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_report(report))
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

pub fn run() {
    let args = Args::parse();
    let reports = read_reports_from_file(&args.file_path);
    let safe_reports_count = count_safe_reports(&reports);

    println!("Number of safe reports: {}", safe_reports_count);
}
