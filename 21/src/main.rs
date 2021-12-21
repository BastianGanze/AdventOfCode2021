#![feature(test)]

use crate::parse_input::{parse, read_main, ParseOutput};

pub mod parse_input;

type Solution = i32;

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
        println!("old player1  pos {}", p1_pos);
        advance_player(&mut p1_pos, &mut p1_score, &mut dice);
        println!("new player1  pos {}", p1_pos);

        if p1_score >= 1000 {
            return (dice - 1) * p2_score;
        }

        println!("old player2  pos {}", p2_pos);
        advance_player(&mut p2_pos, &mut p2_score, &mut dice);
        println!("new player2  pos {}", p2_pos);

        if p2_score >= 1000 {
            return (dice - 1) * p1_score;
        }
    }
}

fn advance_player(p_pos: &mut i32, p_score: &mut i32, dice: &mut i32) {
    *p_pos += get_dice_roll_value(3, dice);
    println!("{} -> {}", p_pos, ((*p_pos - 1) % 10) + 1);
    *p_pos = ((*p_pos - 1) % 10) + 1;

    *p_score += *p_pos;
}

fn get_dice_roll_value(times: i32, amount_of_dice_roll: &mut i32) -> i32 {
    let mut value = 0;
    print!("adding ");
    for _ in 0..times {
        value += ((*amount_of_dice_roll - 1) % 100) + 1;
        print!("{} ", ((*amount_of_dice_roll - 1) % 100) + 1);
        *amount_of_dice_roll += 1;
    }
    print!("  {}  \n", value);

    value
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
    pub fn test_part_1() {
        let parse_output = parse(&read_test());

        assert_eq!(part_1(&parse_output), 739785);
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
            assert_eq!(part_1(black_box(&parse_output)), 1620);
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
