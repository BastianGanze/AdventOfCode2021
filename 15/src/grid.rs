use std::cmp::Ordering;

pub type CostType = usize;
pub type MaskType = i32;

#[derive(Clone)]
pub struct Grid {
    pub fields: Vec<Vec<(CostType, MaskType)>>,
    size: (usize, usize),
    wrapped_size: (usize, usize),
    wrap: (usize, usize),
}

pub fn manhattan_distance(p1: (usize, usize), p2: (usize, usize)) -> u32 {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as u32
}

impl Grid {
    pub fn new(size: (usize, usize)) -> Grid {
        Grid {
            fields: vec![vec![(0, 0); size.1]; size.0],
            size: size.clone(),
            wrapped_size: size,
            wrap: (1, 1),
        }
    }

    pub fn set_wrap(&mut self, wrap: (usize, usize)) {
        self.wrapped_size = (self.size.0 * wrap.0, self.size.1 * wrap.1);
        self.wrap = wrap;
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.wrapped_size.clone()
    }

    pub(self) fn is_field_in_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && x >= 0 && y < self.wrapped_size.0 as i32 && x < self.wrapped_size.1 as i32
    }

    pub fn get_field_cost(&self, y: usize, x: usize) -> CostType {
        let wrap = (y / self.size.0) + (x / self.size.0);
        let real_y = y % self.size.0;
        let real_x = x % self.size.1;
        let value = self.fields[real_y][real_x].clone();

        (((value.0 + wrap) - 1) % 9) + 1
    }

    pub fn apply_field_mask(&mut self, y: usize, x: usize, mask: MaskType) {
        self.fields[y % self.size.0][x % self.size.1].1 |= mask;
    }

    pub fn get_field_mask(&self, y: usize, x: usize) -> MaskType {
        self.fields[y % self.size.0][x % self.size.1].1
    }

    pub fn set_field_cost(&mut self, y: usize, x: usize, cost: CostType) {
        if !self.is_field_in_bounds(y as i32, x as i32) {
            return;
        }

        self.fields[y][x].0 = cost;
    }

    pub fn mark_field(&mut self, y: usize, x: usize) {
        let field = 1 << (((y / self.size.0) * self.wrap.1) + (x / self.size.1));

        self.apply_field_mask(y, x, field);
    }

    pub fn is_field_marked(&self, y: usize, x: usize) -> bool {
        let field = 1 << (((y / self.size.0) * self.wrap.1) + (x / self.size.1));

        (self.get_field_mask(y, x) & field).count_ones() == 1
    }

    pub fn get_left(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32, x as i32 - 1) {
            return None;
        }

        Some(self.get_field_cost(y, x - 1))
    }

    pub fn get_right(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32, x as i32 + 1) {
            return None;
        }

        Some(self.get_field_cost(y, x + 1))
    }

    pub fn get_top(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32 - 1, x as i32) {
            return None;
        }

        Some(self.get_field_cost(y - 1, x))
    }

    pub fn get_bottom(&self, y: usize, x: usize) -> Option<CostType> {
        if !self.is_field_in_bounds(y as i32 + 1, x as i32) {
            return None;
        }

        Some(self.get_field_cost(y + 1, x))
    }

    pub fn get_unmarked_neighbours(&self, y: usize, x: usize) -> Vec<(usize, usize, CostType)> {
        let mut neighbours = Vec::new();
        let (left_o, top_o, right_o, bottom_o) = (
            self.get_left(y, x),
            self.get_top(y, x),
            self.get_right(y, x),
            self.get_bottom(y, x),
        );
        if let Some(n) = left_o {
            if !self.is_field_marked(y, x - 1) {
                neighbours.push((y, x - 1, n));
            }
        };
        if let Some(n) = right_o {
            if !self.is_field_marked(y, x + 1) {
                neighbours.push((y, x + 1, n));
            }
        };
        if let Some(n) = top_o {
            if !self.is_field_marked(y - 1, x) {
                neighbours.push((y - 1, x, n));
            }
        };
        if let Some(n) = bottom_o {
            if !self.is_field_marked(y + 1, x) {
                neighbours.push((y + 1, x, n));
            }
        };

        neighbours
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Field {
    pub coordinate: (usize, usize),
    pub cost: u32,
}

impl Field {
    pub fn new(coordinate: (usize, usize), cost: u32) -> Field {
        Field { coordinate, cost }
    }
}

impl PartialOrd<Self> for Field {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost > other.cost {
            return Some(Ordering::Less);
        }

        if self.cost == other.cost {
            return Some(Ordering::Equal);
        }

        return Some(Ordering::Greater);
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            return Ordering::Less;
        }

        if self.cost == other.cost {
            return Ordering::Equal;
        }

        return Ordering::Greater;
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use crate::parse_input::{parse, read_test};
    use std::collections::BinaryHeap;

    #[test]
    pub fn test_grid() {
        let mut grid = parse(&read_test());
        grid.set_wrap((5, 5));

        assert_eq!(grid.get_field_cost(0, 0), 1);
        assert_eq!(grid.get_field_cost(0, 1), 1);
        assert_eq!(grid.get_field_cost(0, 2), 6);
        assert_eq!(grid.get_field_cost(0, 3), 3);
        assert_eq!(grid.get_field_cost(0, 9), 2);
        assert_eq!(grid.get_field_cost(9, 9), 1);

        assert_eq!(grid.get_field_cost(10, 0), 2);
        assert_eq!(grid.get_field_cost(10, 1), 2);
        assert_eq!(grid.get_field_cost(10, 2), 7);
        assert_eq!(grid.get_field_cost(10, 3), 4);
        assert_eq!(grid.get_field_cost(10, 9), 3);
        assert_eq!(grid.get_field_cost(19, 9), 2);

        assert_eq!(grid.get_field_cost(10, 10), 3);
        assert_eq!(grid.get_field_cost(10, 11), 3);
        assert_eq!(grid.get_field_cost(10, 12), 8);
        assert_eq!(grid.get_field_cost(10, 13), 5);
        assert_eq!(grid.get_field_cost(10, 19), 4);
        assert_eq!(grid.get_field_cost(19, 19), 3);

        assert_eq!(grid.get_field_cost(20, 10), 4);
        assert_eq!(grid.get_field_cost(20, 11), 4);
        assert_eq!(grid.get_field_cost(20, 12), 9);
        assert_eq!(grid.get_field_cost(20, 13), 6);
        assert_eq!(grid.get_field_cost(20, 19), 5);
        assert_eq!(grid.get_field_cost(29, 19), 4);

        assert_eq!(grid.get_field_cost(29, 29), 5);

        assert_eq!(grid.get_field_cost(49, 49), 9);

        assert_eq!(grid.is_field_in_bounds(49, 49), true);
        assert_eq!(grid.is_field_in_bounds(50, 50), false);

        assert_eq!(grid.get_unmarked_neighbours(19, 19).len(), 4);
    }

    #[test]
    pub fn test_heap() {
        let mut binary_heap = BinaryHeap::<Field>::new();

        binary_heap.push(Field::new((0, 0), 2));
        binary_heap.push(Field::new((1, 0), 5));
        binary_heap.push(Field::new((2, 0), 10));
        binary_heap.push(Field::new((3, 0), 1));
        assert_eq!(binary_heap.pop().unwrap().cost, 1);
        assert_eq!(binary_heap.pop().unwrap().cost, 2);
        assert_eq!(binary_heap.pop().unwrap().cost, 5);
        assert_eq!(binary_heap.pop().unwrap().cost, 10);
    }

    #[test]
    pub fn manhatten_distance_test() {
        assert_eq!(manhattan_distance((0, 0), (50, 50)), 100);
        assert_eq!(manhattan_distance((80, 40), (3, 3)), 114);
    }
}
