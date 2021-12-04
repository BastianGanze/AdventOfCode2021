#![feature(drain_filter)]
#![feature(test)]

use crate::bingo_board::{parse_board_data, BingoBoard, MarkNumberResult};
use std::collections::HashMap;

pub mod bingo_board;

fn main() {
    match std::fs::read_to_string("./src/04.txt") {
        Ok(text) => {
            let (numbers, mut bingo_boards) = parse_board_data(&text);
            println!(
                "Solution is 1 {}",
                part_1(numbers.clone(), &mut bingo_boards.clone())
            );
            println!("Solution is 2 {}", part_2(numbers, &mut bingo_boards));
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

pub fn part_1(numbers: Vec<u32>, bingo_boards: &mut HashMap<u32, BingoBoard<5>>) -> u32 {
    for number in numbers {
        for board_id_that_one in play_game(number, bingo_boards) {
            return bingo_boards
                .get(&board_id_that_one)
                .unwrap()
                .get_sum_of_unmarked_fields()
                * number;
        }
    }
    return 0;
}

pub fn part_2(numbers: Vec<u32>, bingo_boards: &mut HashMap<u32, BingoBoard<5>>) -> u32 {
    let mut last_sum_some_of_unmarked_fields: u32 = 0;
    let mut last_number_played: u32 = 0;

    for number in numbers {
        for board_id_that_one in play_game(number, bingo_boards) {
            let won_board = bingo_boards.remove(&board_id_that_one).unwrap();
            last_sum_some_of_unmarked_fields = won_board.get_sum_of_unmarked_fields();
            last_number_played = number;
        }
    }

    return last_sum_some_of_unmarked_fields * last_number_played;
}

pub fn play_game<const N: usize>(
    number: u32,
    bingo_boards: &mut HashMap<u32, BingoBoard<N>>,
) -> Vec<u32> {
    let mut boards_that_won: Vec<u32> = Vec::new();
    for (board_id, board) in bingo_boards.into_iter() {
        if board.mark_number(number) == MarkNumberResult::GameWon {
            boards_that_won.push(*board_id);
        }
    }
    return boards_that_won;
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::bingo_board::parse_board_data;
    use test::Bencher;

    #[test]
    pub fn play_test_game() {
        match std::fs::read_to_string("./src/test.txt") {
            Ok(text) => {
                let (numbers, mut bingo_boards) = parse_board_data::<5>(&text);
                for number in numbers {
                    for board_id in play_game(number, &mut bingo_boards) {
                        assert_eq!(
                            bingo_boards
                                .get(&board_id)
                                .unwrap()
                                .get_sum_of_unmarked_fields()
                                * number,
                            4512
                        );
                    }
                    return;
                }
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        match std::fs::read_to_string("./src/04.txt") {
            Ok(text) => {
                b.iter(|| {
                    let (_, _) = parse_board_data::<5>(&text);
                });
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        match std::fs::read_to_string("./src/04.txt") {
            Ok(text) => {
                let (numbers, bingo_boards) = parse_board_data(&text);
                b.iter(|| {
                    assert_eq!(part_1(numbers.clone(), &mut bingo_boards.clone()), 82440);
                });
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        match std::fs::read_to_string("./src/04.txt") {
            Ok(text) => {
                let (numbers, bingo_boards) = parse_board_data(&text);
                b.iter(|| {
                    assert_eq!(part_2(numbers.clone(), &mut bingo_boards.clone()), 20774);
                });
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}
