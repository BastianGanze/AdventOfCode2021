#![feature(test)]

use crate::parse_input::{parse_main, ParseOutput};
use std::collections::VecDeque;
pub mod parse_input;

type Solution = u64;

fn main() {
    let parse_output = parse_main();
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution = 0;
    for line in parse_output {
        let mut stack: VecDeque<char> = VecDeque::new();
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push_back(char),
                ')' | ']' | '}' | '>' => {
                    let last_open_parenthesis = stack.pop_back().unwrap();
                    if !do_parenthesis_match(last_open_parenthesis, char) {
                        solution += get_solution_part_1_rating(char);
                    }
                }
                e => unreachable!("Tried to match {}", e),
            }
        }
    }

    solution
}

fn do_parenthesis_match(opening: char, closing: char) -> bool {
    match opening {
        '(' => closing == ')',
        '[' => closing == ']',
        '{' => closing == '}',
        '<' => closing == '>',
        e => unreachable!("Tried to match {}", e),
    }
}

fn get_solution_part_1_rating(closing_parenthesis: char) -> u64 {
    match closing_parenthesis {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        e => unreachable!("Tried to match {}", e),
    }
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut stack_solutions = Vec::new();
    'line_loop: for line in parse_output {
        let mut stack: VecDeque<char> = VecDeque::new();
        for char in line.chars() {
            match char {
                '(' | '[' | '{' | '<' => stack.push_back(char),
                ')' | ']' | '}' | '>' => {
                    let last_open_parenthesis = stack.pop_back().unwrap();
                    if !do_parenthesis_match(last_open_parenthesis, char) {
                        // Discard line
                        continue 'line_loop;
                    }
                }
                e => unreachable!("Tried to match {}", e),
            }
        }

        let mut stack_solution = 0;
        for char in stack.iter().rev() {
            stack_solution *= 5;
            stack_solution += get_solution_part_2_rating(*char);
        }
        stack_solutions.push(stack_solution)
    }
    stack_solutions.sort();

    let middle = stack_solutions.len() / 2;

    stack_solutions[middle]
}

fn get_solution_part_2_rating(closing_parenthesis: char) -> u64 {
    match closing_parenthesis {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        e => unreachable!("Tried to match {}", e),
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse_main, parse_test};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse_test();
        assert_eq!(part_1(&parse_output), 26397);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_test();
        assert_eq!(part_2(&parse_output), 288957);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_main();
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 316851);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 2182912364);
        });
    }
}
