#![feature(test)]

use crate::grid::{Field, Grid};
use crate::parse_input::{parse, read_main, ParseOutput};
use std::collections::BinaryHeap;

pub mod grid;
pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(grid: &ParseOutput) -> Solution {
    let grid_size = grid.get_size();
    let mut mark_grid = Grid::<u32>::new(grid_size, u32::MAX);
    let mut open_fields = BinaryHeap::<Field>::new();
    let mut lowest_cost = 0;
    let start = Field {
        coordinate: (0, 0),
        cost: 0,
    };
    open_fields.push(start);

    'outer: while open_fields.len() > 0 {
        let current_field = open_fields.pop().unwrap();

        let neighbours = mark_grid
            .get_untouched_neighbours(current_field.coordinate.0, current_field.coordinate.1);

        for (y, x) in neighbours {
            let new_cost = current_field.cost + grid.fields[y][x] as u32;
            if y == grid_size.0 - 1 && x == grid_size.1 - 1 {
                lowest_cost = new_cost;
                break 'outer;
            }

            mark_grid.fields[y][x] = new_cost;
            open_fields.push(Field::new((y, x), new_cost));
        }
    }

    lowest_cost
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    todo!()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse, read_main, read_test};
    use test::{black_box, Bencher};

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
    pub fn test_part_1() {
        let parse_output = parse(&read_test());
        assert_eq!(part_1(&parse_output), 40);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 315);
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
            assert_eq!(part_1(black_box(&parse_output)), 741);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 371);
        });
    }
}
