use crate::snail_fish_number::SnailFishNumber;
use std::collections::VecDeque;

pub type ParseOutput = (VecDeque<SnailFishNumber>, u32);

pub fn read_main() -> String {
    read_file("src/18.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let snail_numbers = file.trim().split("\n");
    let mut id_gen = 0;
    (
        snail_numbers
            .map(|num| SnailFishNumber::from_string(num, &mut id_gen))
            .collect(),
        id_gen,
    )
}
