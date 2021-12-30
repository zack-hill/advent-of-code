use crate::solver::AoCSolver;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::BufRead,
    io::BufReader,
};

#[allow(dead_code)] // name_map is not used, but would be used if the chain was printed out for debugging purposes
pub struct Solver {
    cave_connections: Vec<Vec<usize>>,
    cave_is_small_map: Vec<bool>,
    name_map: Vec<String>,
    start_id: usize,
    end_id: usize,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2021/day_12.txt").unwrap();
        let reader = BufReader::new(file);

        let mut cave_connections = Vec::new();
        let mut cave_is_small_map = Vec::new();
        let mut id_to_name = Vec::new();
        let mut name_to_id = HashMap::new();

        cave_connections.resize(12, Vec::new());
        cave_is_small_map.resize(12, false);
        id_to_name.resize(12, String::from(""));

        for line in reader.lines() {
            let line = line.unwrap();
            let (left, right) = line.split_once('-').unwrap();

            let next_id = name_to_id.len();
            let left_id = *name_to_id.entry(left.to_string()).or_insert(next_id);
            let next_id = name_to_id.len();
            let right_id = *name_to_id.entry(right.to_string()).or_insert(next_id);

            id_to_name[left_id] = left.to_string();
            cave_connections[left_id].push(right_id);
            cave_is_small_map[left_id] = is_cave_small(left);

            id_to_name[right_id] = right.to_string();
            cave_connections[right_id].push(left_id);
            cave_is_small_map[right_id] = is_cave_small(right);
        }

        let start_id = name_to_id["start"];
        let end_id = name_to_id["end"];

        Solver {
            cave_connections,
            cave_is_small_map,
            name_map: id_to_name,
            start_id,
            end_id,
        }
    }

    fn explore(
        &self,
        cave: usize,
        chain: &mut VecDeque<usize>,
        can_re_explore_small_cave: bool,
        have_re_explored_cave: bool,
    ) -> u32 {
        let mut exit_path_count = 0;
        chain.push_back(cave);
        for &connected_cave in self.cave_connections[cave].iter() {
            if connected_cave == self.start_id {
                continue;
            }
            if connected_cave == self.end_id {
                // println!("{:?}", chain);
                exit_path_count += 1;
                continue;
            }
            if self.cave_is_small_map[connected_cave] && chain.contains(&connected_cave) {
                if can_re_explore_small_cave && !have_re_explored_cave {
                    exit_path_count +=
                        self.explore(connected_cave, chain, can_re_explore_small_cave, true);
                }
                continue;
            }
            exit_path_count += self.explore(
                connected_cave,
                chain,
                can_re_explore_small_cave,
                have_re_explored_cave,
            );
        }
        chain.pop_back();
        return exit_path_count;
    }
}

fn is_cave_small(cave: &str) -> bool {
    cave.to_lowercase() == cave
}

impl AoCSolver for Solver {
    fn solve_part_1(&self) -> String {
        // println!("{:?}", self.cave_connections);
        let exit_path_count = self.explore(self.start_id, &mut VecDeque::new(), false, false);
        return exit_path_count.to_string();
    }

    fn solve_part_2(&self) -> String {
        let exit_path_count = self.explore(self.start_id, &mut VecDeque::new(), true, false);
        return exit_path_count.to_string();
    }
}
