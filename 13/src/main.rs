#![feature(test)]

use crate::parse_input::{parse, read_main, FoldList, ParseOutput, Point};
use std::cmp::max;
use std::collections::HashSet;

pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is \n{}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let (mut points, _, _, first_fold) = parse_output.clone();
    let mut unique_point_hash_set: HashSet<Point> = HashSet::new();

    let y_folds = if first_fold.0 > 0 {
        vec![first_fold.0]
    } else {
        vec![]
    };

    let x_folds = if first_fold.1 > 0 {
        vec![first_fold.1]
    } else {
        vec![]
    };

    for point in points.drain(0..) {
        unique_point_hash_set.insert(fold_point(point, &y_folds, &x_folds));
    }
    unique_point_hash_set.len() as u32
}

fn fold_point(point: Point, folds_y: &FoldList, folds_x: &FoldList) -> Point {
    let mut y = point.0;
    let mut x = point.1;

    for fold in folds_y.iter() {
        if &y < fold {
            continue;
        }

        y = (2 * fold) - y;
    }

    for fold in folds_x.iter() {
        if &x < fold {
            continue;
        }
        x = (2 * fold) - x;
    }
    (y, x)
}

fn part_2(parse_output: &ParseOutput) -> String {
    let (mut points, y_folds, x_folds, _) = parse_output.clone();
    let mut unique_point_hash_set: HashSet<Point> = HashSet::new();

    for point in points.drain(0..) {
        unique_point_hash_set.insert(fold_point(point, &y_folds, &x_folds));
    }

    let maximum = unique_point_hash_set
        .iter()
        .fold((0, 0), |acc, p| (max(acc.0, p.0), max(acc.1, p.1)));

    let mut solution = String::new();
    for y in 0..=maximum.0 {
        for x in 0..=maximum.1 {
            if unique_point_hash_set.contains(&(y, x)) {
                solution.push_str(" ");
            } else {
                solution.push_str("#");
            }
        }
        solution.push_str("\n");
    }

    solution
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
        assert_eq!(part_1(&parse_output), 17);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(
            part_2(&parse_output),
            "#####\n\
#   #\n\
#   #\n\
#   #\n\
#####\n\
"
        );
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
            assert_eq!(part_1(black_box(&parse_output)), 671);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(
                part_2(black_box(&parse_output)),
                "###   ##  ###  #  #  ##  ###  #  # #   \n#  # #  # #  # #  # #  # #  # # #  #   \n#  # #    #  # #### #  # #  # ##   #   \n###  #    ###  #  # #### ###  # #  #   \n#    #  # #    #  # #  # # #  # #  #   \n#     ##  #    #  # #  # #  # #  # ####\n"
            );
        });
    }
}
