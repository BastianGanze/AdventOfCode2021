use std::cmp::Ordering;
use std::fs::{read_link, File};

#[derive(Clone)]
pub struct Grid {
    pub fields: Vec<Vec<(i32, u8)>>,
    size: (usize, usize),
    default_element: i32,
    wraped_size: (usize, usize),
    wrap: (usize, usize),
}

impl Grid {
    pub fn new(size: (usize, usize), default_element: i32) -> Grid {
        Grid {
            fields: vec![vec![(default_element.clone(), 0); size.1]; size.0],
            size: size.clone(),
            default_element,
            wraped_size: size,
            wrap: (1, 1),
        }
    }

    pub fn set_wrap(&mut self, wrap: (usize, usize)) {
        self.wraped_size = (self.size.0 * wrap.0, self.size.1 * wrap.1);
        self.wrap = wrap;
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.wraped_size.clone()
    }

    pub(self) fn is_field_in_bounds(&self, y: i32, x: i32) -> bool {
        y >= 0 && x >= 0 && y < self.wraped_size.0 as i32 && x < self.wraped_size.1 as i32
    }

    pub fn get_field_cost(&self, y: usize, x: usize) {
        let wrap = (y / self.size.0) + (x / self.size.0);
        let real_y = y % self.size.0;
        let real_x = x % self.size.1;
        let value = self.fields[real_y][real_x].clone();

        (((value.0 + wrap + 1) % 9) - 1, value.1);
    }

    pub fn set_field(&mut self, y: usize, x: usize, num: i32) {
        if !self.is_field_in_bounds(y as i32, x as i32) {
            return;
        }

        self.fields[y][x] = num;
    }

    pub fn get_left(&self, y: usize, x: usize) -> Option<i32> {
        if !self.is_field_in_bounds(y as i32, x as i32 - 1) {
            return None;
        }

        Some(self.get_field(y, x).clone())
    }

    pub fn get_right(&self, y: usize, x: usize) -> Option<i32> {
        if !self.is_field_in_bounds(y as i32, x as i32 + 1) {
            return None;
        }

        Some(self.fields[y][x + 1].clone())
    }

    pub fn get_top(&self, y: usize, x: usize) -> Option<T> {
        if !self.is_field_in_bounds(y as i32 - 1, x as i32) {
            return None;
        }

        Some(self.fields[y - 1][x].clone())
    }

    pub fn get_bottom(&self, y: usize, x: usize) -> Option<T> {
        if !self.is_field_in_bounds(y as i32 + 1, x as i32) {
            return None;
        }

        Some(self.fields[y + 1][x].clone())
    }

    pub fn get_untouched_neighbours(&self, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();
        let (left_o, top_o, right_o, bottom_o) = (
            self.get_left(y, x),
            self.get_top(y, x),
            self.get_right(y, x),
            self.get_bottom(y, x),
        );
        if let Some(n) = left_o {
            if n == self.default_element {
                neighbours.push((y, x - 1));
            }
        };
        if let Some(n) = right_o {
            if n == self.default_element {
                neighbours.push((y, x + 1));
            }
        };
        if let Some(n) = top_o {
            if n == self.default_element {
                neighbours.push((y - 1, x));
            }
        };
        if let Some(n) = bottom_o {
            if n == self.default_element {
                neighbours.push((y + 1, x));
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
