#![feature(test)]
#![feature(int_abs_diff)]

use crate::grid::Field;
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

fn part_1(parse_output: &ParseOutput) -> Solution {
    calc_lowest_risk(parse_output.clone())
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut grid = parse_output.clone();
    grid.set_wrap((5, 5));
    calc_lowest_risk(grid)
}

fn calc_lowest_risk(mut grid: ParseOutput) -> Solution {
    let grid_size = grid.get_size();

    let mut open_fields = BinaryHeap::<Field>::new();
    let mut lowest_risk_level = 0;

    let start = Field::new((0, 0), 0);
    grid.mark_field(0, 0);

    open_fields.push(start);

    'outer: while open_fields.len() > 0 {
        let current_field = open_fields.pop().unwrap();

        let neighbours =
            grid.get_unmarked_neighbours(current_field.coordinate.0, current_field.coordinate.1);
        for (y, x, neighbour_cost) in neighbours {
            let new_cost = current_field.cost + neighbour_cost as u32;

            if y == grid_size.0 - 1 && x == grid_size.1 - 1 {
                lowest_risk_level = new_cost;
                break 'outer;
            }

            grid.mark_field(y, x);
            open_fields.push(Field::new((y, x), new_cost));
        }
    }

    lowest_risk_level
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
            assert_eq!(part_2(black_box(&parse_output)), 2976);
        });
    }
}
