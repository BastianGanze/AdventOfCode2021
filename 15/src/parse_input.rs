use crate::grid::Grid;

pub type ParseOutput = Grid<u8>;

pub fn read_main() -> String {
    read_file("src/15.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let lines: Vec<&str> = file.trim().split("\n").filter(|s| s.len() > 0).collect();
    let mut output = Grid::new((lines.len() as usize, lines[0].len() as usize), 0);

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            output.set_field(y, x, char.to_digit(10).unwrap() as u8);
        }
    }

    output
}
