use crate::bit_reader::{char_to_bit, BitReader, BitReaderBufferType};

pub type ParseOutput = BitReader<32>;

pub fn read_main() -> String {
    read_file("src/16.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let chars: Vec<BitReaderBufferType> = file.trim().chars().map(|c| char_to_bit(c)).collect();
    return BitReader::new(chars.into_iter());
}
