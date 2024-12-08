use crate::utils::Args;
use std::collections::{HashMap, HashSet};
use std::fs;

/// Parses the input file to build the antenna map and determine grid dimensions.
fn parse_antenna_map(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize, usize) {
    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let grid_height = input.lines().count();
    let grid_width = input.lines().next().unwrap_or("").len();

    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antenna_map.entry(ch).or_default().push((x, y));
            }
        }
    }

    (antenna_map, grid_width, grid_height)
}


/// Finds GCD of two numbers (used for normalizing direction vectors).
fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// Finds unique antinodes based on the rules of Part 1.
fn find_antinodes_part1(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    grid_width: usize,
    grid_height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for positions in antenna_map.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Compute and validate antinode positions
                let candidates = [
                    (x1 as isize + 2 * dx, y1 as isize + 2 * dy),
                    (x2 as isize - 2 * dx, y2 as isize - 2 * dy),
                ];

                for (x, y) in candidates {
                    if x >= 0 && x < grid_width as isize && y >= 0 && y < grid_height as isize {
                        antinodes.insert((x as usize, y as usize));
                    }
                }
            }
        }

        // Add singleton antennas
        if positions.len() == 1 {
            antinodes.insert(positions[0]);
        }
    }

    antinodes
}

/// Finds unique antinodes based on the rules of Part 2.
fn find_antinodes_part2(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    grid_width: usize,
    grid_height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for positions in antenna_map.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                // Normalize direction vector using GCD
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;
                let divisor = gcd(dx, dy);
                let (step_x, step_y) = (dx / divisor, dy / divisor);

                // Traverse in both directions along the line
                let mut traverse = |mut x: isize, mut y: isize, step_x: isize, step_y: isize| {
                    while x >= 0 && x < grid_width as isize && y >= 0 && y < grid_height as isize {
                        antinodes.insert((x as usize, y as usize));
                        x += step_x;
                        y += step_y;
                    }
                };

                traverse(x1 as isize, y1 as isize, -step_x, -step_y);
                traverse(x1 as isize + step_x, y1 as isize + step_y, step_x, step_y);
            }

            // Add all antennas themselves as antinodes
            antinodes.insert(positions[i]);
        }
    }

    antinodes
}

/// Calculates the number of antinodes based on the provided ruleset.
fn get_signal_impact<F>(file_path: &str, find_antinodes: F) -> usize
where
    F: Fn(&HashMap<char, Vec<(usize, usize)>>, usize, usize) -> HashSet<(usize, usize)>,
{
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let (antenna_map, grid_width, grid_height) = parse_antenna_map(&input);
    find_antinodes(&antenna_map, grid_width, grid_height).len()
}

/// Part 1 entry point
pub fn run_part1() {
    let args = Args::parse();
    let result = get_signal_impact(&args.file_path, find_antinodes_part1);
    println!("Part 1: {}", result);
}

/// Part 2 entry point
pub fn run_part2() {
    let args = Args::parse();
    let result = get_signal_impact(&args.file_path, find_antinodes_part2);
    println!("Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = get_signal_impact("input/8-resonant-collinearity/test-input-1.txt", find_antinodes_part1);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part2() {
        let result = get_signal_impact("input/8-resonant-collinearity/test-input-2.txt", find_antinodes_part2);
        assert_eq!(result, 34);
    }
}
