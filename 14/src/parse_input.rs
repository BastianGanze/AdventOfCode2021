use std::collections::HashMap;

pub type Polymer = Vec<char>;
pub type Instructions = HashMap<(char, char), char>;
pub type BaseFrequencyMap = HashMap<char, u64>;

pub type ParseOutput = (Polymer, Instructions, BaseFrequencyMap);

pub fn read_main() -> String {
    read_file("src/14.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut polymer = Vec::new();
    let mut instructions = HashMap::new();
    let mut base_frequency = HashMap::new();

    let (start_poly, instruction_lines) = file.split_once("\n\n").unwrap();

    for char in start_poly.chars().into_iter() {
        if let None = base_frequency.get(&char) {
            base_frequency.insert(char.clone(), 0);
        }
        *base_frequency.get_mut(&char).unwrap() += 1;

        polymer.push(char);
    }

    for instruction_line in instruction_lines.split("\n") {
        let (from, to_str) = instruction_line.split_once(" -> ").unwrap();
        let to = to_str.chars().nth(0).unwrap();
        let mut from_as_chars = from.chars();
        let (from_0, from_1) = (from_as_chars.nth(0).unwrap(), from_as_chars.nth(0).unwrap());
        if let None = base_frequency.get(&from_0) {
            base_frequency.insert(from_0.clone(), 0);
        }
        if let None = base_frequency.get(&from_1) {
            base_frequency.insert(from_1.clone(), 0);
        }
        if let None = base_frequency.get(&to) {
            base_frequency.insert(to.clone(), 0);
        }
        instructions.insert((from_0, from_1), to);
    }

    (polymer, instructions, base_frequency)
}
