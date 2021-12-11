use crate::grid::Grid;

pub type ParseOutput = Grid<10, 10>;

pub fn parse_main() -> ParseOutput {
    parse("src/11.txt")
}

pub fn parse_test() -> ParseOutput {
    parse("src/test.txt")
}

fn parse(file_name: &str) -> ParseOutput {
    let mut output = Grid::new();

    for (y, line) in std::fs::read_to_string(file_name)
        .unwrap()
        .split("\n")
        .filter(|s| s.len() > 0)
        .enumerate()
    {
        for (x, char) in line.chars().enumerate() {
            output.set_field(y as i16, x as i16, char.to_digit(10).unwrap() as u8);
        }
    }

    output
}
