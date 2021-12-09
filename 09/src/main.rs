#![feature(test)]

use crate::parse_input::{parse_main, ParseOutput};
use std::collections::HashMap;

pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse_main();
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut solution: u32 = 0;
    let (max_y, max_x) = (parse_output.len() - 1, parse_output[0].len() - 1);

    for (y, line) in parse_output.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let (top, bottom, left, right, middle) =
                get_surrounding_heights(parse_output, y, x, max_y, max_x);

            if middle < top && middle < bottom && middle < left && middle < right {
                solution += middle as u32 + 1;
            }
        }
    }

    solution
}

fn get_surrounding_heights(
    parse_output: &ParseOutput,
    y: usize,
    x: usize,
    max_y: usize,
    max_x: usize,
) -> (i8, i8, i8, i8, i8) {
    let top: i8 = if y == 0 { 10 } else { parse_output[y - 1][x] };
    let bottom: i8 = if y == max_y {
        10
    } else {
        parse_output[y + 1][x]
    };
    let left: i8 = if x == 0 { 10 } else { parse_output[y][x - 1] };
    let right: i8 = if x == max_x {
        10
    } else {
        parse_output[y][x + 1]
    };
    let middle = parse_output[y][x];

    (top, bottom, left, right, middle)
}

#[derive(Debug)]
struct Basin {
    fields: Vec<(usize, usize)>,
}

type BasinMap = HashMap<u32, Basin>;
type BasinLookupMap = HashMap<(usize, usize), u32>;

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut solution: u32 = 1;
    let (max_y, max_x) = (parse_output.len() - 1, parse_output[0].len() - 1);

    let mut basin_map: BasinMap = HashMap::new();
    let mut basin_lookup_map: BasinLookupMap = HashMap::new();
    let fields_per_height = get_fields_per_height_map(parse_output);
    let mut basin_ids = 0;
    for (current_height, fields) in fields_per_height.iter().enumerate() {
        if current_height == 9 {
            break;
        }

        for field in fields {
            let surrounding_basins = get_surrounding_basins(
                parse_output,
                &mut basin_lookup_map,
                current_height as i8,
                field.0,
                field.1,
                max_y,
                max_x,
            );

            if let Some(basin_id) = basin_lookup_map.get(field) {
                let basin_to_merge_into_id = basin_id.clone();

                for surrounding_basin_id in surrounding_basins {
                    connect_basins(
                        basin_to_merge_into_id,
                        surrounding_basin_id,
                        &mut basin_map,
                        &mut basin_lookup_map,
                    );
                }
            } else {
                match surrounding_basins.len() {
                    0 => {
                        basin_ids += 1;

                        basin_map.insert(
                            basin_ids,
                            Basin {
                                fields: vec![field.clone()],
                            },
                        );
                        basin_lookup_map.insert(field.clone(), basin_ids);
                    }
                    1 => {
                        let other_basin_id = surrounding_basins[0];
                        add_field_to_basin(
                            field,
                            other_basin_id,
                            &mut basin_lookup_map,
                            &mut basin_map,
                        );
                    }
                    _ => {
                        let basin_to_merge_into_id = surrounding_basins[0];
                        add_field_to_basin(
                            field,
                            basin_to_merge_into_id,
                            &mut basin_lookup_map,
                            &mut basin_map,
                        );
                        for surrounding_basin_id in surrounding_basins.iter().skip(1) {
                            connect_basins(
                                surrounding_basin_id.clone(),
                                basin_to_merge_into_id,
                                &mut basin_map,
                                &mut basin_lookup_map,
                            );
                        }
                    }
                }
            }
        }
    }

    let mut basin_sizes: Vec<u32> = Vec::new();
    for (_, basin) in basin_map {
        basin_sizes.push(basin.fields.len() as u32);
    }

    basin_sizes.sort();

    for size in basin_sizes.iter().rev().take(3) {
        solution *= size;
    }

    solution
}

fn add_field_to_basin(
    field: &(usize, usize),
    basin_id: u32,
    basin_lookup_map: &mut BasinLookupMap,
    basin_map: &mut BasinMap,
) {
    basin_map
        .get_mut(&basin_id)
        .unwrap()
        .fields
        .push(field.clone());
    basin_lookup_map.insert(field.clone(), basin_id);
}

fn connect_basins(
    source_basin_id: u32,
    basin_to_merge_into_id: u32,
    basin_map: &mut BasinMap,
    basin_lookup_map: &mut BasinLookupMap,
) {
    let source_basin = basin_map.get_mut(&source_basin_id).unwrap();
    let current_basin_fields = source_basin.fields.clone();

    let basin_to_merge_into = basin_map.get_mut(&basin_to_merge_into_id).unwrap();

    // Overwrite current basin to surrounding basin fields
    for field in current_basin_fields {
        basin_lookup_map
            .insert(field.clone(), basin_to_merge_into_id)
            .unwrap();
        basin_to_merge_into.fields.push(field);
    }

    basin_map.remove(&source_basin_id).unwrap();
}

fn get_surrounding_basins(
    parse_output: &ParseOutput,
    basin_lookup_map: &mut BasinLookupMap,
    current_height: i8,
    y: usize,
    x: usize,
    max_y: usize,
    max_x: usize,
) -> Vec<u32> {
    let edge = ((0, 0), 10);
    let top = if y == 0 {
        edge
    } else {
        ((y - 1, x), parse_output[y - 1][x])
    };
    let bottom = if y == max_y {
        edge
    } else {
        ((y + 1, x), parse_output[y + 1][x])
    };
    let left = if x == 0 {
        edge
    } else {
        ((y, x - 1), parse_output[y][x - 1])
    };
    let right = if x == max_x {
        edge
    } else {
        ((y, x + 1), parse_output[y][x + 1])
    };
    let surrounding_fields = vec![top, bottom, left, right];
    let mut basins = Vec::new();
    let current_basin_id = basin_lookup_map.get(&(y, x)).unwrap_or(&0).clone();

    for surrounding_field in surrounding_fields {
        if surrounding_field.1 <= current_height {
            let other_basin_id_option = basin_lookup_map.get(&surrounding_field.0);
            if let Some(other_basin_id) = other_basin_id_option {
                if &current_basin_id != other_basin_id {
                    basins.push(other_basin_id.clone());
                }
            }
        }
    }

    basins.sort();

    basins.dedup();

    basins
}

fn get_fields_per_height_map(parse_output: &ParseOutput) -> Vec<Vec<(usize, usize)>> {
    let mut fields_per_height: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 10];
    for (y, line) in parse_output.iter().enumerate() {
        for (x, num) in line.iter().enumerate() {
            fields_per_height[num.clone() as usize].push((y, x));
        }
    }
    fields_per_height
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
        assert_eq!(part_1(&parse_output), 15);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_test();
        assert_eq!(part_2(&parse_output), 1134);
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
            assert_eq!(part_1(black_box(&parse_output)), 486);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 1059300);
        });
    }
}
