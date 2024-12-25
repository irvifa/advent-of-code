use crate::utils::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct ClawMachine {
    button_a: (i64, i64), // (X movement, Y movement)
    button_b: (i64, i64),
    prize: (i64, i64), // Prize location (X, Y)
}

enum PuzzlePart {
    Part1,
    Part2,
}

fn parse_input(input_path: &str) -> Vec<ClawMachine> {
    let file = File::open(input_path).expect("Failed to open input file");
    let reader = BufReader::new(file);
    let mut machines = Vec::new();
    let mut current_machine = None;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            if let Some(machine) = current_machine.take() {
                machines.push(machine);
            }
            continue;
        }

        if line.starts_with("Button A:") {
            let parts: Vec<&str> = line.split(", ").collect();
            let x = parts[0]
                .trim_start_matches("Button A: X")
                .parse::<i64>()
                .unwrap();
            let y = parts[1].trim_start_matches("Y").parse::<i64>().unwrap();
            current_machine = Some(ClawMachine {
                button_a: (x, y),
                button_b: (0, 0),
                prize: (0, 0),
            });
        } else if line.starts_with("Button B:") {
            if let Some(ref mut machine) = current_machine {
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0]
                    .trim_start_matches("Button B: X")
                    .parse::<i64>()
                    .unwrap();
                let y = parts[1].trim_start_matches("Y").parse::<i64>().unwrap();
                machine.button_b = (x, y);
            }
        } else if line.starts_with("Prize:") {
            if let Some(ref mut machine) = current_machine {
                let parts: Vec<&str> = line.split(", ").collect();
                let x = parts[0]
                    .trim_start_matches("Prize: X=")
                    .parse::<i64>()
                    .unwrap();
                let y = parts[1].trim_start_matches("Y=").parse::<i64>().unwrap();
                machine.prize = (x, y);
            }
        }
    }

    if let Some(machine) = current_machine {
        machines.push(machine);
    }

    machines
}

fn find_solution(machine: &ClawMachine, max_presses: i64) -> Option<(i64, i64)> {
    // Try all combinations of button presses up to max_presses
    for a in 0..=max_presses {
        for b in 0..=max_presses {
            let x = a * machine.button_a.0 + b * machine.button_b.0;
            let y = a * machine.button_a.1 + b * machine.button_b.1;

            if x == machine.prize.0 && y == machine.prize.1 {
                return Some((a, b));
            }
        }
    }
    None
}

fn calculate_tokens(a_presses: i64, b_presses: i64) -> i64 {
    a_presses * 3 + b_presses * 1
}

fn solve(machines: &[ClawMachine], max_presses: i64) -> i64 {
    let mut total_tokens = 0;
    let mut winnable_prizes = 0;

    for machine in machines {
        if let Some((a_presses, b_presses)) = find_solution(machine, max_presses) {
            total_tokens += calculate_tokens(a_presses, b_presses);
            winnable_prizes += 1;
        }
    }

    println!("Winnable prizes: {}", winnable_prizes);
    total_tokens
}

pub fn run(part: PuzzlePart, input_path: &str) -> i64 {
    let machines = parse_input(input_path);
    match part {
        PuzzlePart::Part1 => solve(&machines, 100),
        PuzzlePart::Part2 => 0, // Part 2 not implemented yet
    }
}

pub fn run_part1() {
    let args = Args::parse();
    let result = run(PuzzlePart::Part1, &args.file_path);
    println!("Total tokens needed: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // Run the solution
        let result = run(
            PuzzlePart::Part1,
            "input/13-claw-contraption/test-input1.txt",
        );

        // According to the puzzle description:
        // - First machine costs 280 tokens (80 A presses * 3 + 40 B presses * 1)
        // - Second machine has no solution
        // - Third machine costs 200 tokens (38 A presses * 3 + 86 B presses * 1)
        // - Fourth machine has no solution
        // Total: 480 tokens
        assert_eq!(result, 480);
    }
}
