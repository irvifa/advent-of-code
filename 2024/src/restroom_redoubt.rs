use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

fn parse_input(file_path: &str) -> Vec<Robot> {
    let path = Path::new(file_path);
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let position = parts[0]
                .strip_prefix("p=")
                .unwrap()
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let velocity = parts[1]
                .strip_prefix("v=")
                .unwrap()
                .split(',')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            Robot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        })
        .collect()
}

fn simulate_robots(robots: &[Robot], seconds: i64, width: i64, height: i64) -> Vec<(i64, i64)> {
    robots
        .iter()
        .map(|robot| {
            let x = (robot.position.0 + robot.velocity.0 * seconds).rem_euclid(width);
            let y = (robot.position.1 + robot.velocity.1 * seconds).rem_euclid(height);
            (x, y)
        })
        .collect()
}

fn calculate_safety_factor(positions: Vec<(i64, i64)>, width: i64, height: i64) -> i64 {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0; 4];

    for (x, y) in positions {
        if x == mid_x || y == mid_y {
            continue;
        }
        if x < mid_x && y < mid_y {
            quadrants[0] += 1; // Top-left
        } else if x >= mid_x && y < mid_y {
            quadrants[1] += 1; // Top-right
        } else if x < mid_x && y >= mid_y {
            quadrants[2] += 1; // Bottom-left
        } else {
            quadrants[3] += 1; // Bottom-right
        }
    }

    quadrants.iter().product()
}

pub fn solve(robots: &[Robot], seconds: i64) -> i64 {
    let width = 101;
    let height = 103;

    let positions = simulate_robots(robots, seconds, width, height);
    calculate_safety_factor(positions, width, height)
}

pub enum PuzzlePart {
    Part1,
    Part2,
}

pub fn run(part: PuzzlePart, input_path: &str) -> i64 {
    let robots = parse_input(input_path);
    match part {
        PuzzlePart::Part1 => solve(&robots, 100),
        PuzzlePart::Part2 => 0, // Part 2 not implemented yet
    }
}

pub fn run_part1() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }
    let input_path = &args[1];
    let result = run(PuzzlePart::Part1, input_path);
    println!("Safety factor: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_calculate_safety_factor() {
        let robots = vec![
            Robot {
                position: (0, 4),
                velocity: (3, -3),
            },
            Robot {
                position: (6, 3),
                velocity: (-1, -3),
            },
            Robot {
                position: (10, 3),
                velocity: (-1, 2),
            },
            Robot {
                position: (2, 0),
                velocity: (2, -1),
            },
            Robot {
                position: (0, 0),
                velocity: (1, 3),
            },
        ];
        let positions = simulate_robots(&robots, 100, 11, 7);
        let safety_factor = calculate_safety_factor(positions, 11, 7);
        assert_eq!(safety_factor, 21); // Update to the correct expected value
    }

    #[test]
    fn test_simulate_robots_wrapping() {
        let robots = vec![Robot {
            position: (10, 6),
            velocity: (2, 3),
        }];
        let positions = simulate_robots(&robots, 1, 11, 7);
        assert_eq!(positions, vec![(1, 2)]); // Wraps around the edges
    }

    #[test]
    fn test_parse_input() {
        let input = "p=0,4 v=3,-3\np=6,3 v=-1,-3";
        let path = "/tmp/test_input.txt";
        std::fs::write(path, input).unwrap();
        let robots = parse_input(path);
        assert_eq!(robots.len(), 2);
        assert_eq!(robots[0].position, (0, 4));
        assert_eq!(robots[0].velocity, (3, -3));
        assert_eq!(robots[1].position, (6, 3));
        assert_eq!(robots[1].velocity, (-1, -3));
    }
}
