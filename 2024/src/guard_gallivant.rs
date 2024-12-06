use crate::utils::Args;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    let original_map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();

    let (guard_row, guard_col, direction) = find_initial_guard_state(&original_map);
    let mut map = original_map.clone();

    // Remove initial guard marker
    map[guard_row][guard_col] = '.';

    let visited_positions = run_simulation(&map, guard_row, guard_col, direction);

    visited_positions.len()
}

fn run_simulation(
    map: &Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    start_dir: Direction,
) -> HashSet<(usize, usize)> {
    let mut visited_positions = HashSet::new();
    visited_positions.insert((start_row, start_col));

    let mut guard_row = start_row;
    let mut guard_col = start_col;
    let mut direction = start_dir;

    // A safety limit to prevent infinite loops if something is off.
    // This is just a large number to prevent accidental infinite loops in this simulation
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

        // Check out-of-bounds
        if next_row >= map.len() || next_col >= map[0].len() {
            // Guard leaves the map
            break;
        }

        // Check if blocked
        if map[next_row][next_col] == '#' {
            // Turn right
            direction = direction.turn_right();
        } else {
            // Move forward
            guard_row = next_row;
            guard_col = next_col;
            visited_positions.insert((guard_row, guard_col));
        }
    }

    visited_positions
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

/// Runs a simulation with a single added obstruction and checks for loops.
/// Returns true if adding the obstruction at `obstacle_pos` causes a loop.
fn causes_loop(
    mut map: Vec<Vec<char>>,
    start_row: usize,
    start_col: usize,
    start_dir: Direction,
    obstacle_pos: (usize, usize),
) -> bool {
    // Place the obstruction
    map[obstacle_pos.0][obstacle_pos.1] = '#';

    let mut guard_row = start_row;
    let mut guard_col = start_col;
    let mut direction = start_dir;

    // Track visited states of (row, col, direction)
    // If we ever see a state twice, we have a loop
    let mut visited_states = HashSet::new();
    visited_states.insert((guard_row, guard_col, direction));

    let max_steps = map.len() * map[0].len() * 10; // Arbitrary large number
    let mut steps = 0;

    while steps < max_steps {
        steps += 1;

        let (next_row, next_col) = match direction {
            Direction::North => (guard_row.wrapping_sub(1), guard_col),
            Direction::East => (guard_row, guard_col + 1),
            Direction::South => (guard_row + 1, guard_col),
            Direction::West => (guard_row, guard_col.wrapping_sub(1)),
        };

        // Check out-of-bounds
        if next_row >= map.len() || next_col >= map[0].len() {
            // Guard leaves the map, no loop
            return false;
        }

        if map[next_row][next_col] == '#' {
            // Turn right
            direction = direction.turn_right();
        } else {
            // Move forward
            guard_row = next_row;
            guard_col = next_col;
        }

        let state = (guard_row, guard_col, direction);
        if visited_states.contains(&state) {
            // Found a loop
            return true;
        } else {
            visited_states.insert(state);
        }
    }

    // If we reached this many steps without escaping and no repeated state found,
    // it's likely safe to say no loop.
    // (The large max_steps is to prevent false positives.)
    false
}

pub fn run_part1() {
    let args = Args::parse();
    let visited_positions = simulate_guard_path(&args.file_path);
    println!("Part 1 - {}", visited_positions);
}

fn patrol_routes(file_path: &str) -> usize {
    let map_str = fs::read_to_string(file_path).expect("Could not read input file");
    let original_map: Vec<Vec<char>> = map_str.lines().map(|line| line.chars().collect()).collect();

    let (guard_row, guard_col, guard_dir) = find_initial_guard_state(&original_map);
    let mut map = original_map.clone();
    // Remove guard marker
    map[guard_row][guard_col] = '.';

    let mut possible_positions = 0;

    // We must not place obstruction at the guard's starting position
    // The guard start position is (guard_row, guard_col)

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            // Check if this position is a free space and not the guard's start
            if (r, c) != (guard_row, guard_col) && map[r][c] == '.' {
                // Try placing an obstruction and see if it causes a loop
                if causes_loop(map.clone(), guard_row, guard_col, guard_dir, (r, c)) {
                    possible_positions += 1;
                }
            }
        }
    }
    possible_positions
}

pub fn run_part2() {
    let args = Args::parse();
    let possible_positions = patrol_routes(&args.file_path);
    println!("Part 2 - {}", possible_positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_patrol_example() {
        let visited_positions = simulate_guard_path("input/6-guard-gallivant/test-input-1.txt");
        assert_eq!(visited_positions, 41);
    }

    #[test]
    fn test_part_two_example() {
        let count = patrol_routes("input/6-guard-gallivant/test-input-1.txt");
        assert_eq!(count, 6);
    }
}
