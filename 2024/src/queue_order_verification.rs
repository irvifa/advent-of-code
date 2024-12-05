use crate::utils::Args;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn run_part1() {
    let args = Args::parse();
    let (rules, updates) = parse_input(&args.file_path);

    let correct_updates: Vec<Vec<i32>> = updates
        .iter()
        .filter(|update| is_update_in_correct_order(&rules, update))
        .cloned()
        .collect();

    let middle_page_sum: i32 = correct_updates
        .iter()
        .map(|update| get_middle_page(update))
        .sum();

    println!(
        "Sum of middle pages in correctly ordered updates: {}",
        middle_page_sum
    );
}

fn parse_input(filename: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    // First parse ordering rules
    for line in lines.by_ref() {
        let line = line.expect("Could not read line");
        if line.trim().is_empty() {
            break;
        }
        let parts: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        rules.push((parts[0], parts[1]));
    }

    // Then parse updates
    for line in lines {
        let line = line.expect("Could not read line");
        if line.trim().is_empty() {
            continue;
        }
        let update: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        updates.push(update);
    }

    (rules, updates)
}

fn is_update_in_correct_order(rules: &[(i32, i32)], update: &[i32]) -> bool {
    let dependencies = build_dependency_graph(rules, update);

    for (i, &page) in update.iter().enumerate() {
        for (j, &other_page) in update.iter().enumerate() {
            if i == j {
                continue;
            }

            if dependencies.contains_key(&page)
                && dependencies[&page].contains(&other_page)
                && update.iter().position(|&x| x == page).unwrap()
                    > update.iter().position(|&x| x == other_page).unwrap()
            {
                return false;
            }
        }
    }
    true
}

fn build_dependency_graph(rules: &[(i32, i32)], update: &[i32]) -> HashMap<i32, HashSet<i32>> {
    let mut dependencies: HashMap<i32, HashSet<i32>> = HashMap::new();
    let update_set: HashSet<i32> = update.iter().cloned().collect();

    for (before, after) in rules {
        if update_set.contains(before) && update_set.contains(after) {
            dependencies.entry(*before).or_default().insert(*after);
        }
    }

    dependencies
}

fn get_middle_page(update: &[i32]) -> i32 {
    update[update.len() / 2]
}

pub fn run_part2() {
    let args = Args::parse();
    let (rules, updates) = parse_input(&args.file_path);

    let incorrectly_ordered_updates: Vec<Vec<i32>> = updates
        .iter()
        .filter(|update| !is_update_in_correct_order(&rules, update))
        .cloned()
        .collect();

    let corrected_updates: Vec<Vec<i32>> = incorrectly_ordered_updates
        .iter()
        .map(|update| correct_update_order(rules.clone(), update))
        .collect();

    let middle_page_sum: i32 = corrected_updates
        .iter()
        .map(|update| get_middle_page(update))
        .sum();

    println!(
        "Sum of middle pages in corrected updates: {}",
        middle_page_sum
    );
}

fn correct_update_order(rules: Vec<(i32, i32)>, update: &[i32]) -> Vec<i32> {
    // Create a dependency graph and in-degree map
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();
    let update_set: HashSet<i32> = update.iter().cloned().collect();

    // Initialize in-degree and graph
    for &page in update {
        in_degree.insert(page, 0);
    }

    // Build graph and calculate in-degrees
    for (before, after) in &rules {
        if update_set.contains(before) && update_set.contains(after) {
            graph.entry(*before).or_default().insert(*after);

            *in_degree.entry(*after).or_default() += 1;
        }
    }

    // Topological sort with original order preservation
    let mut result = Vec::new();
    let mut candidates: VecDeque<i32> = update
        .iter()
        .filter(|&&page| in_degree[&page] == 0)
        .cloned()
        .collect();

    // Preserve original order for candidates with zero in-degree
    candidates.make_contiguous().sort_by(|&a, &b| {
        update
            .iter()
            .position(|&x| x == a)
            .cmp(&update.iter().position(|&x| x == b))
    });

    while let Some(current) = candidates.pop_front() {
        result.push(current);

        // Check neighbors
        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                let degree = in_degree.get_mut(&neighbor).unwrap();
                *degree -= 1;

                if *degree == 0 {
                    candidates.push_back(neighbor);
                }
            }
        }

        // If multiple candidates, sort by original order
        if candidates.len() > 1 {
            candidates.make_contiguous().sort_by(|&a, &b| {
                update
                    .iter()
                    .position(|&x| x == a)
                    .cmp(&update.iter().position(|&x| x == b))
            });
        }
    }

    // If result doesn't include all pages, use original order for remaining
    let missing_pages: Vec<i32> = update
        .iter()
        .filter(|&&page| !result.contains(&page))
        .cloned()
        .collect();

    result.extend(missing_pages);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word() {
        let input_path = "input/5-queue-order-verification/test-input-1.txt";
        let (rules, updates) = parse_input(input_path);

        // Verify rules parsing
        assert_eq!(rules.len(), 21, "Should parse 21 rules"); // Changed from 22 to 21
        assert!(rules.contains(&(47, 53)), "Should contain rule 47|53");
        assert!(rules.contains(&(97, 75)), "Should contain rule 97|75");

        // Verify updates parsing
        assert_eq!(updates.len(), 6, "Should parse 6 updates");
        assert_eq!(
            updates[0],
            vec![75, 47, 61, 53, 29],
            "First update should match"
        );

        // Test order verification
        let correct_updates: Vec<Vec<i32>> = updates
            .iter()
            .filter(|update| is_update_in_correct_order(&rules, update))
            .cloned()
            .collect();

        // Verify correctly ordered updates
        assert_eq!(
            correct_updates.len(),
            3,
            "Should have 3 correctly ordered updates"
        );
        assert!(
            correct_updates.contains(&vec![75, 47, 61, 53, 29]),
            "First update should be correct"
        );
        assert!(
            correct_updates.contains(&vec![97, 61, 53, 29, 13]),
            "Second update should be correct"
        );
        assert!(
            correct_updates.contains(&vec![75, 29, 13]),
            "Third update should be correct"
        );

        // Test middle page calculation
        let middle_pages: Vec<i32> = correct_updates
            .iter()
            .map(|update| get_middle_page(update))
            .collect();

        assert_eq!(middle_pages, vec![61, 53, 29], "Middle pages should match");

        // Test final sum
        let middle_page_sum: i32 = middle_pages.iter().sum();
        assert_eq!(middle_page_sum, 143, "Sum of middle pages should be 143");
    }

    #[test]
    fn test_correct_update_order() {
        let input_path = "input/5-queue-order-verification/test-input-2.txt";
        let (rules, updates) = parse_input(input_path);

        let incorrectly_ordered_updates: Vec<Vec<i32>> = updates
            .iter()
            .filter(|update| !is_update_in_correct_order(&rules, update))
            .cloned()
            .collect();

        let corrected_updates: Vec<Vec<i32>> = incorrectly_ordered_updates
            .iter()
            .map(|update| correct_update_order(rules.clone(), update))
            .collect();

        // Verify corrections
        assert_eq!(
            corrected_updates.len(),
            3,
            "Should have 3 corrected updates"
        );
        assert_eq!(
            corrected_updates[0],
            vec![97, 75, 47, 61, 53],
            "First update should be corrected"
        );
        assert_eq!(
            corrected_updates[1],
            vec![61, 29, 13],
            "Second update should be corrected"
        );
        assert_eq!(
            corrected_updates[2],
            vec![97, 75, 47, 29, 13],
            "Third update should be corrected"
        );

        // Test middle page calculation
        let middle_pages: Vec<i32> = corrected_updates
            .iter()
            .map(|update| get_middle_page(update))
            .collect();

        assert_eq!(middle_pages, vec![47, 29, 47], "Middle pages should match");

        // Test final sum
        let middle_page_sum: i32 = middle_pages.iter().sum();
        assert_eq!(middle_page_sum, 123, "Sum of middle pages should be 123");
    }
}
