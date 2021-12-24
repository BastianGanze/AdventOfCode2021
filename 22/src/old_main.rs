#![feature(test)]
#![feature(int_abs_diff)]

use crate::parse_input::{parse, read_main, ParseOutput};
use std::collections::{HashMap, HashSet};

pub mod parse_input;

type Solution = i64;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let smoll_cuboids = parse_output.iter().filter(|c| {
        c.1 .0.abs() <= 50
            && c.1 .1.abs() <= 50
            && c.2 .0.abs() <= 50
            && c.2 .1.abs() <= 50
            && c.3 .0.abs() <= 50
            && c.3 .1.abs() <= 50
    });
    let mut slowpoke_hashmap: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut a = 0;
    for cuboid in smoll_cuboids {
        println!("{}", a);
        for x in cuboid.1 .0..=cuboid.1 .1 {
            for y in cuboid.2 .0..=cuboid.2 .1 {
                for z in cuboid.3 .0..=cuboid.3 .1 {
                    if cuboid.0 == true {
                        slowpoke_hashmap.insert((x, y, z));
                    } else {
                        slowpoke_hashmap.remove(&(x, y, z));
                    }
                }
            }
        }
        a += 1;
    }

    slowpoke_hashmap.len() as i64
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut slowpoke_hashmap: HashSet<(i64, i64, i64)> = HashSet::new();
    let mut a = 0;
    for cuboid in parse_output {
        println!("{}", a);
        for x in cuboid.1 .0..=cuboid.1 .1 {
            for y in cuboid.2 .0..=cuboid.2 .1 {
                for z in cuboid.3 .0..=cuboid.3 .1 {
                    if cuboid.0 == true {
                        slowpoke_hashmap.insert((x, y, z));
                    } else {
                        slowpoke_hashmap.remove(&(x, y, z));
                    }
                }
            }
        }
        a += 1;
    }

    slowpoke_hashmap.len() as i64
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse, read_main, read_test, read_test_2};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse(&read_test());
        assert_eq!(part_1(&parse_output), 590784);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test_2());
        assert_eq!(part_2(&parse_output), 2758514936282235);
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
