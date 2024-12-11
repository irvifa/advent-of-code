use crate::utils::Args;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum PuzzlePart {
    Part1,
    Part2,
}

pub fn run_part1() {
    let args = Args::parse();
    let input_path = Path::new(&args.file_path);
    run(PuzzlePart::Part1, input_path);
}

/// Entrypoint for handling both parts.
/// Reads the initial stones, applies transformations, and prints the result.
fn run(part: PuzzlePart, input_path: &Path) {
    let stones = read_stones(input_path);
    let result = match part {
        PuzzlePart::Part1 => count_stones_after_blinks(stones, 25),
        PuzzlePart::Part2 => count_stones_after_blinks(stones, 75),
    };

    println!("{}", result);
}

/// Reads stones from the given file path.
/// Each line can contain multiple numbers separated by whitespace.
/// Converts each stone into a `BigUint`.
fn read_stones(path: &Path) -> Vec<BigUint> {
    let file = File::open(path).expect("Could not open input file.");
    let reader = BufReader::new(file);

    let mut stones = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Error reading line from file.");
        for num in line.split_whitespace() {
            let val = num.parse::<BigUint>().expect("Invalid number in input");
            stones.push(val);
        }
    }
    stones
}

/// Applies the transformation rules a specified number of times and returns the final count.
fn count_stones_after_blinks(mut stones: Vec<BigUint>, blinks: u32) -> usize {
    for _ in 0..blinks {
        stones = transform_stones(&stones);
    }
    stones.len()
}

/// Transforms a slice of stones according to the three rules and returns the new set of stones.
///
/// To improve performance:
/// - We only convert to a string to determine if digits are even or odd.
/// - We do numeric splitting for even-digit stones without parsing strings again.
fn transform_stones(stones: &[BigUint]) -> Vec<BigUint> {
    let multiplier = BigUint::from(2024u64);

    stones
        .par_iter() // parallel iterator
        .flat_map(|stone| transform_stone(stone, &multiplier))
        .collect()
}

/// Transforms a single stone according to the rules:
///
/// 1. If the stone is 0, it becomes 1.
/// 2. If the stone's decimal representation has an even number of digits, split it into two numbers.
/// 3. Otherwise, multiply by 2024.
fn transform_stone(stone: &BigUint, multiplier: &BigUint) -> Vec<BigUint> {
    // 0 -> 1
    if stone.is_zero() {
        return vec![BigUint::one()];
    }

    let digits_str = stone.to_str_radix(10);
    let digits = digits_str.len();

    // even-digit split
    if digits % 2 == 0 {
        let half = digits / 2;
        let power = BigUint::from(10u64).pow(half as u32);
        let left = stone / &power;
        let right = stone % &power;
        return vec![left, right];
    }

    // multiply by 2024
    vec![stone * multiplier]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_simple_part1() {
        let path = "input/11-plutonian-pebbles/test-input-1.txt";
        let stones = read_stones(Path::new(path));

        let count = count_stones_after_blinks(stones, 25);
        // For input "125 17", the known result after 25 transformations is 55312.
        assert_eq!(count, 55312);
    }
}
