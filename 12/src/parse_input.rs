use crate::cave_system::CaveSystem;

pub type ParseOutput = CaveSystem;

pub fn read_main() -> String {
    read_file("src/12.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut out = CaveSystem::new();

    for line in file.trim().split('\n') {
        let (start, end) = line.split_once('-').unwrap();
        out.connect_caves(&String::from(start), &String::from(end));
    }

    out
}
