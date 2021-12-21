#![feature(test)]

use crate::parse_input::{parse, read_main, ParseOutput};
use std::cmp::max;

pub mod parse_input;

type Solution = u64;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let (mut p1_pos, mut p2_pos) = parse_output.clone();
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice: i32 = 1;

    loop {
        advance_player(&mut p1_pos, &mut p1_score, &mut dice);

        if p1_score >= 1000 {
            return ((dice - 1) * p2_score) as u64;
        }

        advance_player(&mut p2_pos, &mut p2_score, &mut dice);

        if p2_score >= 1000 {
            return ((dice - 1) * p1_score) as u64;
        }
    }
}

fn advance_player(p_pos: &mut i32, p_score: &mut i32, dice: &mut i32) {
    *p_pos += get_dice_roll_value(3, dice);
    *p_pos = ((*p_pos - 1) % 10) + 1;
    *p_score += *p_pos;
}

fn get_dice_roll_value(times: i32, amount_of_dice_roll: &mut i32) -> i32 {
    let mut value = 0;
    for _ in 0..times {
        value += ((*amount_of_dice_roll - 1) % 100) + 1;
        *amount_of_dice_roll += 1;
    }
    value
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let (p1, p2) = parse_output.clone();
    let possible_next_field_values = get_possible_next_field_values();
    let possible_dice_value_frequency = [1, 3, 6, 7, 6, 3, 1]; // 3x1..3 -> 1x3, 3x4, 6x5, 7x6, 6x7, 3x8, 1x9

    let p1_start = p1 as u64;
    let p2_start = p2 as u64;
    let mut p1_won: u64 = 0;
    let mut p2_won: u64 = 0;
    traverse_options(
        p1_start,
        0,
        p2_start,
        0,
        1,
        &mut p1_won,
        &mut p2_won,
        true,
        &possible_next_field_values,
        &possible_dice_value_frequency,
    );

    max(p1_won, p2_won)
}

fn traverse_options(
    p1_pos: u64,
    p1_val: u64,
    p2_pos: u64,
    p2_val: u64,
    possible_paths: u64,
    won_p1: &mut u64,
    won_p2: &mut u64,
    next_player_is_1: bool,
    possible_next_values_map: &[[u64; 7]; 10],
    possible_dice_value_frequency: &[u64; 7],
) {
    if next_player_is_1 {
        if p2_val >= 21 {
            *won_p2 += possible_paths;
            return;
        }
        let possible_next_values = possible_next_values_map[p1_pos as usize - 1];
        for i in 0..7 {
            let next_pos = &possible_next_values[i];
            let next_pos_freq = &possible_dice_value_frequency[i];

            traverse_options(
                *next_pos,
                p1_val + *next_pos,
                p2_pos,
                p2_val,
                possible_paths * *next_pos_freq,
                won_p1,
                won_p2,
                false,
                possible_next_values_map,
                possible_dice_value_frequency,
            );
        }
    } else {
        if p1_val >= 21 {
            *won_p1 += possible_paths;
            return;
        }
        let possible_next_values = possible_next_values_map[p2_pos as usize - 1];
        for i in 0..7 {
            let next_pos = &possible_next_values[i];
            let next_pos_freq = &possible_dice_value_frequency[i];

            traverse_options(
                p1_pos,
                p1_val,
                *next_pos,
                p2_val + *next_pos,
                possible_paths * *next_pos_freq,
                won_p1,
                won_p2,
                true,
                possible_next_values_map,
                possible_dice_value_frequency,
            );
        }
    }
}

fn get_possible_next_field_values() -> [[u64; 7]; 10] {
    [
        [4, 5, 6, 7, 8, 9, 10], // Start pos 1
        [5, 6, 7, 8, 9, 10, 1], // Start pos 2
        [6, 7, 8, 9, 10, 1, 2], // Start pos 3
        [7, 8, 9, 10, 1, 2, 3], // Start pos 4
        [8, 9, 10, 1, 2, 3, 4], // Start pos 5
        [9, 10, 1, 2, 3, 4, 5], // Start pos 6
        [10, 1, 2, 3, 4, 5, 6], // Start pos 7
        [1, 2, 3, 4, 5, 6, 7],  // Start pos 8
        [2, 3, 4, 5, 6, 7, 8],  // Start pos 9
        [3, 4, 5, 6, 7, 8, 9],  // Start pos 10
    ]
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

        assert_eq!(part_1(&parse_output), 739785);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 444356092776315);
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
            assert_eq!(part_1(black_box(&parse_output)), 918081);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 158631174219251);
        });
    }
}
