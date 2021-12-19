#![feature(test)]

use crate::parse_input::{parse, read_main, ParseOutput};
use std::cmp::max;

pub mod parse_input;
pub mod snail_fish_number;

type Solution = u32;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let (mut snail_fish_numbers, mut id_gen) = parse_output.clone();
    let mut first_number = snail_fish_numbers.pop_front().unwrap();

    for snail_fish_number in snail_fish_numbers.drain(..) {
        first_number.add(&snail_fish_number, &mut id_gen);
    }

    first_number.magnitude()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let (snail_fish_numbers, mut id_gen) = parse_output.clone();
    let mut max_magnitude = 0;

    for (is1, s1) in snail_fish_numbers.iter().enumerate() {
        for (is2, s2) in snail_fish_numbers.iter().enumerate() {
            if is1 == is2 {
                continue;
            };
            let mut bla = s2.clone();
            bla.add(&s1, &mut id_gen);
            max_magnitude = max(max_magnitude, bla.magnitude());
        }
    }

    max_magnitude
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse, read_main, read_test};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let (mut snail_fish_numbers, mut id_gen) = parse(&read_test()).clone();
        let mut first_number = snail_fish_numbers.pop_front().unwrap();

        for snail_fish_number in snail_fish_numbers.drain(..) {
            first_number.add(&snail_fish_number, &mut id_gen);
        }
        assert_eq!(
            first_number.to_string(),
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
        assert_eq!(first_number.magnitude(), 4140);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 195);
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
            assert_eq!(part_1(black_box(&parse_output)), 4072);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 4483);
        });
    }
}
