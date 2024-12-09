use crate::utils::Args;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn run_part1() {
    let args = Args::parse();
    match compact_disk(&args.file_path) {
        Ok((_, checksum)) => println!("Part 1: {}", checksum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn compact_disk(file_path: &str) -> Result<(String, u128), std::io::Error> {
    let disk_map = read_input_from_file(file_path)?;
    if disk_map.is_empty() || !disk_map.chars().all(|c| c.is_ascii_digit()) {
        panic!("Input contains non-digit characters or is empty!");
    }

    for (i, c) in disk_map.chars().enumerate() {
        if c.to_digit(10).is_none() {
            println!("Non-digit character at position {}: {:?}", i, c);
            panic!("Invalid input character");
        }
    }

    // Parse the input into alternating file and free space lengths
    let lengths: Vec<usize> = disk_map
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    // Create the initial block representation
    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut current_file_id = 0;

    for (i, &length) in lengths.iter().enumerate() {
        let is_file = i % 2 == 0;
        blocks.extend(
            std::iter::repeat(if is_file { Some(current_file_id) } else { None }).take(length),
        );
        if is_file {
            current_file_id += 1;
        }
    }

    // Perform the step-by-step compaction for Part 1
    loop {
        let leftmost_free = blocks.iter().position(|&b| b.is_none());
        if leftmost_free.is_none() {
            // No free blocks, all compacted
            break;
        }
        let leftmost_free = leftmost_free.unwrap();

        let rightmost_file = blocks.iter().rposition(|&b| {
            b.is_some()
                && blocks
                    .iter()
                    .position(|&x| x.is_none())
                    .map_or(true, |fpos| fpos < blocks.len())
        });
        if let Some(rf) = rightmost_file {
            if rf <= leftmost_free {
                // All file blocks are to the left or equal to this free space
                break;
            }

            let file_id = blocks[rf].take();
            blocks[leftmost_free] = file_id;
        } else {
            break;
        }
    }

    let checksum: u128 = calculate_checksum(&blocks);

    // Convert blocks to string representation
    let block_string: String = blocks
        .iter()
        .map(|&block| match block {
            None => '.',
            Some(id) => id.to_string().chars().next().unwrap(),
        })
        .collect();

    Ok((block_string, checksum))
}

pub fn run_part2() {
    let args = Args::parse();
    match compact_disk_part2(&args.file_path) {
        Ok((_, checksum)) => println!("Part 2: {}", checksum),
        Err(e) => eprintln!("Error: {}", e),
    }
}

pub fn compact_disk_part2(file_path: &str) -> Result<(String, u128), std::io::Error> {
    let disk_map = read_input_from_file(file_path)?;
    if disk_map.is_empty() || !disk_map.chars().all(|c| c.is_ascii_digit()) {
        panic!("Input contains non-digit characters or is empty!");
    }

    // Parse the input
    let lengths: Vec<usize> = disk_map
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut blocks: Vec<Option<usize>> = Vec::new();
    let mut current_file_id = 0;

    for (i, &length) in lengths.iter().enumerate() {
        let is_file = i % 2 == 0;
        blocks.extend(
            std::iter::repeat(if is_file { Some(current_file_id) } else { None }).take(length),
        );
        if is_file {
            current_file_id += 1;
        }
    }

    let max_file_id = current_file_id.saturating_sub(1);

    // Move files from highest ID to lowest ID
    for file_id in (0..=max_file_id).rev() {
        // Find positions of this file
        let file_positions: Vec<usize> = blocks
            .iter()
            .enumerate()
            .filter_map(|(pos, &b)| if b == Some(file_id) { Some(pos) } else { None })
            .collect();

        if file_positions.is_empty() {
            continue;
        }

        let file_len = file_positions.len();
        let leftmost_file_pos = *file_positions.first().unwrap();

        // Find a suitable free span to the left of leftmost_file_pos
        let free_spans = find_free_spans(&blocks, 0, leftmost_file_pos);
        let suitable_span = free_spans.into_iter().find(|span| span.len() >= file_len);

        if let Some(span) = suitable_span {
            // Move the entire file into this free span
            // First clear old positions
            for &pos in &file_positions {
                blocks[pos] = None;
            }

            // Fill the new position
            for pos in span.start..(span.start + file_len) {
                blocks[pos] = Some(file_id);
            }
        }
    }

    let checksum = calculate_checksum(&blocks);
    let block_string = blocks
        .iter()
        .map(|&b| match b {
            None => '.',
            Some(id) => id.to_string().chars().next().unwrap(),
        })
        .collect();

    Ok((block_string, checksum))
}

/// Find contiguous free spans in [0, end_limit).
fn find_free_spans(
    blocks: &[Option<usize>],
    start_limit: usize,
    end_limit: usize,
) -> Vec<std::ops::Range<usize>> {
    let mut spans = Vec::new();
    let mut current_start = None;

    for pos in start_limit..end_limit {
        if blocks[pos].is_none() {
            if current_start.is_none() {
                current_start = Some(pos);
            }
        } else {
            if let Some(s) = current_start {
                spans.push(s..pos);
                current_start = None;
            }
        }
    }

    // End in a free span
    if let Some(s) = current_start {
        spans.push(s..end_limit);
    }

    spans
}

fn calculate_checksum(blocks: &[Option<usize>]) -> u128 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &b)| {
            b.map(|file_id| {
                if pos > 0 {
                    pos as u128 * file_id as u128
                } else {
                    0
                }
            })
        })
        .sum()
}

pub fn read_input_from_file(filepath: &str) -> Result<String, std::io::Error> {
    let path = Path::new(filepath);
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_disk() {
        let file_path = "input/9-disk-fragmentation/test-input-1.txt";
        let (blocks, checksum) = compact_disk(file_path).expect("Failed to compact disk");
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn test_compact_disk_part2() {
        let file_path = "input/9-disk-fragmentation/test-input-2.txt";
        let (blocks, checksum) = compact_disk_part2(file_path).expect("Failed to compact disk");
        assert_eq!(checksum, 2858);
    }
}
