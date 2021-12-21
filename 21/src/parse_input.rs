pub type ParseOutput = (i32, i32);

pub fn read_main() -> String {
    read_file("src/21.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let (p1, p2) = file.split_once("Player 2 starting position:").unwrap();

    (
        p1.trim().chars().last().unwrap().to_digit(10).unwrap() as i32,
        p2.trim().parse().unwrap(),
    )
}
