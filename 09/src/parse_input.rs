pub type ParseOutput = Vec<Vec<i8>>;

pub fn parse_main() -> ParseOutput {
    parse("src/09.txt")
}

pub fn parse_test() -> ParseOutput {
    parse("src/test.txt")
}

fn parse(file_name: &str) -> ParseOutput {
    std::fs::read_to_string(file_name)
        .unwrap()
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|line| line.chars().map(|c| match_char(c)).collect())
        .collect()
}

fn match_char(c: char) -> i8 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        e => unreachable!("Tried to parse {}", e),
    }
}
