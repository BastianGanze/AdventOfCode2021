use crate::grid::Grid;

pub type ParseOutput = Grid<10, 10>;

pub fn read_main() -> String {
    read_file("src/11.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut output = Grid::new();
    let lines = file.trim().split("\n").filter(|s| s.len() > 0);

    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            output.set_field(y as i16, x as i16, char.to_digit(10).unwrap() as u8);
        }
    }

    output
}
