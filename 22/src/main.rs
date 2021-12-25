#![feature(test)]
#![feature(int_abs_diff)]
#![feature(map_first_last)]

use crate::parse_input::{parse, read_main, ParseOutput};
use algebruh::Instruction;

use crate::algebruh::{Point2D, Point3D};
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

pub mod algebruh;
pub mod parse_input;

type Solution = i64;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut instructions: Vec<Instruction> = parse_output.clone();

    start_reactor(&mut instructions)
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut cubes: Vec<Instruction> = parse_output
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

    start_reactor(&mut cubes)
}

fn start_reactor(instructions: &mut Vec<Instruction>) -> i64 {
    let mut plane_sweep_by_z_axis = get_plane_sweep_by_z_axis(instructions);
    let mut current_solutions: VecDeque<(Point2D, Point2D)> =
        VecDeque::with_capacity(instructions.len());

    let mut active_instructions: BTreeSet<usize> = BTreeSet::new();

    let mut ones_active = 0;
    for ((start_z, mut instructions_start), (end_z, instructions_end)) in
        plane_sweep_by_z_axis.drain(..)
    {
        // Algorithm

        for inst_i in instructions_start.drain(..) {
            active_instructions.insert(inst_i);
        }
        // Get active cube list by operation order
        // Push first of that list in current_solution VecDeque
        // In a loop (starting with the second one in the list)
        // Apply next operation
        // iterative loop over current_solutions length
        // Pop a slice from the front of the current_solutions and see if operation is applicable,
        // if it is not, push back to current_solutions vector
        // If it is, apply operation and push back solutions of operation
        // After loop: Count volumes in current_solution

        if active_instructions.len() == 0 {
            continue;
        }

        let mut start_i = 0;
        let mut skip = 1;
        for (i, int_i) in active_instructions.iter().enumerate() {
            if instructions[*int_i].sets_1 {
                start_i = *int_i;
                break;
            }
            skip += 1;
        }

        if skip > active_instructions.len() {
            continue;
        }

        let first = &instructions[*active_instructions.get(&(start_i)).unwrap()];
        current_solutions.push_back(([first.min[0], first.min[1]], [first.max[0], first.max[1]]));

        for i in active_instructions.iter().skip(skip) {
            let temp = &instructions[*i];
            let operation = ([temp.min[0], temp.min[1]], [temp.max[0], temp.max[1]]);
            for _ in 0..current_solutions.len() {
                let slice = current_solutions.pop_front().unwrap();
                if let Some(intersection_rec) =
                    Instruction::intersection_rectangle_2d(&slice, &operation)
                {
                    Instruction::split_by_containing_rectangle_2d(
                        &slice,
                        &intersection_rec,
                        &mut current_solutions,
                    );
                } else {
                    current_solutions.push_back(slice);
                }
            }
            if temp.sets_1 {
                current_solutions.push_back(operation);
            }
        }

        for s in current_solutions.drain(..) {
            ones_active += Instruction::vol([s.0[0], s.0[1], start_z], [s.1[0], s.1[1], end_z]);
        }

        for inst_i in instructions_end {
            active_instructions.remove(&inst_i);
        }
    }

    ones_active
}

fn get_plane_sweep_by_z_axis(
    instructions: &mut Vec<Instruction>,
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
                starting_instructions.push(*i);
            }
            if instruction.max[2] == current_z {
                ending_instructions.push(*i);
            }
        }

        if starting_instructions.len() > 0 && ending_instructions.len() == 0 {
            let start_of_next_z = prep.peek().unwrap().0;
            instruction_pairs.push((
                (current_z, starting_instructions.clone()),
                (start_of_next_z - 1, vec![]),
            ));
        }

        if starting_instructions.len() == 0 && ending_instructions.len() > 0 {
            let i_l = instruction_pairs.len();
            let end_of_previous_z = &instruction_pairs[i_l - 1].1 .0.clone();
            instruction_pairs.push((
                (end_of_previous_z + 1, vec![]),
                (current_z, ending_instructions.clone()),
            ));
        }

        if starting_instructions.len() > 0 && ending_instructions.len() > 0 {
            instruction_pairs.push((
                (current_z, starting_instructions.clone()),
                (current_z, ending_instructions.clone()),
            ));
        }
    }

    for i in (1..instruction_pairs.len() - 1).rev() {
        let ((_, _), (p1_end_z, p1_end_i)) = &instruction_pairs[i - 1].clone();
        let ((p2_start_z, p2_start_i), (_, _)) = &instruction_pairs[i].clone();
        if p1_end_i.len() > 0 && p2_start_i.len() > 0 && (p2_start_z - p1_end_z) > 1 {
            instruction_pairs.insert(i, ((p1_end_z + 1, vec![]), (p2_start_z - 1, vec![])));
        }
    }

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
    pub fn test_part_1_2() {
        let parse_output = parse(&read_test_2());
        assert_eq!(part_1(&parse_output), 474140);
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
            assert_eq!(part_2(black_box(&parse_output)), 1235164413198198);
        });
    }
}
