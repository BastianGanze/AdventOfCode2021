#![feature(test)]

use crate::parse_input::{parse_main, ParseOutput};

pub mod grid;
pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse_main();
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut grid = parse_output.clone();
    let (height, width) = grid.dimensions();
    let steps = 100;
    let mut flash_count: u32 = 0;

    for _ in 0..steps {
        simulate_step(&mut grid, height, width, &mut flash_count);
    }

    flash_count
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut grid = parse_output.clone();
    let (height, width) = grid.dimensions();
    let grid_field_count = (height * width) as u32;
    let mut step: u32 = 1;

    loop {
        let mut flash_count: u32 = 0;
        simulate_step(&mut grid, height, width, &mut flash_count);
        if flash_count == grid_field_count {
            break step;
        }
        step += 1;
    }
}

fn simulate_step(grid: &mut ParseOutput, height: i16, width: i16, flash_count: &mut u32) {
    for y in 0..height {
        for x in 0..width {
            grid.inc_field(y, x);
        }
    }

    for y in 0..height {
        for x in 0..width {
            if let Some(num) = grid.get_field(y, x) {
                if num > 9 {
                    flash(grid, y, x, flash_count);
                }
            }
        }
    }
}

fn flash(grid: &mut ParseOutput, y: i16, x: i16, flash_count: &mut u32) {
    grid.set_field(y, x, 0);
    *flash_count += 1;

    increment_energies_of_adjacent_fields(grid, y, x);

    for d_y in [-1, 0, 1] {
        for d_x in [-1, 0, 1] {
            if let Some(num) = grid.get_field(y - d_y, x - d_x) {
                if num > 9 {
                    flash(grid, y - d_y, x - d_x, flash_count);
                }
            }
        }
    }
}

fn increment_energies_of_adjacent_fields(grid: &mut ParseOutput, y: i16, x: i16) {
    grid.inc_field_if_not_0(y - 1, x - 1);
    grid.inc_field_if_not_0(y - 1, x);
    grid.inc_field_if_not_0(y - 1, x + 1);

    grid.inc_field_if_not_0(y, x - 1);
    grid.inc_field_if_not_0(y, x + 1);

    grid.inc_field_if_not_0(y + 1, x - 1);
    grid.inc_field_if_not_0(y + 1, x);
    grid.inc_field_if_not_0(y + 1, x + 1);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse_main, parse_test};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse_test();
        assert_eq!(part_1(&parse_output), 1656);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_test();
        assert_eq!(part_2(&parse_output), 195);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_main();
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 1620);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 371);
        });
    }
}
