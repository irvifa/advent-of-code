use crate::utils::Args;
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

pub fn run_part2() {
    let args = Args::parse();
    let input_path = Path::new(&args.file_path);
    run(PuzzlePart::Part2, input_path);
}

fn run(part: PuzzlePart, input_path: &Path) {
    let map = read_map(input_path);

    let trailheads = find_trailheads(&map);

    match part {
        PuzzlePart::Part1 => {
            // For each trailhead, calculate how many '9' tiles are reachable via valid hiking trails (Part 1 score)
            let mut total_score = 0;
            for &start_pos in &trailheads {
                let score = calculate_trailhead_score(&map, start_pos);
                total_score += score;
            }
            println!("{}", total_score);
        }
        PuzzlePart::Part2 => {
            // For Part 2, calculate the trailhead rating (distinct number of hiking trails)
            let mut memo = vec![vec![None; map[0].len()]; map.len()];
            let mut total_rating = 0;
            for &start_pos in &trailheads {
                let rating = calculate_trailhead_rating(&map, start_pos, &mut memo);
                total_rating += rating;
            }
            println!("{}", total_rating);
        }
    }
}

fn read_map(path: &std::path::Path) -> Vec<Vec<u8>> {
    let file = File::open(path).expect("Failed to open input file");
    let reader = BufReader::new(file);

    let mut map = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.expect("Failed to read line");
        let trimmed_line = line.trim_matches('\u{feff}').trim();

        // If the line is empty, skip it
        if trimmed_line.is_empty() {
            continue;
        }

        let row = trimmed_line
            .chars()
            .map(|c| {
                if let Some(digit) = c.to_digit(10) {
                    digit as u8
                } else {
                    panic!(
                        "Non-numeric height found in line: {:?}. Character: {:?}",
                        trimmed_line, c
                    );
                }
            })
            .collect::<Vec<u8>>();

        map.push(row);
    }

    map
}

/// Find all trailheads in the map.
/// A trailhead is defined as any cell with height '0'.
fn find_trailheads(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (r, row) in map.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((r, c));
            }
        }
    }
    trailheads
}

/// Part 1: Calculate the score of a given trailhead.
/// The score is the number of distinct '9' tiles reachable from the start position
/// via valid hiking trails (paths that ascend by exactly 1 at each step, no diagonals).
fn calculate_trailhead_score(map: &[Vec<u8>], start_pos: (usize, usize)) -> usize {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut found_nines = std::collections::HashSet::new();

    dfs_trail_search_helper(
        map,
        start_pos,
        map[start_pos.0][start_pos.1],
        &mut visited,
        &mut found_nines,
    );
    found_nines.len()
}

/// Part 2: Calculate the rating of a given trailhead.
/// The rating is the number of distinct hiking trails that start at this trailhead.
/// This requires counting all unique paths that lead from height=0 to height=9.
fn calculate_trailhead_rating(
    map: &[Vec<u8>],
    start: (usize, usize),
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    // Implement the logic as discussed for Part 2:
    // 1. If at height 9, return 1.
    // 2. Otherwise, sum the distinct trails from all next-step neighbors (height+1).
    // Use memoization to avoid recalculating.
    //
    // Below is a simplified placeholder. Fill in full logic based on your solution for Part 2.
    count_distinct_trails(map, start, memo)
}

/// Counts the number of distinct trails from a given cell.
/// Memoized to avoid expensive recomputation.
fn count_distinct_trails(
    map: &[Vec<u8>],
    pos: (usize, usize),
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(val) = memo[pos.0][pos.1] {
        return val;
    }

    let current_height = map[pos.0][pos.1];
    if current_height == 9 {
        memo[pos.0][pos.1] = Some(1);
        return 1;
    }

    let directions = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    let mut total_trails = 0usize;
    for &(dr, dc) in &directions {
        let nr = pos.0 as i32 + dr;
        let nc = pos.1 as i32 + dc;
        if nr < 0 || nc < 0 || nr as usize >= map.len() || nc as usize >= map[0].len() {
            continue;
        }
        let nr_usize = nr as usize;
        let nc_usize = nc as usize;
        if map[nr_usize][nc_usize] == current_height + 1 {
            total_trails += count_distinct_trails(map, (nr_usize, nc_usize), memo);
        }
    }

    memo[pos.0][pos.1] = Some(total_trails);
    total_trails
}

fn dfs_trail_search_helper(
    map: &[Vec<u8>],
    pos: (usize, usize),
    current_height: u8,
    visited: &mut Vec<Vec<bool>>,
    found_nines: &mut std::collections::HashSet<(usize, usize)>,
) {
    visited[pos.0][pos.1] = true;

    if current_height == 9 {
        found_nines.insert(pos);
        return;
    }

    let directions = [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)];
    for &(dr, dc) in &directions {
        let new_r = pos.0 as i32 + dr;
        let new_c = pos.1 as i32 + dc;

        if new_r < 0 || new_c < 0 || new_r as usize >= map.len() || new_c as usize >= map[0].len() {
            continue; // Out of bounds
        }

        let new_r_usize = new_r as usize;
        let new_c_usize = new_c as usize;

        if visited[new_r_usize][new_c_usize] {
            continue; // Already visited
        }

        let next_height = map[new_r_usize][new_c_usize];
        if next_height == current_height + 1 {
            dfs_trail_search_helper(
                map,
                (new_r_usize, new_c_usize),
                next_height,
                visited,
                found_nines,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_part1() {
        let path = "input/10-hoof-it/test-input-1-simple-rating-1.txt";
        let file = std::fs::File::open(path).expect("Failed to open test input file");
        let reader = std::io::BufReader::new(file);

        let map = reader
            .lines()
            .map(|line_result| {
                let line = line_result.expect("Failed to read line from test input");
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            255
                        } else {
                            c.to_digit(10).expect("Non-numeric height in test input") as u8
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let trailheads = find_trailheads(&map);
        assert_eq!(trailheads.len(), 1);
        let trailhead = trailheads[0];

        let score = calculate_trailhead_score(&map, trailhead);
        assert_eq!(score, 1);
    }

    #[test]
    fn test_complex_part1() {
        let path = "input/10-hoof-it/test-input-1-complex-rating-1.txt";
        let file = std::fs::File::open(path).expect("Failed to open test input file");
        let reader = std::io::BufReader::new(file);

        let map = reader
            .lines()
            .map(|line_result| {
                let line = line_result.expect("Failed to read line from test input");
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            255
                        } else {
                            c.to_digit(10).expect("Non-numeric height in test input") as u8
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let trailheads = find_trailheads(&map);
        assert_eq!(trailheads.len(), 1);

        let score = calculate_trailhead_score(&map, trailheads[0]);
        assert_eq!(score, 2);
    }

    #[test]
    fn test_simple_part2_rating_3() {
        let path = "input/10-hoof-it/test-input-2-rating-3.txt";
        let file = std::fs::File::open(path).expect("Failed to open test input file");
        let reader = std::io::BufReader::new(file);

        let map = reader
            .lines()
            .map(|line_result| {
                let line = line_result.expect("Failed to read line from test input");
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            255
                        } else {
                            c.to_digit(10).expect("Non-numeric height in test input") as u8
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let trailheads = find_trailheads(&map);
        assert_eq!(trailheads.len(), 1);

        let mut memo = vec![vec![None; map[0].len()]; map.len()];

        let rating = calculate_trailhead_rating(&map, trailheads[0], &mut memo);
        assert_eq!(rating, 3);
    }

    #[test]
    fn test_simple_part2_rating_13() {
        let path = "input/10-hoof-it/test-input-2-rating-13.txt";
        let file = std::fs::File::open(path).expect("Failed to open test input file");
        let reader = std::io::BufReader::new(file);

        let map = reader
            .lines()
            .map(|line_result| {
                let line = line_result.expect("Failed to read line from test input");
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            255
                        } else {
                            c.to_digit(10).expect("Non-numeric height in test input") as u8
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let trailheads = find_trailheads(&map);
        assert_eq!(trailheads.len(), 1); // Only one '0' again

        let mut memo = vec![vec![None; map[0].len()]; map.len()];

        let rating = calculate_trailhead_rating(&map, trailheads[0], &mut memo);
        assert_eq!(rating, 13);
    }

    #[test]
    fn test_complex_part2_rating_227() {
        let path = "input/10-hoof-it/test-input-2-rating-227.txt";
        let file = std::fs::File::open(path).expect("Failed to open test input file");
        let reader = std::io::BufReader::new(file);

        let map = reader
            .lines()
            .map(|line_result| {
                let line = line_result.expect("Failed to read line from test input");
                line.chars()
                    .map(|c| {
                        if c == '.' {
                            255
                        } else {
                            c.to_digit(10).expect("Non-numeric height in test input") as u8
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let trailheads = find_trailheads(&map);
        assert_eq!(trailheads.len(), 1); // One '0'

        let mut memo = vec![vec![None; map[0].len()]; map.len()];

        let rating = calculate_trailhead_rating(&map, trailheads[0], &mut memo);
        assert_eq!(rating, 227);
    }
}
