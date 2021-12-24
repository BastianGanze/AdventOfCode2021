use std::cmp::{max, min};

pub type Point3D = [i64; 3];

#[derive(Debug, Clone)]
pub struct Cube {
    pub min: Point3D,
    pub max: Point3D,
    pub sets_1: bool,
    pub volume: i64,
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
        Cube {
            min: [x0, y0, z0],
            max: [x1, y1, z1],
            sets_1,
            volume: (x1 - x0) * (y1 - y0) * (z1 - z0) as i64,
            order,
        }
    }

    pub fn vol(p1: Point3D, p2: Point3D) -> i64 {
        (p2[0] - p1[0]) * (p2[1] - p1[1]) * (p2[2] - p1[2]) as i64
    }

    pub fn from_m(min: Point3D, max: Point3D, sets_1: bool, order: i64) -> Cube {
        Cube {
            min,
            max,
            sets_1,
            volume: (max[0] - min[0]) * (max[1] - min[1]) * (max[2] - min[2]) as i64,
            order,
        }
    }

    pub fn intersection_rectangle_2d(&self, c: &Cube) -> Option<(Point3D, Point3D)> {
        if !(self.min[0] < c.max[0]
            && self.max[0] > c.min[0]
            && self.min[1] < c.max[1]
            && self.max[1] > c.min[1])
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

    pub fn contains_completely_2d(&self, c: &Cube) -> bool {
        self.min[0] <= c.min[0]
            && self.max[0] >= c.max[0]
            && self.min[1] <= c.min[1]
            && self.max[1] >= c.max[1]
    }

    pub fn split_by_containing_rectangle_2d(&self, c: &Cube) -> Vec<(Point3D, Point3D)> {
        let mut recs = Vec::new();
        let mut gap_mask: u8 = 0;

        if c.min[0] > self.min[0] {
            println!("left {:8b}", { gap_mask });
            gap_mask |= 8;
        };
        if c.max[0] < self.max[0] {
            println!("right {:8b}", { gap_mask });
            gap_mask |= 4;
        };
        if c.min[1] > self.min[1] {
            println!("bottom {:8b}", { gap_mask });
            gap_mask |= 2;
        };
        if c.max[1] < self.max[1] {
            println!("top {:8b}", { gap_mask });
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
                    [c.min[0], c.max[1], self.max[2]],
                ));
                recs.push((
                    [c.max[0], self.min[1], self.min[2]],
                    [self.max[0], c.max[1], self.max[2]],
                ));
            }
            0b1110 => {
                recs.push(self.bottom_rec(c));

                recs.push((
                    [self.min[0], c.min[1], self.min[2]],
                    [c.min[0], self.max[1], self.max[2]],
                ));
                recs.push((
                    [c.max[0], c.min[1], self.min[2]],
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
                    [c.max[0], c.min[1], self.max[2]],
                ));
                recs.push((
                    [c.min[0], c.max[1], self.min[2]],
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
            [c.min[0], self.max[1], self.max[2]],
        )
    }

    fn right_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [c.max[0], self.min[1], self.min[2]],
            [self.max[0], self.max[1], self.max[2]],
        )
    }

    fn bottom_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], self.min[1], self.min[2]],
            [self.max[0], c.min[1], self.max[2]],
        )
    }

    fn top_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], c.max[1], self.min[2]],
            [self.max[0], self.max[1], self.max[2]],
        )
    }

    fn top_left_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], c.max[1], self.min[2]],
            [c.max[0], self.max[1], self.max[2]],
        )
    }

    fn bottom_right_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [c.min[0], self.min[1], self.min[2]],
            [self.max[0], c.min[1], self.max[2]],
        )
    }

    fn top_right_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [c.min[0], c.max[1], self.min[2]],
            [self.max[0], self.max[1], self.max[2]],
        )
    }

    fn bottom_left_rec(&self, c: &Cube) -> ([i64; 3], [i64; 3]) {
        (
            [self.min[0], self.min[1], self.min[2]],
            [c.max[0], c.min[1], self.max[2]],
        )
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use crate::set_1s;
    use std::collections::HashSet;

    fn _vol(vec: &Vec<(Point3D, Point3D)>) -> i64 {
        vec.iter().fold(0, |a, c| a + Cube::vol(c.0, c.1))
    }

    fn _hash_vol(vec: &Vec<(Point3D, Point3D)>) -> i64 {
        let mut h: HashSet<(i64, i64, i64)> = HashSet::new();
        for (p1, p2) in vec.iter() {
            set_1s(&mut h, &Cube::from_m(p1.clone(), p2.clone(), true, -1));
        }
        h.len() as i64
    }

    #[test]
    pub fn split_by_2d() {
        let c1 = Cube::from(0, 5, 0, 5, 0, 20, true, 0);
        assert_eq!(c1.split_by_containing_rectangle_2d(&c1), vec![], "Case 1");
        let s = vec![([0, 0, 0], [5, 5, 20])];
        assert_eq!(_hash_vol(&s), _vol(&s), "Case 1 Correct hashset volume");

        let left = Cube::from(0, 3, 0, 5, 0, 20, true, 0);
        let s = vec![([4, 0, 0], [5, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&left), s, "Case 2");
        assert_eq!(c1.volume - left.volume, _vol(&s), "Case 2 Volume");
        assert_eq!(_hash_vol(&s), _vol(&s), "Case 2 Correct hashset volume");

        let right = Cube::from(3, 5, 0, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [3, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&right), s, "Case 2.21");
        assert_eq!(c1.volume - right.volume, _vol(&s), "Case 2.2 Volume");

        let bottom = Cube::from(0, 5, 0, 3, 0, 20, true, 0);
        let s = vec![([0, 3, 0], [5, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&bottom), s, "Case 2.2");
        assert_eq!(c1.volume - bottom.volume, _vol(&s), "Case 2.1 Volume");

        let top = Cube::from(0, 5, 3, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [5, 3, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&top), s, "Case 2.3");
        assert_eq!(c1.volume - top.volume, _vol(&s), "Case 2.3 Volume");

        let c2 = Cube::from(2, 4, 0, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [2, 5, 20]), ([4, 0, 0], [5, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 3");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 3 Volume");

        let c2 = Cube::from(0, 5, 1, 3, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [5, 1, 20]), ([0, 3, 0], [5, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 3.5");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 3.5 Volume");

        let c2 = Cube::from(0, 3, 0, 3, 0, 20, true, 0);
        let s = vec![([3, 0, 0], [5, 5, 20]), ([0, 3, 0], [3, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 4");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 4 Volume");

        let c2 = Cube::from(3, 5, 3, 5, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [3, 5, 20]), ([3, 0, 0], [5, 3, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 4.1");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 4.1 Volume");

        let c2 = Cube::from(0, 3, 3, 5, 0, 20, true, 0);
        let s = vec![([3, 0, 0], [5, 5, 20]), ([0, 0, 0], [3, 3, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 4.2");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 4.2 Volume");

        let c2 = Cube::from(3, 5, 0, 3, 0, 20, true, 0);
        let s = vec![([0, 0, 0], [3, 5, 20]), ([3, 3, 0], [5, 5, 20])];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 4.3");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 4.3 Volume");

        let c2 = Cube::from(3, 5, 2, 3, 0, 20, true, 0);
        let s = vec![
            ([0, 0, 0], [3, 5, 20]),
            ([3, 0, 0], [5, 2, 20]),
            ([3, 3, 0], [5, 5, 20]),
        ];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 5");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 5 Volume");

        let c2 = Cube::from(0, 3, 2, 3, 0, 20, true, 0);
        let s = vec![
            ([3, 0, 0], [5, 5, 20]),
            ([0, 0, 0], [3, 2, 20]),
            ([0, 3, 0], [3, 5, 20]),
        ];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 5.1");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 5.1 Volume");

        let c2 = Cube::from(2, 4, 0, 3, 0, 20, true, 0);
        let s = vec![
            ([0, 3, 0], [5, 5, 20]),
            ([0, 0, 0], [2, 3, 20]),
            ([4, 0, 0], [5, 3, 20]),
        ];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 5.2");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 5.2 Volume");

        let c2 = Cube::from(2, 4, 3, 5, 0, 20, true, 0);
        let s = vec![
            ([0, 0, 0], [5, 3, 20]),
            ([0, 3, 0], [2, 5, 20]),
            ([4, 3, 0], [5, 5, 20]),
        ];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 5.3");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 5.3 Volume");

        let c2 = Cube::from(2, 3, 2, 3, 0, 20, true, 0);
        let s = vec![
            ([0, 0, 0], [2, 5, 20]),
            ([3, 0, 0], [5, 5, 20]),
            ([2, 0, 0], [3, 2, 20]),
            ([2, 3, 0], [3, 5, 20]),
        ];
        assert_eq!(c1.split_by_containing_rectangle_2d(&c2), s, "Case 6");
        assert_eq!(c1.volume - c2.volume, _vol(&s), "Case 6 Volume");
    }

    #[test]
    pub fn contains_completely_2d() {
        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(1, 2, 2, 4, 0, 20, true, 0)),
            true,
            "Case 1"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(0, 2, 2, 4, 0, 20, true, 0)),
            true,
            "Case 2"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(0, 5, 2, 4, 0, 20, true, 0)),
            true,
            "Case 3"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(0, 5, 0, 4, 0, 20, true, 0)),
            true,
            "Case 4"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(0, 5, 0, 5, 0, 20, true, 0)),
            true,
            "Case 5"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(0, 6, 0, 5, 0, 20, true, 0)),
            false,
            "Case 5"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .contains_completely_2d(&Cube::from(5, 6, 0, 5, 0, 20, true, 0)),
            false,
            "Case 5"
        );
    }

    #[test]
    pub fn cube_intersection_xy() {
        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(-5, -3, -5, -1, 0, 20, true, 0)),
            None,
            "Case 1"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(5, 10, 0, 5, 0, 20, true, 0)),
            None,
            "Case 2"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(1, 3, 2, 4, 0, 20, true, 0)),
            Some(([1, 2, 0], [3, 4, 20])),
            "Case 3"
        );

        assert_eq!(
            Cube::from(1, 3, 2, 4, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(0, 5, 0, 5, 0, 20, true, 0)),
            Some(([1, 2, 0], [3, 4, 20])),
            "Case 5"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(3, 10, -5, 10, 0, 20, true, 0)),
            Some(([3, 0, 0], [5, 5, 20])),
            "Case 4"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(3, 10, 2, 4, 0, 20, true, 0)),
            Some(([3, 2, 0], [5, 4, 20])),
            "Case 5"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(3, 10, -10, 2, 0, 20, true, 0)),
            Some(([3, 0, 0], [5, 2, 20])),
            "Case 6"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(3, 10, 2, 10, 0, 20, true, 0)),
            Some(([3, 2, 0], [5, 5, 20])),
            "Case 6.5"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(0, 5, 2, 4, 0, 20, true, 0)),
            Some(([0, 2, 0], [5, 4, 20])),
            "Case 7"
        );

        assert_eq!(
            Cube::from(0, 5, 0, 5, 0, 20, true, 0)
                .intersection_rectangle_2d(&Cube::from(-5, 10, 2, 4, 0, 20, true, 0)),
            Some(([0, 2, 0], [5, 4, 20])),
            "Case 8"
        );
    }
}
// -> Create 3D Cube that is sortable
// -> Create list of Cubes that is sorted by z-axis
// -> Watch result
