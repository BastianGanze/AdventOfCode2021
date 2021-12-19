#![feature(test)]
use crate::parse_input::{parse, read_main, ParseOutput};
use std::cmp::{max, min};

pub mod parse_input;

type Solution = i32;

fn main() {
    let parse_output = parse(&read_main());
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let ((y1, _), (y2, _)) = parse_output.clone();

    max_y(max_y_v(y1, y2))
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let ((y1, x1), (y2, x2)) = parse_output.clone();
    let y_range = (y1, y2);
    let x_range = (x1, x2);

    let max_y_velocity = max_y_v(y1, y2);
    let min_y_velocity = min_y_v(y1, y2);
    let min_x_velocity = min_x_v(x1, x2);
    let max_x_velocity = max_x_v(x1, x2);
    let mut current_x_v;
    let mut current_y_v = min_y_velocity;
    let mut valid_combinations = 0;

    for _ in 0..=max_y_velocity - min_y_velocity {
        current_x_v = min_x_velocity;
        for _ in 0..=max_x_velocity - min_x_velocity {
            let (t0, t1) = get_hit_t_y(current_y_v, y_range);
            let x_at0 = x_at_t(t0, current_x_v);
            let x_at1 = x_at_t(t1, current_x_v);
            if in_range(x_at0, x_range.0, x_range.1) || in_range(x_at1, x_range.0, x_range.1) {
                valid_combinations += 1;
            }

            current_x_v += 1;
        }
        current_y_v += 1;
    }

    valid_combinations
}

fn get_hit_t_y(v: i32, range: (i32, i32)) -> (i32, i32) {
    let y0 = range.0;
    let y1 = range.1;
    let t0 = get_t_for_when_target_hit(v, y0);
    let t1 = get_t_for_when_target_hit(v, y1);

    let mut t0_in_range = -1;
    let mut t1_in_range = -1;

    let lower_bound_t0 = t0.floor() as i32;
    let upper_bound_t0 = t0.ceil() as i32;
    let lower_bound_t1 = t1.floor() as i32;
    let upper_bound_t1 = t1.ceil() as i32;

    if in_range(y_at_t(lower_bound_t0, v), y0, y1) {
        t0_in_range = lower_bound_t0;
    }
    if in_range(y_at_t(upper_bound_t0, v), y0, y1) {
        t0_in_range = upper_bound_t0;
    }
    if in_range(y_at_t(lower_bound_t1, v), y0, y1) {
        t1_in_range = lower_bound_t1;
    }
    if in_range(y_at_t(upper_bound_t1, v), y0, y1) {
        t1_in_range = upper_bound_t1;
    }

    (t0_in_range, t1_in_range)
}

fn in_range(n: i32, s: i32, e: i32) -> bool {
    let min = min(s, e);
    let max = max(s, e);
    n >= min && n <= max
}

fn get_t_for_when_target_hit(v: i32, target: i32) -> f64 {
    let a = -0.5_f64;
    let b = v as f64 + 0.5_f64;
    let c = -target as f64;
    let sqrt_examination: f64 = (b * b) - (4_f64 * a * c);
    let sqrt = sqrt_examination.sqrt();
    if sqrt_examination >= 0_f64 {
        let t1 = (-b + sqrt) / (2_f64 * a);
        let t2 = (-b - sqrt) / (2_f64 * a);

        if t1 > 0_f64 {
            return t1;
        }
        return t2;
    }

    -1_f64
}

fn max_y_v(y1: i32, y2: i32) -> i32 {
    let max = min(y1, y2) + 1;
    -max
}

fn min_y_v(y1: i32, y2: i32) -> i32 {
    let max = min(y1, y2);
    max
}

fn min_x_v(x1: i32, x2: i32) -> i32 {
    let max = max(x1, x2);
    let x = max as f64;
    let sqrt = ((8_f64 * x) + 1_f64).sqrt();
    (0.5_f64 * (sqrt - 1_f64)) as i32 - 1
}

fn max_x_v(x1: i32, x2: i32) -> i32 {
    max(x1, x2)
}

fn max_y(v: i32) -> i32 {
    // t_max is v + 0.5
    let t_max = v + 1;
    y_at_t(t_max, v)
}

fn y_at_t(t: i32, v: i32) -> i32 {
    (v * t) - ((t * t) / 2) + (t / 2)
}

fn x_at_t(t: i32, v: i32) -> i32 {
    let mut clamped_t = t;
    if t > v {
        clamped_t = v;
    }
    (v * clamped_t) - ((clamped_t * clamped_t) / 2) + (clamped_t / 2)
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
        assert_eq!(part_1(&parse_output), 45);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse(&read_test());
        assert_eq!(part_2(&parse_output), 112);
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
            assert_eq!(part_1(black_box(&parse_output)), 17766);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse(&read_main());
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 1733);
        });
    }
}
