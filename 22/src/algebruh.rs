use std::cmp::{max, min};

pub type Point3D = [i64; 3];

#[derive(Debug, Clone)]
pub struct Cube {
    pub min: Point3D,
    pub max: Point3D,
    pub sets_1: bool,
    pub one_amount: i64,
    pub order: i64,
}

impl Cube {
    pub fn from(
        x0: i64,
        x1: i64,
        y0: i64,
        y1: i64,
        z0: i64,
        z1: i64,
        sets_1: bool,
        order: i64,
    ) -> Cube {
        let min = [x0, y0, z0];
        let max = [x1, y1, z1];
        Cube::from_m(min, max, sets_1, order)
    }

    pub fn from_m(min: Point3D, max: Point3D, sets_1: bool, order: i64) -> Cube {
        Cube {
            min,
            max,
            sets_1,
            one_amount: Cube::vol(min, max),
            order,
        }
    }

    pub fn vol(min: Point3D, max: Point3D) -> i64 {
        (max[0] - min[0] + 1) * (max[1] - min[1] + 1) * (max[2] - min[2] + 1) as i64
    }

    pub fn intersection_rectangle_2d(&self, c: &Cube) -> Option<(Point3D, Point3D)> {
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

        Some(([x0, y0, self.min[2]], [x1, y1, c.max[2]]))
    }

    pub fn intersection_line_len_2d(&self, c: &Cube) -> Option<(Point3D, Point3D)> {
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

        Some(([x0, y0, self.min[2]], [x1, y1, c.max[2]]))
    }

    pub fn split_by_containing_rectangle_2d(&self, c: &Cube) -> Vec<(Point3D, Point3D)> {
        let mut recs = Vec::new();
        let mut gap_mask: u8 = 0;

        if c.min[0] > self.min[0] {
            gap_mask |= 8;
        };
        if c.max[0] < self.max[0] {
            gap_mask |= 4;
        };
        if c.min[1] > self.min[1] {
            gap_mask |= 2;
        };
        if c.max[1] < self.max[1] {
            gap_mask |= 1;
        };

        // bits signify if there is a gap: left right bottom top
        match gap_mask {
            0b1000 => {
                recs.push(self.left_rec(c));
            }
            0b0100 => {
                recs.push(self.right_rec(c));
            }
            0b0010 => {
                recs.push(self.bottom_rec(c));
            }
            0b0001 => {
                recs.push(self.top_rec(c));
            }
            0b1100 => {
                recs.push(self.left_rec(c));
                recs.push(self.right_rec(c));
            }
            0b0011 => {
                recs.push(self.bottom_rec(c));
                recs.push(self.top_rec(c));
            }
            0b1001 => {
                recs.push(self.left_rec(c));
                recs.push(self.top_right_rec(c));
            }
            0b1010 => {
                recs.push(self.left_rec(c));
                recs.push(self.bottom_right_rec(c));
            }
            0b0101 => {
                recs.push(self.right_rec(c));
                recs.push(self.top_left_rec(c));
            }
            0b0110 => {
                recs.push(self.right_rec(c));
                recs.push(self.bottom_left_rec(c));
            }
            0b1101 => {
                recs.push(self.top_rec(c));

                recs.push((
                    [self.min[0], self.min[1], self.min[2]],
                    [c.min[0] - 1, c.max[1], self.max[2]],
                ));
                recs.push((
                    [c.max[0] + 1, self.min[1], self.min[2]],
                    [self.max[0], c.max[1], self.max[2]],
                ));
            }
            0b1110 => {
                recs.push(self.bottom_rec(c));

                recs.push((
                    [self.min[0], c.min[1], self.min[2]],
                    [c.min[0] - 1, self.max[1], self.max[2]],
                ));
                recs.push((
                    [c.max[0] + 1, c.min[1], self.min[2]],
                    [self.max[0], self.max[1], self.max[2]],
                ));
            }
            0b0111 => {
                recs.push(self.right_rec(c));

                recs.push(self.bottom_left_rec(c));
                recs.push(self.top_left_rec(c));
            }
            0b1011 => {
                recs.push(self.left_rec(c));

                recs.push(self.bottom_right_rec(c));
                recs.push(self.top_right_rec(c));
            }
            0b1111 => {
                recs.push(self.left_rec(c));
                recs.push(self.right_rec(c));
                recs.push((
                    [c.min[0], self.min[1], self.min[2]],
                    [c.max[0], c.min[1] - 1, self.max[2]],
                ));
                recs.push((
                    [c.min[0], c.max[1] + 1, self.min[2]],
                    [c.max[0], self.max[1], self.max[2]],
                ));
            }
            0 => {}
            _ => unreachable!(),
        }

        recs
    }

    fn left_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], self.min[1], self.min[2]],
            [c.min[0] - 1, self.max[1], self.max[2]],
        )
    }

    fn right_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [c.max[0] + 1, self.min[1], self.min[2]],
            [self.max[0], self.max[1], self.max[2]],
        )
    }

    fn bottom_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], self.min[1], self.min[2]],
            [self.max[0], c.min[1] - 1, self.max[2]],
        )
    }

    fn top_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], c.max[1] + 1, self.min[2]],
            [self.max[0], self.max[1], self.max[2]],
        )
    }

    fn top_left_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], c.max[1] + 1, self.min[2]],
            [c.max[0], self.max[1], self.max[2]],
        )
    }

    fn bottom_right_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [c.min[0], self.min[1], self.min[2]],
            [self.max[0], c.min[1] - 1, self.max[2]],
        )
    }

    fn top_right_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [c.min[0], c.max[1] + 1, self.min[2]],
            [self.max[0], self.max[1], self.max[2]],
        )
    }

    fn bottom_left_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], self.min[1], self.min[2]],
            [c.max[0], c.min[1] - 1, self.max[2]],
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    fn _vol(vec: &Vec<(Point3D, Point3D)>) -> i64 {
        vec.iter().fold(0, |a, c| a + Cube::vol(c.0, c.1))
    }

    #[test]
    pub fn split_by_2d() {
        let c1 = Cube::from(0, 5, 0, 5, 0, 20, true, 0);
        assert_eq!(c1.split_by_containing_rectangle_2d(&c1), vec![], "Case 1");
        let s = vec![([0, 0, 0], [5, 5, 20])];
        assert_eq!(c1.one_amount, _vol(&s), "Case 1 Correct volume");

        let left = Cube::from(0, 3, 0, 5, 0, 20, true, 0);
        let s = vec![([4, 0, 0], [5, 5, 20])];
        assert_correct_intersection(&c1, left, s, "Case 2");

        let right = Cube::from(3, 5, 0, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [2, 5, 20])];
        assert_correct_intersection(&c1, right, s, "Case 2.2");

        let bottom = Cube::from(0, 5, 0, 3, 0, 20, true, 0);
        let s = vec![([0, 4, 0], [5, 5, 20])];
        assert_correct_intersection(&c1, bottom, s, "Case 2.2");

        let top = Cube::from(0, 5, 3, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [5, 2, 20])];
        assert_correct_intersection(&c1, top, s, "Case 2.3");

        let middle = Cube::from(2, 4, 0, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [1, 5, 20]), ([5, 0, 0], [5, 5, 20])];
        assert_correct_intersection(&c1, middle, s, "Case 3");

        let center = Cube::from(0, 5, 1, 3, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [5, 0, 20]), ([0, 4, 0], [5, 5, 20])];
        assert_correct_intersection(&c1, center, s, "Case 3.2");

        let b_left = Cube::from(0, 3, 0, 3, 0, 20, true, 0);
        let s = vec![([4, 0, 0], [5, 5, 20]), ([0, 4, 0], [3, 5, 20])];
        assert_correct_intersection(&c1, b_left, s, "Case 4");

        let t_right = Cube::from(3, 5, 3, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [2, 5, 20]), ([3, 0, 0], [5, 2, 20])];
        assert_correct_intersection(&c1, t_right, s, "Case 4.1");

        let b_left = Cube::from(0, 3, 3, 5, 0, 20, true, 0);
        let s = vec![([4, 0, 0], [5, 5, 20]), ([0, 0, 0], [3, 2, 20])];
        assert_correct_intersection(&c1, b_left, s, "Case 4.2");

        let b_right = Cube::from(3, 5, 0, 3, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [2, 5, 20]), ([3, 4, 0], [5, 5, 20])];
        assert_correct_intersection(&c1, b_right, s, "Case 4.3");

        let r_mid = Cube::from(3, 5, 2, 3, 0, 20, true, 0);
        let s = vec![
            ([0, 0, 0], [2, 5, 20]),
            ([3, 0, 0], [5, 1, 20]),
            ([3, 4, 0], [5, 5, 20]),
        ];
        assert_correct_intersection(&c1, r_mid, s, "Case 5");

        let l_mid = Cube::from(0, 3, 2, 3, 0, 20, true, 0);
        let s = vec![
            ([4, 0, 0], [5, 5, 20]),
            ([0, 0, 0], [3, 1, 20]),
            ([0, 4, 0], [3, 5, 20]),
        ];
        assert_correct_intersection(&c1, l_mid, s, "Case 5.1");

        let b_center = Cube::from(2, 4, 0, 3, 0, 20, true, 0);
        let s = vec![
            ([0, 4, 0], [5, 5, 20]),
            ([0, 0, 0], [1, 3, 20]),
            ([5, 0, 0], [5, 3, 20]),
        ];
        assert_correct_intersection(&c1, b_center, s, "Case 5.2");

        let t_center = Cube::from(2, 4, 3, 5, 0, 20, true, 0);
        let s = vec![
            ([0, 0, 0], [5, 2, 20]),
            ([0, 3, 0], [1, 5, 20]),
            ([5, 3, 0], [5, 5, 20]),
        ];
        assert_correct_intersection(&c1, t_center, s, "Case 5.3");

        let c2 = Cube::from(2, 3, 2, 3, 0, 20, true, 0);
        let s = vec![
            ([0, 0, 0], [1, 5, 20]),
            ([4, 0, 0], [5, 5, 20]),
            ([2, 0, 0], [3, 1, 20]),
            ([2, 4, 0], [3, 5, 20]),
        ];
        assert_correct_intersection(&c1, c2, s, "Case 6");

        let c1 = Cube::from_m([-21710, 40705, 61220], [8680, 71234, 61458], true, 0);
        let c2 = Cube::from_m([-20031, 50563, 61220], [4022, 64702, 61458], true, 0);
        let s = vec![
            ([-21710, 40705, 61220], [-20032, 71234, 61458]),
            ([4023, 40705, 61220], [8680, 71234, 61458]),
            ([-20031, 40705, 61220], [4022, 50562, 61458]),
            ([-20031, 64703, 61220], [4022, 71234, 61458]),
        ];
        assert_correct_intersection(&c1, c2, s, "Test 1");
    }

    fn assert_correct_intersection(c1: &Cube, c2: Cube, s: Vec<([i64; 3], [i64; 3])>, name: &str) {
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "{}", name);
        assert_eq!(c1.one_amount, _vol(&s) + c2.one_amount, "{} volume", name);
    }
}
// -> Create 3D Cube that is sortable
// -> Create list of Cubes that is sorted by z-axis
// -> Watch result
