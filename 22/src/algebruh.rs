use std::cmp::{max, min};
use std::collections::VecDeque;

pub type Point2D = [i64; 2];
pub type Point3D = [i64; 3];

#[derive(Debug, Clone)]
pub struct Instruction {
    pub min: Point3D,
    pub max: Point3D,
    pub sets_1: bool,
}

impl Instruction {
    pub fn from(x0: i64, x1: i64, y0: i64, y1: i64, z0: i64, z1: i64, sets_1: bool) -> Instruction {
        let min = [x0, y0, z0];
        let max = [x1, y1, z1];
        Instruction::from_m(min, max, sets_1)
    }

    pub fn from_m(min: Point3D, max: Point3D, sets_1: bool) -> Instruction {
        Instruction { min, max, sets_1 }
    }

    pub fn vol(min: Point3D, max: Point3D) -> i64 {
        (max[0] - min[0] + 1) * (max[1] - min[1] + 1) * (max[2] - min[2] + 1) as i64
    }

    pub fn intersection_rectangle_2d(
        r0: &(Point2D, Point2D),
        r1: &(Point2D, Point2D),
    ) -> Option<(Point2D, Point2D)> {
        if !(r0.0[0] <= r1.1[0] && r0.1[0] >= r1.0[0] && r0.0[1] <= r1.1[1] && r0.1[1] >= r1.0[1]) {
            return None;
        };

        let x0 = max(r0.0[0], r1.0[0]);
        let y0 = max(r0.0[1], r1.0[1]);
        let x1 = min(r0.1[0], r1.1[0]);
        let y1 = min(r0.1[1], r1.1[1]);

        Some(([x0, y0], [x1, y1]))
    }

    pub fn intersection_line_len_2d(&self, c: &Instruction) -> Option<(Point2D, Point2D)> {
        if !(self.min[0] <= c.max[0]
            && self.max[0] >= c.min[0]
            && self.min[1] <= c.max[1]
            && self.max[1] >= c.min[1])
        {
            return None;
        };

        let x0 = max(self.min[0], c.min[0]);
        let y0 = max(self.min[1], c.min[1]);
        let x1 = min(self.max[0], c.max[0]);
        let y1 = min(self.max[1], c.max[1]);

        Some(([x0, y0], [x1, y1]))
    }

    pub fn split_by_containing_rectangle_2d(
        r0: &(Point2D, Point2D),
        r1: &(Point2D, Point2D),
        recs: &mut VecDeque<(Point2D, Point2D)>,
    ) {
        let mut gap_mask: u8 = 0;

        if r1.0[0] > r0.0[0] {
            gap_mask |= 8;
        };
        if r1.1[0] < r0.1[0] {
            gap_mask |= 4;
        };
        if r1.0[1] > r0.0[1] {
            gap_mask |= 2;
        };
        if r1.1[1] < r0.1[1] {
            gap_mask |= 1;
        };

        // bits signify if there is a gap: left right bottom top
        match gap_mask {
            0b1000 => {
                recs.push_back(left_rec(r0, r1));
            }
            0b0100 => {
                recs.push_back(right_rec(r0, r1));
            }
            0b0010 => {
                recs.push_back(bottom_rec(r0, r1));
            }
            0b0001 => {
                recs.push_back(top_rec(r0, r1));
            }
            0b1100 => {
                recs.push_back(left_rec(r0, r1));
                recs.push_back(right_rec(r0, r1));
            }
            0b0011 => {
                recs.push_back(bottom_rec(r0, r1));
                recs.push_back(top_rec(r0, r1));
            }
            0b1001 => {
                recs.push_back(left_rec(r0, r1));
                recs.push_back(top_right_rec(r0, r1));
            }
            0b1010 => {
                recs.push_back(left_rec(r0, r1));
                recs.push_back(bottom_right_rec(r0, r1));
            }
            0b0101 => {
                recs.push_back(right_rec(r0, r1));
                recs.push_back(top_left_rec(r0, r1));
            }
            0b0110 => {
                recs.push_back(right_rec(r0, r1));
                recs.push_back(bottom_left_rec(r0, r1));
            }
            0b1101 => {
                recs.push_back(top_rec(r0, r1));

                recs.push_back(([r0.0[0], r0.0[1]], [r1.0[0] - 1, r1.1[1]]));
                recs.push_back(([r1.1[0] + 1, r0.0[1]], [r0.1[0], r1.1[1]]));
            }
            0b1110 => {
                recs.push_back(bottom_rec(r0, r1));

                recs.push_back(([r0.0[0], r1.0[1]], [r1.0[0] - 1, r0.1[1]]));
                recs.push_back(([r1.1[0] + 1, r1.0[1]], [r0.1[0], r0.1[1]]));
            }
            0b0111 => {
                recs.push_back(right_rec(r0, r1));

                recs.push_back(bottom_left_rec(r0, r1));
                recs.push_back(top_left_rec(r0, r1));
            }
            0b1011 => {
                recs.push_back(left_rec(r0, r1));

                recs.push_back(bottom_right_rec(r0, r1));
                recs.push_back(top_right_rec(r0, r1));
            }
            0b1111 => {
                recs.push_back(left_rec(r0, r1));
                recs.push_back(right_rec(r0, r1));
                recs.push_back(([r1.0[0], r0.0[1]], [r1.1[0], r1.0[1] - 1]));
                recs.push_back(([r1.0[0], r1.1[1] + 1], [r1.1[0], r0.1[1]]));
            }
            0 => {}
            _ => unreachable!(),
        }
    }
}

fn left_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r0.0[0], r0.0[1]], [r1.0[0] - 1, r0.1[1]])
}

fn right_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r1.1[0] + 1, r0.0[1]], [r0.1[0], r0.1[1]])
}

fn bottom_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r0.0[0], r0.0[1]], [r0.1[0], r1.0[1] - 1])
}

fn top_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r0.0[0], r1.1[1] + 1], [r0.1[0], r0.1[1]])
}

fn top_left_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r0.0[0], r1.1[1] + 1], [r1.1[0], r0.1[1]])
}

fn bottom_right_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r1.0[0], r0.0[1]], [r0.1[0], r1.0[1] - 1])
}

fn top_right_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r1.0[0], r1.1[1] + 1], [r0.1[0], r0.1[1]])
}

fn bottom_left_rec(r0: &(Point2D, Point2D), r1: &(Point2D, Point2D)) -> (Point2D, Point2D) {
    ([r0.0[0], r0.0[1]], [r1.1[0], r1.0[1] - 1])
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    fn _vol(vec: &Vec<(Point2D, Point2D)>) -> i64 {
        vec.iter().fold(0, |a, c| {
            a + Instruction::vol([c.0[0], c.0[1], 0], [c.1[0], c.1[1], 0])
        })
    }

    #[test]
    pub fn split_by_2d() {
        let c1 = ([0, 0], [5, 5]);
        let mut recs = VecDeque::new();
        Instruction::split_by_containing_rectangle_2d(&c1, &c1, &mut recs);
        assert_eq!(recs, vec![], "Case 1");
        let s = vec![([0, 0], [5, 5])];
        assert_eq!(
            Instruction::vol([c1.0[0], c1.0[1], 0], [c1.1[0], c1.1[1], 0]),
            _vol(&s),
            "Case 1 Correct volume"
        );

        let left = ([0, 0], [3, 5]);
        let s = vec![([4, 0], [5, 5])];
        assert_correct_intersection(&c1, &left, s, "Case 2");

        let right = ([3, 0], [5, 5]);
        let s = vec![([0, 0], [2, 5])];
        assert_correct_intersection(&c1, &right, s, "Case 2.2");

        let bottom = ([0, 0], [5, 3]);
        let s = vec![([0, 4], [5, 5])];
        assert_correct_intersection(&c1, &bottom, s, "Case 2.2");

        let top = ([0, 3], [5, 5]);
        let s = vec![([0, 0], [5, 2])];
        assert_correct_intersection(&c1, &top, s, "Case 2.3");

        let middle = ([2, 0], [4, 5]);
        let s = vec![([0, 0], [1, 5]), ([5, 0], [5, 5])];
        assert_correct_intersection(&c1, &middle, s, "Case 3");

        let center = ([0, 1], [5, 3]);
        let s = vec![([0, 0], [5, 0]), ([0, 4], [5, 5])];
        assert_correct_intersection(&c1, &center, s, "Case 3.2");

        let b_left = ([0, 0], [3, 3]);
        let s = vec![([4, 0], [5, 5]), ([0, 4], [3, 5])];
        assert_correct_intersection(&c1, &b_left, s, "Case 4");

        let t_right = ([3, 3], [5, 5]);
        let s = vec![([0, 0], [2, 5]), ([3, 0], [5, 2])];
        assert_correct_intersection(&c1, &t_right, s, "Case 4.1");

        let b_left = ([0, 3], [3, 5]);
        let s = vec![([4, 0], [5, 5]), ([0, 0], [3, 2])];
        assert_correct_intersection(&c1, &b_left, s, "Case 4.2");

        let b_right = ([3, 0], [5, 3]);
        let s = vec![([0, 0], [2, 5]), ([3, 4], [5, 5])];
        assert_correct_intersection(&c1, &b_right, s, "Case 4.3");

        let r_mid = ([3, 2], [5, 3]);
        let s = vec![([0, 0], [2, 5]), ([3, 0], [5, 1]), ([3, 4], [5, 5])];
        assert_correct_intersection(&c1, &r_mid, s, "Case 5");

        let l_mid = ([0, 2], [3, 3]);
        let s = vec![([4, 0], [5, 5]), ([0, 0], [3, 1]), ([0, 4], [3, 5])];
        assert_correct_intersection(&c1, &l_mid, s, "Case 5.1");

        let b_center = ([2, 0], [4, 3]);
        let s = vec![([0, 4], [5, 5]), ([0, 0], [1, 3]), ([5, 0], [5, 3])];
        assert_correct_intersection(&c1, &b_center, s, "Case 5.2");

        let t_center = ([2, 3], [4, 5]);
        let s = vec![([0, 0], [5, 2]), ([0, 3], [1, 5]), ([5, 3], [5, 5])];
        assert_correct_intersection(&c1, &t_center, s, "Case 5.3");

        let c2 = ([2, 2], [3, 3]);
        let s = vec![
            ([0, 0], [1, 5]),
            ([4, 0], [5, 5]),
            ([2, 0], [3, 1]),
            ([2, 4], [3, 5]),
        ];
        assert_correct_intersection(&c1, &c2, s, "Case 6");
    }

    fn assert_correct_intersection(
        c1: &(Point2D, Point2D),
        c2: &(Point2D, Point2D),
        s: Vec<(Point2D, Point2D)>,
        name: &str,
    ) {
        let mut recs = VecDeque::new();
        Instruction::split_by_containing_rectangle_2d(c1, c2, &mut recs);
        assert_eq!(recs, s, "{}", name);
        assert_eq!(
            Instruction::vol([c1.0[0], c1.0[1], 0], [c1.1[0], c1.1[1], 0]),
            _vol(&s) + Instruction::vol([c2.0[0], c2.0[1], 0], [c2.1[0], c2.1[1], 0]),
            "{} volume",
            name
        );
    }
}
// -> Create 3D Cube that is sortable
// -> Create list of Cubes that is sorted by z-axis
// -> Watch result
