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
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        let defragged = self.defrag();
        let checksum = Solver::calculate_checksum(&defragged);
        return checksum.to_string();
    }

    fn solve_part_2(&self) -> String {
        return "".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defrag_example() {
        let solver = Solver::from_dense_format("2333133121414131402");
        let defragged = solver.defrag();
        Solver::print(&defragged);
        let checksum = Solver::calculate_checksum(&defragged);
        assert_eq!(checksum, 1928);
    }
}
