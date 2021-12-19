pub type ParseOutput = ((i32, i32), (i32, i32));

pub fn read_main() -> String {
    read_file("src/17.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let (xs, ys) = file.trim()[13..].split_once(',').unwrap();
    let (x1, x2) = xs.trim()[2..].split_once("..").unwrap();
    let (y1, y2) = ys.trim()[2..].split_once("..").unwrap();

    (
        (y1.parse().unwrap(), x1.parse().unwrap()),
        (y2.parse().unwrap(), x2.parse().unwrap()),
    )
}
