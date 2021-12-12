#![feature(test)]

use crate::cave_system::{is_small_cave, CaveId};
use crate::parse_input::{parse, read_main, ParseOutput};
use std::cmp::max;
use std::collections::{HashMap, VecDeque};

pub mod cave_system;
pub mod parse_input;
type Solution = u32;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

#[derive(Clone, Debug)]
struct CavePath {
    small_caves_visited: HashMap<CaveId, u8>,
    path: Vec<CaveId>,
    max_visit_count: u8,
    allow_one_above_max: bool,
}

impl CavePath {
    pub fn visit_small_cave(&mut self, id: &CaveId) -> bool {
        if let None = self.small_caves_visited.get(id) {
            self.small_caves_visited.insert(id.clone(), 0);
        }
        let visited = self.small_caves_visited.get(id).unwrap();

        if visited < &self.max_visit_count
            || (self.allow_one_above_max && self.get_max() < self.max_visit_count + 1)
        {
            *self.small_caves_visited.get_mut(id).unwrap() += 1;
            return true;
        }

        return false;
    }

    pub fn get_max(&self) -> u8 {
        self.small_caves_visited
            .values()
            .fold(0, |acc, num| max(acc, *num))
    }
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut valid_paths_count: u32 = 0;

    let mut active_paths = vec![CavePath {
        small_caves_visited: HashMap::new(),
        allow_one_above_max: false,
        max_visit_count: 1,
        path: vec![parse_output.get_start_id()],
    }];

    while active_paths.len() > 0 {
        active_paths = explore_cave(parse_output, active_paths);

        active_paths = filter_path(active_paths, "dead");
        let active_path_count_after_filtering_dead_ends = active_paths.len();
        active_paths = filter_path(active_paths, "end");
        valid_paths_count +=
            (active_path_count_after_filtering_dead_ends - active_paths.len()) as u32;
    }

    valid_paths_count
}

fn explore_cave(cave_system: &ParseOutput, active_paths: Vec<CavePath>) -> Vec<CavePath> {
    let mut new_paths = Vec::new();

    for path in active_paths.iter() {
        let cave_id = path.path.last().unwrap();
        let adjacent_cave_ids = cave_system.get_adjacent_cave_ids(cave_id);

        for cave_id in &adjacent_cave_ids {
            if cave_id == "start" {
                continue;
            }

            let mut new_path = path.clone();
            if is_small_cave(cave_id) {
                let visit_successful = new_path.visit_small_cave(cave_id);
                if !visit_successful {
                    new_path.visit_small_cave(cave_id);
                    new_path.path.push("dead".to_string());
                    new_paths.push(new_path);
                    continue;
                }
            }

            new_path.path.push(cave_id.clone());
            new_paths.push(new_path)
        }
    }

    new_paths
}

fn filter_path(active_paths: Vec<CavePath>, id: &str) -> Vec<CavePath> {
    active_paths
        .into_iter()
        .filter(|p| p.path.last().unwrap() != id)
        .collect()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut valid_paths_count: u32 = 0;

    let mut active_paths = vec![CavePath {
        small_caves_visited: HashMap::new(),
        allow_one_above_max: true,
        max_visit_count: 1,
        path: vec![parse_output.get_start_id()],
    }];

    while active_paths.len() > 0 {
        active_paths = explore_cave(parse_output, active_paths);

        active_paths = filter_path(active_paths, "dead");
        let active_path_count_after_filtering_dead_ends = active_paths.len();
        active_paths = filter_path(active_paths, "end");
        valid_paths_count +=
            (active_path_count_after_filtering_dead_ends - active_paths.len()) as u32;
    }

    valid_paths_count
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse, read_main, read_test};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(&read_test());
        assert_eq!(part_1(&parse_output), 226);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 3509);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        let file = read_main();
        b.iter(|| {
            let _ = parse(&file);
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 4707);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 130493);
        });
    }
}
