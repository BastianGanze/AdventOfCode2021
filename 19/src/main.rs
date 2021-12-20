#![feature(test)]
#![feature(int_abs_diff)]

use crate::algebruh::{
    minus, point_distance, rotate_point, translate_point, Point, Transformation,
};
use crate::parse_input::{parse, read_main, ParseOutput, Transformations};
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};

pub mod algebruh;
pub mod parse_input;

type Solution = u32;

type ScannerRelationships = HashMap<(usize, usize), (Transformation, Point)>;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let (scanner, _possible_rotations) = parse_output;

    let scanner_relationships = get_scanner_relationships(parse_output);

    let mut all_beacons_set: HashSet<Point> = HashSet::new();

    let mut path_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (r, _) in &scanner_relationships {
        path_map.entry(r.0).or_insert_with(|| Vec::new()).push(r.1);
        path_map.entry(r.1).or_insert_with(|| Vec::new()).push(r.0);
    }

    fill_beacon_set(
        0,
        scanner,
        &path_map,
        VecDeque::new(),
        &mut all_beacons_set,
        &scanner_relationships,
        &mut HashSet::<(usize, usize)>::new(),
    );

    all_beacons_set.len() as u32
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let (scanner, _possible_rotations) = parse_output;

    let scanner_relationships = get_scanner_relationships(parse_output);

    let mut path_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (r, _) in &scanner_relationships {
        path_map.entry(r.0).or_insert_with(|| Vec::new()).push(r.1);
        path_map.entry(r.1).or_insert_with(|| Vec::new()).push(r.0);
    }

    let mut scanner_positions: Vec<Vec<Point>> = vec![Vec::new(); scanner.len()];
    scanner_positions[0].push([0, 0, 0]);

    for i in 1..scanner.len() {
        for d in path_map.get(&i).unwrap() {
            scanner_positions[i].push(scanner_relationships.get(&(i, *d)).unwrap().1);
        }
    }

    println!("{:?}", scanner_positions);

    let mut all_scanners_set: HashSet<Point> = HashSet::new();

    fill_beacon_set(
        0,
        &scanner_positions,
        &path_map,
        VecDeque::new(),
        &mut all_scanners_set,
        &scanner_relationships,
        &mut HashSet::<(usize, usize)>::new(),
    );

    let mut max_manhatten = 0;
    for s in &all_scanners_set {
        for p in &all_scanners_set {
            if s == p {
                continue;
            }
            max_manhatten = max(
                max_manhatten,
                p[0].abs_diff(s[0]) + p[1].abs_diff(s[1]) + p[2].abs_diff(s[2]),
            )
        }
    }

    max_manhatten
}

fn fill_beacon_set(
    current_scanner: usize,
    scanners: &Vec<Vec<Point>>,
    path_map: &HashMap<usize, Vec<usize>>,
    transformations: VecDeque<(Transformation, Point)>,
    beacon_set: &mut HashSet<Point>,
    scanner_relationships: &ScannerRelationships,
    paths_traveled: &mut HashSet<(usize, usize)>,
) {
    for mut p in &scanners[current_scanner] {
        let mut t_p = p.clone();
        for (r, t) in &transformations {
            t_p = translate_point(rotate_point(t_p, *r), *t);
        }
        beacon_set.insert(t_p);
    }

    for destination in path_map.get(&current_scanner).unwrap() {
        if paths_traveled.contains(&(current_scanner, *destination)) {
            continue;
        }

        let t = scanner_relationships
            .get(&(current_scanner, *destination))
            .unwrap();
        let mut trans = transformations.clone();
        trans.push_front(t.clone());
        paths_traveled.insert((current_scanner, *destination));
        fill_beacon_set(
            *destination,
            scanners,
            path_map,
            trans,
            beacon_set,
            scanner_relationships,
            paths_traveled,
        )
    }
}

fn get_scanner_relationships(parse_output: &ParseOutput) -> ScannerRelationships {
    let (scanner, possible_rotations) = parse_output;

    let scanner_points_lookup: Vec<HashSet<Point>> = scanner
        .iter()
        .map(|s| HashSet::from_iter(s.clone()))
        .collect();

    let mut scanner_relationships = HashMap::new();

    for b in 0..scanner.len() {
        for o in 0..scanner.len() {
            if b == o {
                continue;
            }
            if let Some(transformation) = get_transformation(
                possible_rotations,
                &scanner[b],
                &scanner[o],
                &scanner_points_lookup[b],
            ) {
                scanner_relationships.insert((b, o), transformation);
            }
        }
    }

    scanner_relationships
}

fn get_transformation(
    possible_rotations: &Transformations,
    base_scanner: &Vec<Point>,
    other_scanner: &Vec<Point>,
    base_scanner_hashset: &HashSet<Point>,
) -> Option<(Transformation, Point)> {
    for b in 0..base_scanner.len() - 12 {
        for o in 0..base_scanner.len() - 12 {
            if b == o {
                continue;
            };
            let base_point = &base_scanner[b];
            let other_point = &base_scanner[o];

            for tr in possible_rotations {
                let mut distance_hashset = HashSet::<i32>::new();
                for p_os in other_scanner {
                    distance_hashset.insert(point_distance(*base_point, rotate_point(*p_os, *tr)));
                }

                for p_os in other_scanner {
                    let p_os_r = rotate_point(*p_os, *tr);
                    let distance = point_distance(*other_point, p_os_r);
                    if distance_hashset.contains(&distance) {
                        let mut points_matching = 0;
                        let translation = minus(p_os_r, *other_point);

                        for p in other_scanner {
                            let t_p = translate_point(rotate_point(*p, *tr), translation);
                            if base_scanner_hashset.contains(&t_p) {
                                points_matching += 1;
                            }
                        }
                        if points_matching >= 12 {
                            return Some((tr.clone(), translation));
                        }
                    }
                }
            }
        }
    }
    None
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
        assert_eq!(part_1(&parse_output), 79);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 3621);
    }
    /*
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

     */
}
