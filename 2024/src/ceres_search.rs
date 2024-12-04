use crate::utils::Args;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run_part1() {
    let args = Args::parse();
    let word = "XMAS";
    let count = find_word(&args.file_path, word);
    println!("The word '{}' appears {} times in the grid.", word, count);
}

fn read_grid(file_path: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(file_path).expect("Failed to read the input file");

    content.lines().map(|line| line.chars().collect()).collect()
}

fn find_word(file_path: &str, word: &str) -> usize {
    let grid = read_grid(file_path);
    let rows = grid.len();
    let cols = grid[0].len();
    let word_len = word.len();
    let word_chars: Vec<char> = word.chars().collect();

    let mut count = 0;

    // Directions for traversal: right, left, down, up, diagonals
    let directions = vec![
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // diagonal down-right
        (1, -1),  // diagonal down-left
        (-1, 1),  // diagonal up-right
        (-1, -1), // diagonal up-left
    ];

    // Iterate over every cell in the grid
    for row in 0..rows {
        for col in 0..cols {
            // Check all directions
            for &(dr, dc) in &directions {
                let mut match_count = 0;

                // Check if the word fits in the current direction
                for k in 0..word_len {
                    let nr = row as isize + k as isize * dr;
                    let nc = col as isize + k as isize * dc;

                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        break;
                    }

                    if grid[nr as usize][nc as usize] == word_chars[k] {
                        match_count += 1;
                    } else {
                        break;
                    }
                }

                if match_count == word_len {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word() {
        let input_path = "input/4-ceres-search/test-input-1.txt";
        let word = "XMAS";
        let expected_count = 18;
        let result = find_word(input_path, word);

        assert_eq!(result, expected_count);
    }
}
