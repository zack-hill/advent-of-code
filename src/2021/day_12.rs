use crate::solver::AoCSolver;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::BufRead,
    io::BufReader,
};

pub struct Solver {
    cave_connections: HashMap<u32, Vec<u32>>,
    cave_is_small_map: HashMap<u32, bool>,
    name_map: HashMap<u32, String>,
    start_id: u32,
    end_id: u32,
}

impl Solver {
    pub fn create() -> Self {
        let file = File::open("src/2021/day_12.txt").unwrap();
        let reader = BufReader::new(file);

        let mut cave_connections = HashMap::new();
        let mut cave_is_small_map = HashMap::new();
        let mut id_to_name = HashMap::new();
        let mut name_to_id = HashMap::new();
        let mut id_seed = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            let (left, right) = line.split_once('-').unwrap();

            let left_id = *name_to_id
                .entry(left.to_string())
                .or_insert(increment_and_return(&mut id_seed));
            let right_id = *name_to_id
                .entry(right.to_string())
                .or_insert(increment_and_return(&mut id_seed));

            id_to_name.insert(left_id, left.to_string());
            let left_connections = cave_connections.entry(left_id).or_insert(Vec::new());
            left_connections.push(right_id);
            cave_is_small_map.insert(left_id, is_cave_small(left));

            id_to_name.insert(right_id, right.to_string());
            let right_connections = cave_connections.entry(right_id).or_insert(Vec::new());
            right_connections.push(left_id);
            cave_is_small_map.insert(right_id, is_cave_small(right));
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
        cave: u32,
        chain: &mut VecDeque<u32>,
        can_re_explore_small_cave: bool,
        have_re_explored_cave: bool,
    ) -> u32 {
        let mut exit_path_count = 0;
        chain.push_back(cave);
        let local_connections = self.cave_connections.get(&cave).unwrap();
        for &connected_cave in local_connections.iter() {
            if connected_cave == self.start_id {
                continue;
            }
            if connected_cave == self.end_id {
                // println!("{:?}", chain);
                exit_path_count += 1;
                continue;
            }
            if self.cave_is_small_map[&connected_cave] && chain.contains(&connected_cave) {
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

fn increment_and_return(value: &mut u32) -> u32 {
    *value = *value + 1;
    return *value;
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
