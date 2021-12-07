#![feature(test)]

use crate::parse_input::{parse_main, ParseOutput};

pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse_main();
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    todo!()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    todo!()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse_main, parse_test};
    use test::Bencher;

    #[test]
    pub fn test_part_1() {
        let parse_output = parse_test();
        assert_eq!(part_1(&parse_output), 0);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_test();
        assert_eq!(part_2(&parse_output), 0);
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
            part_1(&parse_output);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(|| {
            part_2(&parse_output);
        });
    }
}
