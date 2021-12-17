#![feature(test)]

use crate::package::{read_package, Package, PackageType};

use crate::bit_reader::BitReaderBufferType;
use crate::parse_input::{parse, read_main, ParseOutput};
use std::cmp::{max, min};

pub mod bit_reader;
pub mod package;
pub mod parse_input;

type Solution = BitReaderBufferType;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn sum_versions(package: &Package) -> BitReaderBufferType {
    match &package.package_type {
        PackageType::Literal(_) => package.version,
        PackageType::Operator(_, sub_packages) => {
            let mut sum = package.version;
            for package in sub_packages {
                sum += sum_versions(&package);
            }
            sum
        }
    }
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut bit_reader = parse_output.clone();

    bit_reader.prefill();
    let main_package = read_package(&mut bit_reader);

    sum_versions(&main_package)
}

fn eval_expression(package: &Package) -> BitReaderBufferType {
    match &package.package_type {
        PackageType::Literal(value) => value.clone(),
        PackageType::Operator(operator_type, sub_packages) => match operator_type {
            0 => sub_packages
                .iter()
                .fold(0, |acc, p| acc + eval_expression(p)),
            1 => sub_packages
                .iter()
                .fold(1, |acc, p| acc * eval_expression(p)),
            2 => sub_packages
                .iter()
                .fold(BitReaderBufferType::MAX, |acc, p| {
                    min(acc, eval_expression(p))
                }),
            3 => sub_packages
                .iter()
                .fold(BitReaderBufferType::MIN, |acc, p| {
                    max(acc, eval_expression(p))
                }),
            5 => {
                if eval_expression(&sub_packages[0]) > eval_expression(&sub_packages[1]) {
                    1
                } else {
                    0
                }
            }
            6 => {
                if eval_expression(&sub_packages[0]) < eval_expression(&sub_packages[1]) {
                    1
                } else {
                    0
                }
            }
            7 => {
                if eval_expression(&sub_packages[0]) == eval_expression(&sub_packages[1]) {
                    1
                } else {
                    0
                }
            }
            _ => unreachable!(),
        },
    }
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut bit_reader = parse_output.clone();

    bit_reader.prefill();
    let main_package = read_package(&mut bit_reader);

    eval_expression(&main_package)
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
        assert_eq!(part_1(&parse_output), 31);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&"9C0141080250320F1802104A08".to_string());
        assert_eq!(part_2(&parse_output), 1);
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
            assert_eq!(part_1(black_box(&parse_output)), 1012);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 2223947372407);
        });
    }
}
