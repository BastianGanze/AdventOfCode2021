#![feature(test)]
#![feature(int_abs_diff)]

use crate::parse_input::{parse, read_main, ParseOutput};
use algebruh::Cube;
use std::cmp::Ordering;

use crate::algebruh::Point3D;
use std::collections::{HashMap, HashSet, VecDeque};

pub mod algebruh;
pub mod parse_input;

type Solution = i64;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn get_correct_map(cubes: &Vec<Cube>) -> HashSet<(i64, i64, i64)> {
    let mut slowpoke_hashmap: HashSet<(i64, i64, i64)> = HashSet::new();
    for cuboid in cubes {
        set_1s(&mut slowpoke_hashmap, cuboid);
    }

    slowpoke_hashmap
}

fn set_1s(shashmap: &mut HashSet<(i64, i64, i64)>, c: &Cube) {
    for x in 0..=c.max[0] - c.min[0] {
        for y in 0..=c.max[1] - c.min[1] {
            for z in 0..=c.max[2] - c.min[2] {
                if c.sets_1 == true {
                    shashmap.insert((c.min[0] + x, c.min[1] + y, c.min[2] + z));
                } else {
                    shashmap.remove(&(c.min[0] + x, c.min[1] + y, c.min[2] + z));
                }
            }
        }
    }
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut cubes: Vec<Cube> = parse_output.clone();

    start_reactor(&mut cubes)
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut cubes: Vec<Cube> = parse_output
        .clone()
        .into_iter()
        .filter(|c| {
            c.min[0].abs() <= 50
                && c.max[0].abs() <= 50
                && c.min[1].abs() <= 50
                && c.max[1].abs() <= 50
                && c.min[2].abs() <= 50
                && c.max[2].abs() <= 50
        })
        .collect();

    println!("{}", get_correct_map(&cubes).len());
    println!(
        "{:?}",
        cubes
            .iter()
            .map(|c| { format!("{:?} {:?}", c.min, c.max) })
            .collect::<Vec<String>>()
    );
    start_reactor(&mut cubes)
}

fn start_reactor(instructions: &mut Vec<Cube>) -> i64 {
    let correct_map = get_correct_map(instructions);
    let mut incorrect_map = HashSet::new();
    let plane_sweep_by_z_axis = get_plane_sweep_by_z_axis(instructions);
    println!("{:?}", plane_sweep_by_z_axis);
    let mut active_instructions: HashSet<usize> = HashSet::new();

    let mut ones_active = 0;
    for ((start_z, instructions_start), (end_z, instructions_end)) in plane_sweep_by_z_axis {
        // Algorithm

        println!("~~~~~~~~~slice: {} {}~~~~~~~~~~~~", start_z, end_z);
        for inst_i in instructions_start {
            active_instructions.insert(inst_i.clone());
        }

        println!(
            "§§§§§§§§§§§§§§§§§§§§§ active instrfuctions {:?}",
            active_instructions
        );

        // Get active cube list by operation order
        // Push first of that list in current_solution VecDeque
        // In a loop (starting with the second one in the list)
        // Apply next operation
        // iterative loop over current_solutions length
        // Pop a slice from the front of the current_solutions and see if operation is applicable,
        // if it is not, push back to current_solutions vector
        // If it is, apply operation and push back solutions of operation
        // After loop: Count volumes in current_solution
        let mut slices_of_instructions = get_slice_of_active_ordered_instructions(
            instructions,
            &mut active_instructions,
            start_z,
            end_z,
        );
        println!(
            "slices_of_instructions {:?}",
            slices_of_instructions
                .iter()
                .map(|a| (a.min, a.max))
                .collect::<Vec<(Point3D, Point3D)>>()
        );

        if slices_of_instructions.len() == 0 {
            continue;
        }

        let mut current_solutions: VecDeque<Cube> = VecDeque::new();
        current_solutions.push_back(slices_of_instructions.pop_front().unwrap());

        for operation in slices_of_instructions.drain(..) {
            println!(
                "################# next operation {} {} ####################",
                operation.sets_1, operation.order
            );

            for _ in 0..current_solutions.len() {
                let slice = current_solutions.pop_front().unwrap();
                current_solutions.extend(calculate_intersections_2d(slice, &operation));
            }
            if operation.sets_1 {
                current_solutions.push_back(operation);
            }
        }

        println!("cs {:?}", current_solutions);

        let mut check_set: HashSet<(i64, i64, i64)> = HashSet::new();
        let mut volume_for_this_solution: i64 = 0;
        for s in current_solutions.drain(..) {
            set_1s(&mut incorrect_map, &s);
            set_1s(&mut check_set, &s);
            ones_active += s.one_amount;
            volume_for_this_solution += s.one_amount;
        }
        //ones_active += check_set.len() as i64;
        println!(
            "1 by hashset {} <-> 1 by volume {}",
            check_set.len(),
            volume_for_this_solution
        );
        assert_eq!(check_set.len() as i64, volume_for_this_solution);
        println!("ones_active {}", ones_active);

        /*
        println!(
            "active cubes: {:?} {:?}",
            active_cubes,
            active_ordered_cube_slices
                .iter()
                .map(|a| { format!("{:?} {:?}", a.min, a.max) })
                .collect::<Vec<String>>()
        );
        if active_ordered_cube_slices.is_empty() {
            last_z = current_z.clone();
            continue;
        }


        println!("{:?}", current_solutions);
        let mut check_set: HashSet<(i64, i64, i64)> = HashSet::new();
        let mut volume_for_this_solution: i64 = 0;
        for s in current_solutions.drain(..) {
            set_1s(&mut check_set, &s);
            set_1s(&mut incorrect_map, &s);
            ones_active += s.one_amount;
            volume_for_this_solution += s.one_amount;
        }
        //ones_active += check_set.len() as i64;
        println!(
            "1 by hashset {} <-> 1 by volume {}",
            check_set.len(),
            volume_for_this_solution
        );
        assert_eq!(check_set.len() as i64, volume_for_this_solution);
        println!("ones_active {}", ones_active);

        // End of algorithm

         */
        for inst_i in instructions_end {
            active_instructions.remove(&inst_i);
        }
    }

    println!(
        "correct map - incorrect map {} {:?}",
        (&correct_map - &incorrect_map).len(),
        (&correct_map - &incorrect_map)
            .into_iter()
            .enumerate()
            .filter(|(a, _)| { a < &10_usize })
            .map(|(_, b)| { b })
            .collect::<Vec<(i64, i64, i64)>>()
    );

    println!(
        "{} - {} = {}",
        correct_map.len(),
        ones_active,
        ones_active - correct_map.len() as i64
    );

    ones_active
}

fn calculate_intersections_2d(cube: Cube, operation: &Cube) -> Vec<Cube> {
    println!(
        "Calc intersection between {:?} {:?} and {:?} {:?}",
        cube.min, cube.max, operation.min, operation.max
    );
    if let Some((min, max)) = cube.intersection_rectangle_2d(operation) {
        println!("i {:?} {:?}", min, max);
        let int_rec_cube_with_operation = Cube::from_m(min, max, operation.sets_1, -1);

        let split_recs: Vec<Cube> = cube
            .split_by_containing_rectangle_2d(&int_rec_cube_with_operation)
            .into_iter()
            .map(|(min, max)| {
                println!("r {:?} {:?}", min, max);
                Cube::from_m(min, max, true, -1)
            })
            .collect();

        println!("returning {}", split_recs.len());

        return split_recs;
    }

    println!("No intersections, returning cube");
    vec![cube]
}

fn get_slice_of_active_ordered_instructions(
    cubes: &mut Vec<Cube>,
    active_cubes: &mut HashSet<usize>,
    start_z: i64,
    end_z: i64,
) -> VecDeque<Cube> {
    let mut active_cube_slices = Vec::new();
    for c in active_cubes.iter() {
        let cube = &cubes[*c];
        active_cube_slices.push(Cube::from(
            cube.min[0],
            cube.max[0],
            cube.min[1],
            cube.max[1],
            start_z,
            end_z,
            cube.sets_1,
            cube.order,
        ));
    }
    active_cube_slices.sort_by(|c1, c2| c1.order.cmp(&c2.order));
    VecDeque::from_iter(active_cube_slices.into_iter())
}

fn get_plane_sweep_by_z_axis(
    instructions: &mut Vec<Cube>,
) -> Vec<((i64, Vec<usize>), (i64, Vec<usize>))> {
    let mut prep: HashMap<i64, Vec<usize>> = HashMap::new();

    for i in 0..instructions.len() {
        let z_min = instructions[i].min[2];
        let z_max = instructions[i].max[2];

        prep.entry(z_min).or_insert_with(|| Vec::new()).push(i);
        prep.entry(z_max).or_insert_with(|| Vec::new()).push(i);
    }

    let mut prep_plane_sweep: Vec<(i64, Vec<usize>)> =
        prep.into_iter().map(|e| (e.0, e.1)).collect();
    prep_plane_sweep.sort_by(|z1, z2| z1.0.cmp(&z2.0));

    println!("prep sweep {:?}", prep_plane_sweep);

    let mut instruction_pairs: Vec<((i64, Vec<usize>), (i64, Vec<usize>))> = Vec::new();
    let mut prep = prep_plane_sweep.into_iter().peekable();
    loop {
        if let None = prep.peek() {
            break;
        }
        let (current_z, current_instructions) = prep.next().unwrap();

        let mut starting_instructions = Vec::new();
        let mut ending_instructions = Vec::new();

        for i in &current_instructions {
            let instruction = &instructions[*i];
            if instruction.min[2] == current_z {
                starting_instructions.push(i);
            }
            if instruction.max[2] == current_z {
                ending_instructions.push(i);
            }
        }

        if starting_instructions.len() > 0 && ending_instructions.len() == 0 {
            println!("Case 1");
            let start_of_next_z = prep.peek().unwrap().0;
            instruction_pairs.push((
                (current_z, current_instructions.clone()),
                (start_of_next_z - 1, vec![]),
            ));
        }

        if starting_instructions.len() == 0 && ending_instructions.len() > 0 {
            println!("Case 2");
            let i_l = instruction_pairs.len();
            let end_of_previous_z = &instruction_pairs[i_l - 1].1 .0.clone();
            instruction_pairs.push((
                (end_of_previous_z + 1, vec![]),
                (current_z, current_instructions.clone()),
            ));
        }

        if starting_instructions.len() > 0 && ending_instructions.len() > 0 {
            println!("Case 3 {} {}", current_z, current_z);
            instruction_pairs.push((
                (current_z, starting_instructions.clone()),
                (current_z, ending_instructions.clone()),
            ));
        }
    }

    for i in (1..instruction_pairs.len() - 1).rev() {
        let ((p1_start_z, p1_start_i), (p1_end_z, p1_end_i)) = &instruction_pairs[i - 1].clone();
        let ((p2_start_z, p2_start_i), (p2_end_z, p2_end_i)) = &instruction_pairs[i].clone();
        if p1_end_i.len() > 0 && p2_start_i.len() > 0 {
            println!("{} {}", p1_end_z, p2_start_z);
            instruction_pairs.insert(i, ((p1_end_z + 1, vec![]), (p2_start_z - 1, vec![])));
        }
    }

    println!("{:?}", instruction_pairs);

    instruction_pairs
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
            assert_eq!(part_1(black_box(&parse_output)), 642125);
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
