use std::collections::hash_map::{Iter, Keys};
use std::collections::{HashMap, HashSet};

pub type CaveId = String;

pub struct CaveSystem {
    start_id: CaveId,
    cave_map: HashMap<CaveId, Cave>,
}

#[derive(Debug)]
pub struct Cave {
    id: CaveId,
    cave_connections: HashSet<CaveId>,
    is_small: bool,
}

impl Cave {
    pub fn new(id: String, is_small: bool) -> Cave {
        Cave {
            id,
            cave_connections: HashSet::new(),
            is_small,
        }
    }
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        let start_id = "start".to_string();
        let start = Cave::new(start_id.clone(), false);
        let mut nodes = HashMap::new();
        nodes.insert(start_id.clone(), start);

        CaveSystem {
            start_id,
            cave_map: nodes,
        }
    }

    pub fn get_start_id(&self) -> CaveId {
        self.start_id.clone()
    }

    pub fn get_adjacent_cave_ids(&self, id: &CaveId) -> Vec<CaveId> {
        let mut adjacent_cave_ids = Vec::new();

        if let Some(cave) = self.cave_map.get(id) {
            for cave_id in cave.cave_connections.iter() {
                adjacent_cave_ids.push(cave_id.clone());
            }
        }
        adjacent_cave_ids
    }

    pub fn connect_caves(&mut self, start_id: &String, end_id: &String) {
        self.add_cave(start_id, is_small_cave(start_id));
        self.add_cave(end_id, is_small_cave(end_id));

        let start_cave = self.cave_map.get_mut(start_id).unwrap();
        if let None = start_cave.cave_connections.get(end_id) {
            start_cave.cave_connections.insert(end_id.clone());
        }

        let end_cave = self.cave_map.get_mut(end_id).unwrap();
        if let None = end_cave.cave_connections.get(start_id) {
            end_cave.cave_connections.insert(start_id.clone());
        }
    }

    pub fn add_cave(&mut self, id: &String, is_small: bool) {
        if let None = self.cave_map.get(id) {
            self.cave_map
                .insert(id.clone(), Cave::new(id.clone(), is_small));
        }
    }
}

pub fn is_small_cave(id: &String) -> bool {
    id.chars().fold(true, |result, char| {
        result && char.to_ascii_lowercase() == char
    })
}
