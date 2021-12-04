#![feature(drain_filter)]

use crate::bingo_board::{parse_board_data, BingoBoard, MarkNumberResult};
use std::collections::HashMap;

pub mod bingo_board;

fn main() {
    match std::fs::read_to_string("./src/04.txt") {
        Ok(text) => {
            let (numbers, mut bingo_boards) = parse_board_data(&text);
            part_1(numbers.clone(), &mut bingo_boards.clone());
            part_2(numbers, &mut bingo_boards);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn part_1(numbers: Vec<u32>, bingo_boards: &mut HashMap<u32, BingoBoard<5>>) {
    for number in numbers {
        for board_id_that_one in play_game(number, bingo_boards) {
            println!(
                "Solution is 1 {}",
                bingo_boards
                    .get(&board_id_that_one)
                    .unwrap()
                    .get_sum_of_unmarked_fields()
                    * number
            );
            return;
        }
    }
}

fn part_2(numbers: Vec<u32>, bingo_boards: &mut HashMap<u32, BingoBoard<5>>) {
    let mut last_sum_some_of_unmarked_fields: u32 = 0;
    let mut last_number_played: u32 = 0;

    for number in numbers {
        for board_id_that_one in play_game(number, bingo_boards) {
            let won_board = bingo_boards.remove(&board_id_that_one).unwrap();
            last_sum_some_of_unmarked_fields = won_board.get_sum_of_unmarked_fields();
            last_number_played = number;
        }
    }

    println!(
        "Solution is 2 {}",
        last_sum_some_of_unmarked_fields * last_number_played
    );
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
    use super::*;
    use crate::bingo_board::parse_board_data;

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
}
