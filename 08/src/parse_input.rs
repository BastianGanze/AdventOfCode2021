use std::str::Chars;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Segment {
    pub map: u8,
    pub count: u8,
}

pub type ParseOutput = Vec<(Vec<Segment>, Vec<Segment>)>;

pub fn parse_main() -> ParseOutput {
    parse("src/08.txt")
}

pub fn parse_test() -> ParseOutput {
    parse("src/test.txt")
}

fn parse(file_name: &str) -> ParseOutput {
    std::fs::read_to_string(file_name)
        .unwrap()
        .split("\n")
        .filter(|s| s.len() > 0)
        .map(|line| {
            let (seg_in, seg_out) = line.split_once("|").unwrap();
            (
                Vec::from_iter(
                    seg_in
                        .split(" ")
                        .filter(|s| s.len() > 0)
                        .map(|s| get_segment_map(s.chars())),
                ),
                Vec::from_iter(
                    seg_out
                        .split(" ")
                        .filter(|s| s.len() > 0)
                        .map(|s| get_segment_map(s.chars())),
                ),
            )
        })
        .collect()
}

fn get_segment_map(chars: Chars) -> Segment {
    let mut segment = Segment {
        map: 0b00000000,
        count: 0,
    };

    for char in chars {
        segment.count += 1;
        match char {
            'a' => segment.map |= 0b1000000,
            'b' => segment.map |= 0b0100000,
            'c' => segment.map |= 0b0010000,
            'd' => segment.map |= 0b0001000,
            'e' => segment.map |= 0b0000100,
            'f' => segment.map |= 0b0000010,
            'g' => segment.map |= 0b0000001,
            e => unreachable!("{}, Something other than what expected was matched.", e),
        }
    }
    segment
}
