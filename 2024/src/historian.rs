use crate::utils::Args;
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

pub fn run() {
    let args = Args::parse();
    let (mut left_list, mut right_list) = read_lists_from_file(&args.file_path);

    sort_lists(&mut left_list, &mut right_list);
    let total_distance = calculate_total_distance(&left_list, &right_list);

    println!("Total distance: {}", total_distance);
}
