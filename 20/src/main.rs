#![feature(test)]

use crate::parse_input::{parse, read_main, EnhancementMap, Grid, ParseOutput};
use std::collections::HashSet;

pub mod parse_input;

type Solution = usize;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let (mut map, enhancement_map, map_size) = parse_output.clone();

    print_map(&map, &map_size);
    enhancement_step(&mut map, &enhancement_map, &map_size, 10);
    print_map(&map, &map_size);
    enhancement_step(&mut map, &enhancement_map, &map_size, 9);
    print_map(&map, &map_size);

    map.len()
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let (mut map, enhancement_map, map_size) = parse_output.clone();

    for i in (55..105).rev() {
        enhancement_step(&mut map, &enhancement_map, &map_size, i);
    }

    map.len()
}

fn enhancement_step(
    map: &mut Grid,
    enhancement_map: &EnhancementMap,
    map_size: &(i32, i32),
    iteration: i32,
) {
    let mut enhanced_pixels = HashSet::new();
    let mut removed_pixels = HashSet::new();
    for y in -iteration..map_size.0 + iteration {
        for x in -iteration..map_size.1 + iteration {
            if enhance_pixel(map, y, x, enhancement_map) {
                if !map.contains(&(y, x)) {
                    enhanced_pixels.insert((y, x));
                }
            } else {
                if map.contains(&(y, x)) {
                    removed_pixels.insert((y, x));
                }
            }
        }
    }

    map.extend(enhanced_pixels);
    for r in removed_pixels {
        map.remove(&r);
    }

    for y in -iteration..map_size.0 + iteration {
        for x in -iteration..map_size.1 + iteration {
            if y == -iteration
                || y == map_size.0 + iteration - 1
                || x == -iteration
                || x == map_size.1 + iteration - 1
            {
                map.remove(&(y, x));
            }
        }
    }
}

fn print_map(map: &Grid, map_size: &(i32, i32)) {
    for y in -11..map_size.0 + 11 {
        for x in -11..map_size.1 + 11 {
            if map.contains(&(y, x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn enhance_pixel(map: &mut Grid, y: i32, x: i32, enhancement_map: &EnhancementMap) -> bool {
    let enhance_pixel = get_pixel(map, (y - 1, x - 1)) << 8
        | get_pixel(map, (y - 1, x)) << 7
        | get_pixel(map, (y - 1, x + 1)) << 6
        | get_pixel(map, (y, x - 1)) << 5
        | get_pixel(map, (y, x)) << 4
        | get_pixel(map, (y, x + 1)) << 3
        | get_pixel(map, (y + 1, x - 1)) << 2
        | get_pixel(map, (y + 1, x)) << 1
        | get_pixel(map, (y + 1, x + 1));

    enhancement_map[enhance_pixel]
}

fn get_pixel(map: &mut Grid, field: (i32, i32)) -> usize {
    if map.contains(&field) {
        1
    } else {
        0
    }
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
        assert_eq!(part_1(&parse_output), 35);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 3351);
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
