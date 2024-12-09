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

    // Perform the step-by-step compaction
    // Keep moving the rightmost file block into the leftmost free slot until fully compacted.
    loop {
        // Find the first free slot from the left
        let leftmost_free = blocks.iter().position(|&b| b.is_none());
        if leftmost_free.is_none() {
            // No free blocks, all compacted
            break;
        }
        let leftmost_free = leftmost_free.unwrap();

        // Find if there is any file block to the right of this free slot
        let rightmost_file = blocks.iter().rposition(|&b| {
            b.is_some()
                && blocks
                    .iter()
                    .position(|&x| x.is_none())
                    .map_or(true, |fpos| fpos < blocks.len())
        });
        if let Some(rf) = rightmost_file {
            if rf <= leftmost_free {
                // All file blocks are at or to the left of this free space, no need to move further
                break;
            }

            // Move that file block into the leftmost free slot
            // Save the file_id
            let file_id = blocks[rf].take();
            blocks[leftmost_free] = file_id;
        } else {
            // No file to the right, done
            break;
        }
    }

    // Calculate the checksum as specified
    let checksum: u128 = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &block)| {
            block.map(|file_id| {
                // Use u128 for multiplication and addition
                if pos > 0 {
                    pos as u128 * file_id as u128
                } else {
                    0
                }
            })
        })
        .sum();

    // Convert blocks to string representation
    let block_string: String = blocks
        .iter()
        .map(|&block| {
            match block {
                None => '.',
                // // Just take the first digit of the ID
                Some(id) => id.to_string().chars().next().unwrap(),
            }
        })
        .collect();

    Ok((block_string, checksum))
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

        println!("Original Input Lengths: [Your input lengths]");
        println!("Compacted blocks: {}", blocks);
        println!("Checksum: {}", checksum);
        println!("Expected Checksum: 1928");

        // Detailed block information
        let detailed_blocks: Vec<_> = blocks
            .chars()
            .enumerate()
            .filter(|&(_, c)| c != '.')
            .collect();

        for (pos, file_id) in detailed_blocks {
            println!("Position: {}, File ID: {}", pos, file_id);
        }

        assert_eq!(checksum, 1928);
    }
}
