use std::collections::HashSet;
use std::fs;
use crate::utils::Args;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn simulate_guard_path(file_path: &str) -> usize {
    let map_str = fs::read_to_string(file_path).expect("Could not read input file");
    let map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();

    let (mut guard_row, mut guard_col, mut direction) = find_initial_guard_state(&map);
    let mut map = map.clone();

    // Remove initial guard marker
    map[guard_row][guard_col] = '.';

    let mut visited_positions = HashSet::new();
    visited_positions.insert((guard_row, guard_col));

    // A safety limit to prevent infinite loops if something is off
    let max_steps = map.len() * map[0].len() * 4;
    let mut steps = 0;

    while steps < max_steps {
        steps += 1;

        let (next_row, next_col) = match direction {
            Direction::North => (guard_row.wrapping_sub(1), guard_col),
            Direction::East => (guard_row, guard_col + 1),
            Direction::South => (guard_row + 1, guard_col),
            Direction::West => (guard_row, guard_col.wrapping_sub(1)),
        };

        // Check if the next position is out-of-bounds before moving
        if next_row >= map.len() || next_col >= map[0].len() {
            // The guard leaves the mapped area without counting an out-of-bounds position
            break;
        }

        // Check if the next position is blocked
        if map[next_row][next_col] == '#' {
            // Turn right if blocked
            direction = direction.turn_right();
        } else {
            // Safe to move forward
            guard_row = next_row;
            guard_col = next_col;
            visited_positions.insert((guard_row, guard_col));
        }
    }

    visited_positions.len()
}

fn find_initial_guard_state(map: &[Vec<char>]) -> (usize, usize, Direction) {
    for (row, line) in map.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            match cell {
                '^' => return (row, col, Direction::North),
                '>' => return (row, col, Direction::East),
                'v' => return (row, col, Direction::South),
                '<' => return (row, col, Direction::West),
                _ => {}
            }
        }
    }
    panic!("No guard found in the map")
}

pub fn run_part1() {
    let args = Args::parse();
    let visited_positions = simulate_guard_path(&args.file_path);
    println!("Part 1 - {}", visited_positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_patrol_example() {
        // Ensure that "input/6-guard-gallivant/test-input-1.txt" matches the example map.
        let visited_positions = simulate_guard_path("input/6-guard-gallivant/test-input-1.txt");
        assert_eq!(visited_positions, 41);
    }
}
