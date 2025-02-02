use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter,
};

use crate::solver::AoCSolver;

#[derive(Clone, Copy)]
enum MemoryBlock {
    FreeSpace,
    File(usize),
}

pub struct Solver {
    memory: Vec<MemoryBlock>,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2024/day_09.txt").unwrap();
        let reader = BufReader::new(file);

        let first_line = reader.lines().next().unwrap().unwrap();
        return Solver::from_dense_format(first_line.as_str());
    }

    pub fn from_dense_format(dense_memory: &str) -> Self {
        let mut memory = Vec::new();
        let mut value_represents_free_space = false;
        let mut id: usize = 0;

        for character in dense_memory.chars() {
            let value: usize = character.to_digit(10).unwrap() as usize;
            let memory_block = if value_represents_free_space {
                MemoryBlock::FreeSpace
            } else {
                let file = MemoryBlock::File(id);
                id += 1;
                file
            };
            memory.extend(iter::repeat_n(memory_block, value));
            value_represents_free_space = !value_represents_free_space;
        }

        Solver { memory }
    }

    fn defrag(&self) -> Vec<MemoryBlock> {
        let mut working_memory = self.memory.clone();

        let mut head_index = 0;
        let mut tail_index = working_memory.len() - 1;

        loop {
            // walk head pointer forward until it points to a free space
            while !matches!(working_memory[head_index], MemoryBlock::FreeSpace)
                && head_index < tail_index
            {
                head_index += 1;
            }

            // walk tail index back until it points to a file block
            while !matches!(working_memory[tail_index], MemoryBlock::File(_))
                && head_index < tail_index
            {
                tail_index -= 1;
            }

            // defrag is complete if the pointers have crossed
            if head_index >= tail_index {
                break;
            }

            // swap memory
            working_memory[head_index] = working_memory[tail_index];
            working_memory[tail_index] = MemoryBlock::FreeSpace;
        }

        return working_memory;
    }

    fn defrag_2(&self) -> Vec<MemoryBlock> {
        let mut working_memory = self.memory.clone();

        // Find id of last memory block. This will be the max id of all file blocks in the memory.
        // This value will be decremented during the defrag process to ensure blocks are not moved more than once.
        let mut max_id = *working_memory
            .iter()
            .filter_map(|block| {
                if let MemoryBlock::File(id) = block {
                    Some(id)
                } else {
                    None
                }
            })
            .last()
            .unwrap();

        let mut free_space_sections = find_free_space_sections(&working_memory);

        loop {
            if let Some((start_index, length, block_id)) =
                find_memory_block_to_move(&working_memory, max_id)
            {
                if let Some(free_space_index) =
                    free_space_sections.iter().position(|(_, l)| *l >= length)
                {
                    let (free_space_start, free_space_length) =
                        free_space_sections[free_space_index];

                    if free_space_start > start_index {
                        // println!(
                        //     "Not moving {}, no free spaces left of {}",
                        //     block_id, free_space_start
                        // );
                    } else {
                        // println!(
                        //     "moving {} from {} to {}",
                        //     block_id, start_index, free_space_start
                        // );

                        // Replace free space block with file
                        for i in free_space_start..free_space_start + length {
                            working_memory[i] = MemoryBlock::File(block_id);
                        }

                        // Replace file block with free space
                        for i in start_index..start_index + length {
                            working_memory[i] = MemoryBlock::FreeSpace;
                        }

                        // Shrink free space accordingly. No need to add new gap created at the end
                        // as the algorithm won't try to place a file there.
                        let updated_free_space_start = free_space_start + length;
                        let updated_free_space_length = free_space_length - length;
                        free_space_sections[free_space_index] =
                            (updated_free_space_start, updated_free_space_length);
                    }
                }
                if max_id > 0 {
                    max_id -= 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        return working_memory;
    }
}

fn find_free_space_sections(memory: &Vec<MemoryBlock>) -> Vec<(usize, usize)> {
    let mut free_space: Vec<(usize, usize)> = Vec::new();
    let mut section_start: Option<usize> = None;
    for (i, memory_block) in memory.iter().enumerate() {
        match memory_block {
            MemoryBlock::FreeSpace => {
                if section_start == None {
                    section_start = Some(i);
                }
            }
            _ => {
                if let Some(start_index) = section_start {
                    free_space.push((start_index, i - start_index));
                    section_start = None;
                }
            }
        }
    }
    // handle scenario where memory ends with free space
    if let Some(start_index) = section_start {
        free_space.push((start_index, memory.len() - start_index));
    }
    return free_space;
}

// Returns (start_index, length, id)
fn find_memory_block_to_move(
    memory: &Vec<MemoryBlock>,
    max_id: usize,
) -> Option<(usize, usize, usize)> {
    let mut end_index: Option<usize> = None;
    let mut block_id: Option<usize> = None;

    for (i, memory_block) in memory.iter().enumerate().rev() {
        match memory_block {
            MemoryBlock::FreeSpace => {
                if let Some(end_index) = end_index {
                    let start_index = i + 1;
                    let length = (end_index - start_index) + 1;
                    return Some((start_index, length, block_id.unwrap()));
                }
            }
            MemoryBlock::File(id) => {
                if let Some(block_id) = block_id {
                    if block_id == *id {
                        continue;
                    }
                    let start_index = i + 1;
                    let length = (end_index.unwrap() - start_index) + 1;
                    return Some((start_index, length, block_id));
                } else {
                    if *id <= max_id {
                        end_index = Some(i);
                        block_id = Some(*id);
                    }
                }
            }
        }
    }
    if let Some(end_index) = end_index {
        let length = end_index;
        return Some((0, length, block_id.unwrap()));
    }
    return None;
}

fn calculate_checksum(memory: &Vec<MemoryBlock>) -> usize {
    memory
        .iter()
        .enumerate()
        .map(|(index, memory_block)| match memory_block {
            MemoryBlock::FreeSpace => 0,
            MemoryBlock::File(id) => id * index,
        })
        .sum()
}

fn print(memory: &Vec<MemoryBlock>) {
    for block in memory.iter() {
        print!(
            "{}",
            match block {
                MemoryBlock::File(id) => id.to_string(),
                MemoryBlock::FreeSpace => ".".to_string(),
            }
        );
    }
    println!("")
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let defragged = self.defrag();
        let checksum = calculate_checksum(&defragged);
        return checksum.to_string();
    }

    fn solve_part_2(&self) -> String {
        let defragged = self.defrag_2();
        let checksum = calculate_checksum(&defragged);
        return checksum.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defrag_example() {
        let solver = Solver::from_dense_format("2333133121414131402");
        let defragged = solver.defrag();
        print(&defragged);
        let checksum = calculate_checksum(&defragged);
        assert_eq!(checksum, 1928);
    }

    #[test]
    fn defrag_2_example() {
        let solver = Solver::from_dense_format("2333133121414131402");
        let defragged = solver.defrag_2();
        print(&defragged);
        let checksum = calculate_checksum(&defragged);
        assert_eq!(checksum, 2858);
    }

    #[test]
    fn find_free_space_sections_finds_expected_sections() {
        let solver = Solver::from_dense_format("2333133121414131402");
        let sections = find_free_space_sections(&solver.memory);
        let expected = vec![
            (2, 3),
            (8, 3),
            (12, 3),
            (18, 1),
            (21, 1),
            (26, 1),
            (31, 1),
            (35, 1),
        ];
        assert_eq!(sections, expected);
    }

    #[test]
    fn find_memory_block_to_move_test() {
        let solver = Solver::from_dense_format("2333133121414131402");
        let (start_index, length, id) = find_memory_block_to_move(&solver.memory, 7).unwrap();
        assert_eq!(start_index, 32);
        assert_eq!(length, 3);
        assert_eq!(id, 7);
    }
}
