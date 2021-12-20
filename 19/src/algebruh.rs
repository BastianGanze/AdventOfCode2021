pub type Point = [i32; 3];
pub type Transformation = (usize, usize, usize, i32, i32, i32);

pub fn minus(p1: Point, p2: Point) -> Point {
    return [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]];
}

pub fn rotate_point(p: Point, t1: Transformation) -> Point {
    return [p[t1.0] * t1.3, p[t1.1] * t1.4, p[t1.2] * t1.5];
}

pub fn translate_point(p1: Point, t: Point) -> Point {
    return [p1[0] + t[0], p1[1] + t[1], p1[2] + t[2]];
}

pub fn point_distance(p1: Point, p2: Point) -> i32 {
    (p2[0] - p1[0]).pow(2) + (p2[1] - p1[1]).pow(2) + (p2[2] - p1[2]).pow(2)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use crate::algebruh::{point_distance, rotate_point};

    #[test]
    pub fn test_distance() {
        assert_eq!(
            point_distance([404, -588, -901], [686, 422, 578]),
            79524 + 1020100 + 2187441
        );
    }

    #[test]
    pub fn test_transformations() {
        assert_eq!(
            rotate_point([686, 422, 578], (0, 1, 2, 1, 1, 1)),
            [686, 422, 578]
        );
        assert_eq!(
            rotate_point([686, 422, 578], (1, 0, 2, 1, 1, 1)),
            [422, 686, 578]
        );
        assert_eq!(
            rotate_point([686, 422, 578], (0, 2, 1, 1, 1, 1)),
            [686, 578, 422]
        );
        assert_eq!(
            rotate_point([686, 422, 578], (0, 1, 2, -1, -1, -1)),
            [-686, -422, -578]
        );

        assert_eq!(
            rotate_point([686, 422, 578], (0, 2, 1, -1, -1, -1)),
            [-686, -578, -422]
        );

        assert_eq!(
            rotate_point([686, 422, 578], (0, 2, 1, -1, 1, -1)),
            [-686, 578, -422]
        );
    }
}
