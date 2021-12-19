#![feature(test)]

use crate::algebruh::{point_distance, rotate_point, Point, Transformation};
use crate::parse_input::{parse, read_main, ParseOutput, Transformations};
use std::collections::HashSet;

pub mod algebruh;
pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    println!("Part 1~~~~~~");
    let (scanner, possible_rotations) = parse_output.clone();

    let base_scaner = &scanner[0];
    let other_scanner = &scanner[1];

    let transform = get_rotation_candidate(possible_rotations, base_scaner, other_scanner);
    println!("rotation that seems to work {:?}", transform);

    // Get two random points
    // Use the first point and put all distances to all other points in a hash map
    // Use the other point and check all the distances to the other points and see if the distance appears again
    // If distance appears again, take another point from the list until you have 12 or the list is empty
    // If distance does not appear again, use another transformation and repeat algorithm

    0
}

fn get_rotation_candidate(
    possible_rotations: Transformations,
    base_scaner: &Vec<Point>,
    other_scanner: &Vec<Point>,
) -> Transformation {
    for b in 0..base_scaner.len() {
        for o in 0..base_scaner.len() {
            if b == o {
                continue;
            };
            let base_point = &base_scaner[b];
            let other_point = &base_scaner[o];

            for tr in possible_rotations {
                let mut distance_hashset = HashSet::<i32>::new();
                for p_os in other_scanner {
                    distance_hashset.insert(point_distance(*base_point, rotate_point(tr, *p_os)));
                }

                for p_os in other_scanner {
                    if distance_hashset
                        .contains(&point_distance(*other_point, rotate_point(tr, *p_os)))
                    {
                        return tr;
                    }
                }
            }
        }
    }
    unreachable!()
}

fn part_2(_parse_output: &ParseOutput) -> Solution {
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
        assert_eq!(part_1(&parse_output), 1656);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_1(&parse_output), 1656);
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
