#[macro_export]

pub type ParseOutput = Vec<String>;

pub fn parse_main() -> ParseOutput {
    parse("src/??.txt")
}

pub fn parse_test() -> ParseOutput {
    parse("src/test.txt")
}

fn parse(file_name: &str) -> ParseOutput {
    std::fs::read_to_string(file_name)
        .unwrap()
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|s| s.to_string())
        .collect()
}
