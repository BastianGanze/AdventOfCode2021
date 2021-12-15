#![feature(test)]
#![feature(linked_list_cursors)]
#![feature(map_try_insert)]

use crate::parse_input::{
    parse, read_main, BaseFrequencyMap, InstructionFrequencyMap, Instructions, ParseOutput, Polymer,
};
use std::cmp::{max, min};

pub mod parse_input;

type Solution = i64;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    build_polymer(parse_output, 10)
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    build_polymer(parse_output, 40)
}

fn build_polymer(parse_output: &ParseOutput, max_steps: u32) -> Solution {
    let (actual_instructions, mut instruction_frequency_map, mut base_frequency) =
        parse_output.clone();

    /*for (a, b) in actual_instructions.iter() {
        println!("{}{}: {}", a.0, a.1, b.to_string());
    }*/

    for _step in 0..max_steps {
        build_step(
            &actual_instructions,
            &mut instruction_frequency_map,
            &mut base_frequency,
        );
    }
    let (min, max) = get_least_and_most_frequent(&base_frequency);

    max - min
}

fn get_least_and_most_frequent(base_frequency: &BaseFrequencyMap) -> (i64, i64) {
    base_frequency
        .iter()
        .fold((i64::MAX, i64::MIN), |acc, (_, freq)| {
            (min(acc.0, *freq), max(acc.1, *freq))
        })
}

fn build_step(
    actual_instructions: &Instructions,
    instruction_frequency_map: &mut InstructionFrequencyMap,
    base_frequency: &mut BaseFrequencyMap,
) {
    for (pair, frequency) in instruction_frequency_map.clone().iter() {
        let instruction = actual_instructions.get(pair).unwrap();
        for (other_pair, change) in instruction.count_change.iter() {
            *instruction_frequency_map.get_mut(other_pair).unwrap() +=
                (*frequency) * (*change as i64);
        }
        for (char, change) in instruction.base_count_change.iter() {
            *base_frequency.get_mut(char).unwrap() += (*frequency) * (*change as i64);
        }
    }
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
            assert_eq!(part_2(black_box(&parse_output)), 3390034818249);
        });
    }
}
