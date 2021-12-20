use crate::algebruh::{Point, Transformation};
use std::collections::HashSet;

pub type Transformations = [Transformation; 24];
pub type ParseOutput = (Vec<Vec<Point>>, Transformations);

pub fn read_main() -> String {
    read_file("src/19.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut scanners = Vec::new();

    let scanner_points_strings: Vec<&str> = file
        .split("---")
        .filter(|fr| !fr.contains("scanner") && !fr.contains("---") && fr.trim().len() > 0)
        .collect();

    for scanner_points_string in scanner_points_strings {
        scanners.push(
            scanner_points_string
                .split("\n")
                .into_iter()
                .filter(|l| l.len() > 0)
                .map(|points| {
                    let mut iter = points
                        .split(",")
                        .into_iter()
                        .map(|num| num.parse().unwrap());
                    [
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                        iter.next().unwrap(),
                    ]
                })
                .collect(),
        );
    }

    (scanners, get_rotations())
}

fn get_rotations() -> Transformations {
    /*
    let mut hashb: HashSet<Transformation> = HashSet::new();
    let mut t = (0, 1, 2, 1, 1, 1);
    for _ in 0..4 {
        t = rotate_around_x(t);
        hashb.insert(t);
        for _ in 0..4 {
            t = rotate_around_y(t);
            hashb.insert(t);
            for _ in 0..4 {
                t = rotate_around_z(t);
                hashb.insert(t);
            }
        }
    }
    fn rotate_around_z(t: Transformation) -> Transformation {
        (t.1, t.0, t.2, -t.4, t.3, t.5)
    }

    fn rotate_around_y(t: Transformation) -> Transformation {
        (t.2, t.1, t.0, -t.5, t.4, t.3)
    }

    fn rotate_around_x(t: Transformation) -> Transformation {
        (t.0, t.2, t.1, t.3, -t.5, t.4)
    }


    */
    [
        (2, 0, 1, -1, -1, 1),
        (2, 0, 1, -1, 1, -1),
        (1, 2, 0, -1, -1, 1),
        (2, 1, 0, -1, 1, 1),
        (0, 2, 1, -1, 1, 1),
        (0, 1, 2, -1, -1, 1),
        (2, 0, 1, 1, -1, -1),
        (1, 2, 0, 1, -1, -1),
        (1, 0, 2, -1, 1, 1),
        (1, 0, 2, 1, 1, -1),
        (0, 1, 2, -1, 1, -1),
        (0, 2, 1, 1, 1, -1),
        (1, 0, 2, -1, -1, -1),
        (1, 2, 0, -1, 1, -1),
        (0, 1, 2, 1, 1, 1),
        (0, 1, 2, 1, -1, -1),
        (2, 1, 0, 1, 1, -1),
        (2, 0, 1, 1, 1, 1),
        (1, 0, 2, 1, -1, 1),
        (0, 2, 1, -1, -1, -1),
        (2, 1, 0, 1, -1, 1),
        (2, 1, 0, 1, -1, -1),
        (1, 2, 0, 1, 1, 1),
        (0, 2, 1, 1, -1, 1),
    ]
}
