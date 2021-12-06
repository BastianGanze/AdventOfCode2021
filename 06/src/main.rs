#![feature(test)]

use std::collections::VecDeque;

type ParseOutput = Vec<usize>;
type Solution = u64;
type FishSchool = VecDeque<u64>;

fn main() {
    let parse_output = parse_input_file("src/06.txt");
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn parse_input_file(input_file: &str) -> ParseOutput {
    std::fs::read_to_string(input_file)
        .unwrap()
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect()
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut fish_school = create_fish_school(parse_output);

    advance_days(&mut fish_school, 80);

    fish_school.into_iter().sum()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut fish_school = create_fish_school(parse_output);

    advance_days(&mut fish_school, 256);

    fish_school.into_iter().sum()
}

fn create_fish_school(parse_output: &ParseOutput) -> VecDeque<u64> {
    let mut fish_start: [u64; 9] = [0; 9];
    for fi in parse_output {
        fish_start[*fi] += 1;
    }
    let fish_school: VecDeque<u64> = VecDeque::from(fish_start);

    fish_school
}

fn advance_days(fish_school: &mut FishSchool, days: u32) {
    for _ in 0..days {
        let fish_day_0 = fish_school.pop_front().unwrap(); // 0
        let fish_day_8 = fish_school.pop_back().unwrap(); // 8
        let fish_day_7 = fish_school.pop_back().unwrap(); // 7

        fish_school.push_back(fish_day_7 + fish_day_0); // 6
        fish_school.push_back(fish_day_8); // 7
        fish_school.push_back(fish_day_0); // 8
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse_input_file("src/test.txt");

        assert_eq!(part_1(&parse_output), 5934);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_input_file("src/test.txt");

        assert_eq!(part_2(&parse_output), 26984457539);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_input_file("src/06.txt");
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse_input_file("src/06.txt");
        b.iter(|| {
            part_1(&parse_output);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_input_file("src/06.txt");
        b.iter(|| {
            part_2(&parse_output);
        });
    }
}
