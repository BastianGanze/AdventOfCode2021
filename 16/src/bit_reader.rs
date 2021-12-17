use std::vec::IntoIter;

pub type BitReaderBufferType = u64;

#[derive(Debug, Clone)]
pub struct BitReader<const BUFFER_SIZE: u32> {
    chars: IntoIter<BitReaderBufferType>,
    read_buffer: BitReaderBufferType,
    bits_left_until_filling_buffer_is_necessary: u32,
}

impl<const T: u32> BitReader<T> {
    pub fn new(chars: IntoIter<BitReaderBufferType>) -> BitReader<T> {
        BitReader {
            chars,
            read_buffer: 0,
            bits_left_until_filling_buffer_is_necessary: 4,
        }
    }

    pub fn print_read_buffer(&self) {
        println!(
            "{:032b}|{:032b}",
            (self.read_buffer >> T) as u32,
            self.read_buffer as u32
        );
    }

    pub fn prefill(&mut self) {
        for _ in 0..7 {
            if let Some(char) = self.chars.next() {
                self.read_buffer |= char;
            }
            self.read_buffer = self.read_buffer << 4;
        }
        if let Some(char) = self.chars.next() {
            self.read_buffer |= char;
        }
    }

    pub fn read_bits(&mut self, mut bits_to_read: u32) -> BitReaderBufferType {
        let read_mask: BitReaderBufferType = (2 as BitReaderBufferType).pow(bits_to_read) - 1;

        if bits_to_read >= self.bits_left_until_filling_buffer_is_necessary {
            self.read_buffer = self.read_buffer << self.bits_left_until_filling_buffer_is_necessary;
            bits_to_read -= self.bits_left_until_filling_buffer_is_necessary;
            self.bits_left_until_filling_buffer_is_necessary = 4;
            if let Some(char) = self.chars.next() {
                self.read_buffer |= char;
            }
        }

        while bits_to_read >= 4 {
            self.read_buffer = self.read_buffer << 4;
            if let Some(char) = self.chars.next() {
                self.read_buffer |= char;
            }
            bits_to_read -= 4;
        }

        self.bits_left_until_filling_buffer_is_necessary -= bits_to_read;

        self.read_buffer = self.read_buffer << bits_to_read;

        ((read_mask << T) & self.read_buffer) >> T
    }
}

pub fn char_to_bit(c: char) -> BitReaderBufferType {
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
        'A' => 10,
        'B' => 11,
        'C' => 12,
        'D' => 13,
        'E' => 14,
        'F' => 15,
        _ => unreachable!(),
    }
}
