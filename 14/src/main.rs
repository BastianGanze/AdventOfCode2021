#![feature(test)]
#![feature(linked_list_cursors)]

use crate::parse_input::{parse, read_main, BaseFrequencyMap, Instructions, ParseOutput, Polymer};
use std::cmp::{max, min};

pub mod parse_input;

type Solution = u64;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    build_polymer(parse_output, 9)
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    build_polymer(parse_output, 39)
}

fn read_next(polymer: &mut Polymer, index: usize, instructions: &Instructions) -> Option<char> {
    let (a, b) = (polymer[index], polymer[index + 1]);
    if let Some(base) = instructions.get(&(a, b)) {
        return Some(base.clone());
    }
    None
}

fn insert(polymer: &mut Polymer, new_base: char, insertion_index: usize) {
    polymer.insert(insertion_index, new_base);
}

fn build_polymer(parse_output: &ParseOutput, max_steps: u32) -> Solution {
    let (mut polymer, instructions, mut base_frequency) = parse_output.clone();

    let mut idx: usize = 0;
    let mut steps = 0;
    loop {
        if idx + 1 > polymer.len() - 1 {
            idx = 0;
            steps += 1;
            continue;
        }
        if steps > max_steps {
            break;
        }
        if let Some(base) = read_next(&mut polymer, idx, &instructions) {
            *base_frequency.get_mut(&base).unwrap() += 1;
            insert(&mut polymer, base, idx + 1);
            idx += 1;
        }
        idx += 1;
    }

    let (min, max) = base_frequency.drain().fold((u64::MAX, 0_u64), |acc, v| {
        (min(acc.0, v.1), max(acc.1, v.1))
    });

    return max - min;
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
        assert_eq!(part_1(&parse_output), 1588);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 2188189693529);
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
            assert_eq!(part_1(black_box(&parse_output)), 2937);
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
