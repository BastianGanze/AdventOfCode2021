use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq)]
pub enum CheckNumberResult {
    NumberDoesNotExist,
    NumberChecked,
    GameWon,
}

pub struct BingoBoard<const N: usize> {
    fields: HashMap<u32, (usize, usize)>,
    size: usize,
    sum_of_all_fields: u32,
    sum_of_all_marked_fields: u32,
    row_checked_count: [usize; N],
    column_checked_count: [usize; N],
}

impl<const N: usize> BingoBoard<N> {
    pub fn new(board_grid: &str) -> BingoBoard<N> {
        let rows: Vec<&str> = board_grid.split("\n").collect();
        let mut x;
        let mut y = 0;
        let mut sum_of_all_fields = 0;

        let mut fields = HashMap::new();

        for row in rows {
            let numbers: Vec<u32> = row
                .split(" ")
                .filter(|number| number.len() > 0)
                .map(|number| number.parse::<u32>().unwrap())
                .collect();
            x = 0;

            for number in numbers {
                sum_of_all_fields = sum_of_all_fields + number;
                fields.insert(number, (x, y));
                x = x + 1;
            }

            y = y + 1;
        }

        BingoBoard {
            fields,
            sum_of_all_fields,
            sum_of_all_marked_fields: 0,
            size: N,
            row_checked_count: [0; N],
            column_checked_count: [0; N],
        }
    }

    pub fn fields_to_string(&self) -> String {
        let mut out = String::new();
        for (number, (x, y)) in self.fields.iter() {
            out.push_str(format!("{}, {}, {}\n", number, x, y).as_str());
        }
        out
    }

    pub fn mark_number(&mut self, num: u32) -> CheckNumberResult {
        match self.fields.get(&num) {
            Some((x, y)) => {
                self.sum_of_all_marked_fields = self.sum_of_all_marked_fields + num;
                self.column_checked_count[*x] = self.column_checked_count[*x] + 1;
                self.row_checked_count[*y] = self.row_checked_count[*y] + 1;
                if self.column_checked_count[*x] == self.size
                    || self.row_checked_count[*y] == self.size
                {
                    return CheckNumberResult::GameWon;
                }
                CheckNumberResult::NumberChecked
            }
            None => CheckNumberResult::NumberDoesNotExist,
        }
    }

    pub fn get_sum_of_unmarked_fields(&self) -> u32 {
        self.sum_of_all_fields - self.sum_of_all_marked_fields
    }
}

pub fn parse_board_data<const N: usize>(input: &String) -> (Vec<u32>, HashMap<u32, BingoBoard<N>>) {
    let mut instructions: VecDeque<&str> = input.split("\n\n").collect();
    let mut id = 0;
    let numbers: Vec<u32> = instructions
        .pop_front()
        .unwrap()
        .split(",")
        .map(|number| number.parse::<u32>().unwrap())
        .collect();

    let bingo_boards = instructions
        .into_iter()
        .map(|board_grid| {
            id = id + 1;
            (id, BingoBoard::new(board_grid))
        })
        .collect();
    (numbers, bingo_boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_board_data_test() {
        match std::fs::read_to_string("./src/test.txt") {
            Ok(text) => {
                let (numbers, boards) = parse_board_data::<5>(&text);
                assert_eq!(
                    numbers,
                    [
                        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22,
                        18, 20, 8, 19, 3, 26, 1
                    ]
                );
                assert_eq!(boards.len(), 3);
            }
            Err(err) => {
                panic!("{:?}", err);
            }
        }
    }

    #[test]
    fn bingo_board_basic_test() {
        let board = "22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19";
        let mut bingo_board: BingoBoard<5> = BingoBoard::new(board);

        assert_eq!(bingo_board.mark_number(2), CheckNumberResult::NumberChecked);
        assert_eq!(
            bingo_board.mark_number(50),
            CheckNumberResult::NumberDoesNotExist
        );
    }

    #[test]
    fn bingo_win_1_test() {
        let board = "22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19";
        let sum_of_all_numbers = 300;
        let mut bingo_board: BingoBoard<5> = BingoBoard::new(board);

        bingo_board.mark_number(22);
        bingo_board.mark_number(8);
        bingo_board.mark_number(21);
        bingo_board.mark_number(6);
        assert_eq!(bingo_board.mark_number(1), CheckNumberResult::GameWon);
        assert_eq!(
            bingo_board.get_sum_of_unmarked_fields(),
            sum_of_all_numbers - 22 - 8 - 21 - 6 - 1
        )
    }

    #[test]
    fn bingo_win_2_test() {
        let board = "22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19";
        let mut bingo_board: BingoBoard<5> = BingoBoard::new(board);
        let sum_of_all_numbers = 300;
        bingo_board.mark_number(1);
        bingo_board.mark_number(12);
        bingo_board.mark_number(20);
        bingo_board.mark_number(15);
        assert_eq!(bingo_board.mark_number(19), CheckNumberResult::GameWon);
        assert_eq!(
            bingo_board.get_sum_of_unmarked_fields(),
            sum_of_all_numbers - 1 - 12 - 20 - 15 - 19
        )
    }
}
