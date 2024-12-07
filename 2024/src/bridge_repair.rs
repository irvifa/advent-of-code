use crate::utils::Args;
use std::collections::{HashMap, HashSet};
use std::fs;

fn evaluate_equation(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0];
    for (i, &op) in ops.iter().enumerate() {
        match op {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            _ => panic!("Unknown operator"),
        }
    }
    result
}

fn solve_equation(test_value: i64, nums: &[i64]) -> Option<Vec<char>> {
    let num_ops = nums.len() - 1;

    // Generate all possible operator combinations
    for i in 0..(2_u32.pow(num_ops as u32)) {
        let mut ops = Vec::new();
        let mut current_combo = i;

        // Generate operators for this combination
        for _ in 0..num_ops {
            let op = if current_combo % 2 == 0 { '+' } else { '*' };
            ops.push(op);
            current_combo /= 2;
        }

        // Check if this combination works
        let result = evaluate_equation(nums, &ops);
        if result == test_value {
            return Some(ops);
        }
    }

    None
}

fn parse_line(line: &str) -> (i64, Vec<i64>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let test_value: i64 = parts[0].parse().unwrap();
    let nums: Vec<i64> = parts[1]
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    (test_value, nums)
}
fn solve_calibrations(input_path: &str) -> i64 {
    let input = fs::read_to_string(input_path).expect("Could not read file");
    input
        .lines()
        .filter_map(|line| {
            let (test_value, nums) = parse_line(line);
            solve_equation(test_value, &nums).map(|_| test_value)
        })
        .sum()
}

pub fn run_part1() {
    let args = Args::parse();
    let result = solve_calibrations(&args.file_path);
    println!("Part 1 - Calibration Result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_calibrations() {
        let result = solve_calibrations("input/7-bridge-repair/test-input-1.txt");
        assert_eq!(result, 3749);
    }
}
