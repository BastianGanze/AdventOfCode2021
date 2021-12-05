#![feature(test)]

use std::cmp::{max, min};

type ParseOutput = Vec<((usize, usize), (usize, usize))>;
type Solution = u16;

fn main() {
    let parse_output = parse_input_file("src/05.txt");
    println!(
        "Solution to part 1 is {}",
        part_1::<1000>(parse_output.clone())
    );
    println!("Solution to part 2 is {}", part_2::<1000>(parse_output));
}

fn parse_input_file(input_file: &str) -> ParseOutput {
    let input_file_string = std::fs::read_to_string(input_file).unwrap();
    let line_descriptions: Vec<&str> = input_file_string.split('\n').collect();

    line_descriptions
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let points = line.split_once(" -> ").unwrap();

            let (x1, y1) = points.0.split_once(",").unwrap();
            let (x2, y2) = points.1.split_once(",").unwrap();

            (
                (x1.parse().unwrap(), y1.parse().unwrap()),
                (x2.parse().unwrap(), y2.parse().unwrap()),
            )
        })
        .collect()
}

fn part_1<const N: usize>(parse_output: ParseOutput) -> Solution {
    let mut grid = vec![[0 as u16; N]; N];

    let straight_lines = filter_straight_lines(parse_output);
    let mut dangerous_point_count = 0 as u16;
    for ((x1, y1), (x2, y2)) in straight_lines {
        let x = x1;
        let y = y1;
        if x1 == x2 {
            let range_y = if y2 > y1 { y1..=y2 } else { y2..=y1 };
            for y in range_y {
                grid[x][y] += 1;
                if grid[x][y] == 2 {
                    dangerous_point_count += 1;
                }
            }
        } else {
            let range_x = if x2 > x1 { x1..=x2 } else { x2..=x1 };
            for x in range_x {
                grid[x][y] += 1;
                if grid[x][y] == 2 {
                    dangerous_point_count += 1;
                }
            }
        }
    }
    dangerous_point_count
}

fn part_2<const N: usize>(parse_output: ParseOutput) -> Solution {
    let mut grid = vec![[0 as u16; N]; N];

    let mut dangerous_point_count = 0 as u16;
    for ((x1, y1), (x2, y2)) in parse_output {
        let x = x1;
        let y = y1;
        let range_y = if y2 > y1 { y1..=y2 } else { y2..=y1 };
        let range_x = if x2 > x1 { x1..=x2 } else { x2..=x1 };
        if x1 == x2 {
            for y in range_y {
                grid[x][y] += 1;
                if grid[x][y] == 2 {
                    dangerous_point_count += 1;
                }
            }
        } else if y1 == y2 {
            for x in range_x {
                grid[x][y] += 1;
                if grid[x][y] == 2 {
                    dangerous_point_count += 1;
                }
            }
        } else {
            let x_min = min(x1, x2);
            let y_min = min(y1, y2);
            let y_max = max(y1, y2);
            let diagonal_distance = y_max - y_min;
            let slope_is_positive = !((x2 < x1) ^ (y2 < y1));

            if slope_is_positive {
                for i in 0..diagonal_distance + 1 {
                    grid[x_min + i][y_min + i] += 1;
                    if grid[x_min + i][y_min + i] == 2 {
                        dangerous_point_count += 1;
                    }
                }
            } else {
                for i in 0..diagonal_distance + 1 {
                    grid[x_min + i][y_max - i] += 1;
                    if grid[x_min + i][y_max - i] == 2 {
                        dangerous_point_count += 1;
                    }
                }
            }
        }
    }

    dangerous_point_count
}

fn filter_straight_lines(parse_output: ParseOutput) -> ParseOutput {
    parse_output
        .into_iter()
        .filter(|((x1, y1), (x2, y2))| x2 ^ x1 == 0 || y2 ^ y1 == 0)
        .collect()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    pub fn bench_pars_should_give_expected_output() {
        let parse_output = parse_input_file("src/test.txt");
        assert_eq!(
            parse_output,
            vec!(
                ((0, 9), (5, 9)),
                ((8, 0), (0, 8)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((6, 4), (2, 0)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
                ((0, 0), (8, 8)),
                ((5, 5), (8, 2)),
            )
        );
    }

    #[test]
    pub fn filtering_non_diagonal_lines_should_work() {
        let parse_output = parse_input_file("src/test.txt");
        assert_eq!(
            filter_straight_lines(parse_output),
            vec!(
                ((0, 9), (5, 9)),
                ((9, 4), (3, 4)),
                ((2, 2), (2, 1)),
                ((7, 0), (7, 4)),
                ((0, 9), (2, 9)),
                ((3, 4), (1, 4)),
            )
        );
    }

    #[test]
    fn test_part_1() {
        let parse_output = parse_input_file("src/test.txt");
        assert_eq!(part_1::<10>(parse_output), 5);
    }

    #[test]
    fn test_part_2() {
        let parse_output = parse_input_file("src/test.txt");
        assert_eq!(part_2::<10>(parse_output), 12);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_input_file("src/05.txt");
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse_input_file("src/05.txt");
        b.iter(|| {
            part_1::<1000>(parse_output.clone());
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_input_file("src/05.txt");
        b.iter(|| {
            part_2::<1000>(parse_output.clone());
        });
    }
}
