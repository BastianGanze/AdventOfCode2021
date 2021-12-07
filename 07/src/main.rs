#![feature(test)]
#![feature(int_abs_diff)]

use crate::parse_input::{parse_main, ParseOutput};

pub mod parse_input;

type Solution = usize;

fn main() {
    let parse_output = parse_main();
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut numbers = parse_output.clone();
    numbers.sort();

    let numbers_len = numbers.len();
    let median_i = numbers_len / 2;
    let median = numbers[median_i];

    let mut fuel_consumption = 0;
    for i in 0..median_i {
        fuel_consumption += median - numbers[i];
    }
    for i in median_i..numbers_len {
        fuel_consumption += numbers[i] - median;
    }

    fuel_consumption
}

fn calc_fuel_consumption(current_pos: usize, desired_pos: usize) -> usize {
    let diff = desired_pos.abs_diff(current_pos);
    diff * (diff + 1) / 2
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut numbers = parse_output.clone();
    let sum: usize = numbers.iter().sum();
    let numbers_len = numbers.len();
    let average = (sum as f32 / numbers_len as f32).floor() as usize;
    println!("{}", average);
    let mut fuel_consumption = 0;
    for number in numbers {
        fuel_consumption += calc_fuel_consumption(number, average);
    }

    fuel_consumption
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse_main, parse_test};
    use test::Bencher;

    #[test]
    pub fn test_fuel_consumption() {
        assert_eq!(calc_fuel_consumption(16, 5), 66);
        assert_eq!(calc_fuel_consumption(1, 5), 10);
        assert_eq!(calc_fuel_consumption(2, 5), 6);
        assert_eq!(calc_fuel_consumption(4, 5), 1);
    }

    #[test]
    pub fn test_part_1() {
        let parse_output = parse_test();
        assert_eq!(part_1(&parse_output), 37);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_test();
        assert_eq!(part_2(&parse_output), 168);
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
            part_1(&parse_output);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(|| {
            part_2(&parse_output);
        });
    }
}
