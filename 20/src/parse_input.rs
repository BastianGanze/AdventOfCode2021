use std::collections::HashSet;

pub type Grid = HashSet<(i32, i32)>;
pub type EnhancementMap = [bool; 512];
pub type ParseOutput = (Grid, EnhancementMap, (i32, i32));

pub fn read_main() -> String {
    read_file("src/20.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut enhancements = [false; 512];
    let mut grid = HashSet::with_capacity(10000);

    let (enhancement, input_map) = file.split_once("\n\n").unwrap();
    for (i, char) in enhancement.trim().chars().enumerate() {
        if char == '#' {
            enhancements[i] = true;
        }
    }
    let lines: Vec<&str> = input_map.trim().split("\n").collect();
    let size = (lines.len() as i32, lines[0].len() as i32);
    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                grid.insert((y as i32, x as i32));
            }
        }
    }

    (grid, enhancements, size)
}
