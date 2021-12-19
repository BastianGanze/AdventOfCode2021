use crate::algebruh::{Point, Transformation};

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

fn get_rotations() -> [(usize, usize, usize, i32, i32, i32); 24] {
    let mut transformations = [(0, 0, 0, 0, 0, 0); 24];
    let swaps = [
        (0, 1, 2),
        (0, 2, 1),
        (1, 0, 2),
        (1, 2, 0),
        (2, 0, 1),
        (2, 1, 0),
    ];
    let orientations = [(-1, -1, -1), (1, -1, -1), (1, 1, -1), (1, 1, 1)];
    let mut i = 0;
    for s in swaps {
        for o in orientations {
            transformations[i] = (s.0, s.1, s.2, o.0, o.1, o.2);
            i += 1;
        }
    }

    transformations
}
