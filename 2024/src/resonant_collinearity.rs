use crate::utils::Args;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_antenna_map(input: &str) -> (HashMap<char, Vec<(usize, usize)>>, usize, usize) {
    let mut antenna_map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut grid_width = 0;
    let mut grid_height = 0;

    for (y, line) in input.lines().enumerate() {
        grid_height = y + 1;
        if y == 0 {
            grid_width = line.len();
        }
        for (x, ch) in line.chars().enumerate() {
            if ch != '.' {
                antenna_map.entry(ch).or_default().push((x, y));
            }
        }
    }

    (antenna_map, grid_width, grid_height)
}

fn find_antinodes(
    antenna_map: &HashMap<char, Vec<(usize, usize)>>,
    grid_width: usize,
    grid_height: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    // Iterate through each frequency group
    for (_, positions) in antenna_map.iter() {
        // Check every pair of antennas with the same frequency
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];

                // Compute direction vector
                let dx = x2 as isize - x1 as isize;
                let dy = y2 as isize - y1 as isize;

                // Calculate Antinode 1: A + 2 * AB
                let antinode1_x = x1 as isize + 2 * dx;
                let antinode1_y = y1 as isize + 2 * dy;

                // Calculate Antinode 2: B - 2 * AB
                let antinode2_x = x2 as isize - 2 * dx;
                let antinode2_y = y2 as isize - 2 * dy;

                // Validate and add Antinode 1
                if antinode1_x >= 0
                    && antinode1_x < grid_width as isize
                    && antinode1_y >= 0
                    && antinode1_y < grid_height as isize
                {
                    antinodes.insert((antinode1_x as usize, antinode1_y as usize));
                }

                // Validate and add Antinode 2
                if antinode2_x >= 0
                    && antinode2_x < grid_width as isize
                    && antinode2_y >= 0
                    && antinode2_y < grid_height as isize
                {
                    antinodes.insert((antinode2_x as usize, antinode2_y as usize));
                }
            }
        }
    }

    // Add antinodes for singleton antennas
    for (_, positions) in antenna_map.iter() {
        if positions.len() == 1 {
            let (x, y) = positions[0];
            antinodes.insert((x, y));
        }
    }

    antinodes
}

pub fn get_signal_impact(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Unable to read file");
    let (antenna_map, grid_width, grid_height) = parse_antenna_map(&input);
    let antinodes = find_antinodes(&antenna_map, grid_width, grid_height);

    antinodes.len()
}

pub fn run_part1() {
    let args = Args::parse();
    let result = get_signal_impact(&args.file_path);
    println!("Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_signal_impact() {
        let result = get_signal_impact("input/8-resonant-collinearity/test-input-1.txt");
        assert_eq!(result, 14);
    }
}
