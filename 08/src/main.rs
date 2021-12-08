#![feature(test)]

use crate::parse_input::{parse_main, ParseOutput, Segment};
use std::collections::HashMap;

pub mod parse_input;

type Solution = u32;

fn main() {
    let parse_output = parse_main();
    println!("Solution to part 1 is {}", part_1(&parse_output));
    println!("Solution to part 2 is {}", part_2(&parse_output));
}

fn part_1(parse_output: &ParseOutput) -> Solution {
    let mut count_numbers = 0;
    let segment_counts = [2, 4, 3, 7];
    for (_, seg_out) in parse_output {
        for segment in seg_out {
            if segment_counts.contains(&segment.count) {
                count_numbers += 1;
            }
        }
    }
    count_numbers
}

fn part_2(parse_output: &ParseOutput) -> Solution {
    let mut count_numbers: u32 = 0;
    for (seg_in, seg_out) in parse_output {
        let map = deduce_map(seg_in);

        for (digit, segment) in seg_out.iter().enumerate() {
            count_numbers += map[&segment.map] as u32 * 10_u32.pow(3_u32 - digit as u32);
        }
    }

    count_numbers
}

fn deduce_map(seg_in: &Vec<Segment>) -> HashMap<u8, u8> {
    let mut segment_maps = [0u8; 10];

    for seg in seg_in {
        match seg.count {
            2 => segment_maps[1] = seg.map,
            4 => segment_maps[4] = seg.map,
            3 => segment_maps[7] = seg.map,
            7 => segment_maps[8] = seg.map,
            _ => (),
        }
    }

    for seg in seg_in {
        match seg.count {
            6 => {
                // if 6 segments are set and only one of the segments of 1 is set, then this is the number 6
                if (seg.map & segment_maps[1]).count_ones() == 1 {
                    segment_maps[6] = seg.map;
                }
                // if 6 segments are set and all the segments of 4 are set, this is 9
                if (seg.map & segment_maps[4]).count_ones() == 4 {
                    segment_maps[9] = seg.map;
                }
            }
            _ => (),
        }
    }

    for seg in seg_in {
        match seg.count {
            6 => {
                // if 6 segments are set and its neither 6 nor 9 this must be 0
                if seg.map != segment_maps[6] && seg.map != segment_maps[9] {
                    segment_maps[0] = seg.map;
                }
            }
            5 => {
                // if 5 segments are set and all 3 segments of 7 are set this is 3
                if (seg.map & segment_maps[7]).count_ones() == 3 {
                    segment_maps[3] = seg.map;
                }
                // if 5 segments are set and there is exactly 1 segment difference to 6 then this is 5
                if (seg.map ^ segment_maps[6]).count_ones() == 1 {
                    segment_maps[5] = seg.map;
                }
            }
            _ => (),
        }
    }

    for seg in seg_in {
        match seg.count {
            5 => {
                // Two is the only one left
                if seg.map != segment_maps[3] && seg.map != segment_maps[5] {
                    segment_maps[2] = seg.map;
                }
            }
            _ => (),
        }
    }

    segment_maps
        .into_iter()
        .enumerate()
        .map(|(i, segment_map)| (segment_map, i as u8))
        .collect()

    /*
        abcdef <- 0,9  acdfg <- 2,5

        0 = abcdef (6 bits are set and its not 6 or 9)
        1 = bd (length is 2)
        2 = acdfg (length is 5 and it differs from 5 in exactly 2 bits)
        4 = bdeg (lengths is 4)
        3 = agcbd (length is 5 and it has both bits of 1 set)
        5 = abceg (length is 5 and only one bit of 6 is missing)
        6 = abcefg (6 bits are set and only one the bits of 1 is set)
        7 = dbc (length is 3)
        8 = dfebcag (lengths is 7)
        9 = abcdeg (6 bits are set and all the bits of 4 are set)

       0 has (2 of 1) & (3 of 4) & (3 of 7) & (6 of 8)
       1 = be
       4 = bedf
       7 = abe
       8 = abcdefg

       0 -> abef

    */
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::parse_input::{parse_main, parse_test};
    use test::{black_box, Bencher};

    #[test]
    pub fn test_part_1() {
        let parse_output = parse_test();
        assert_eq!(part_1(&parse_output), 26);
    }

    #[test]
    pub fn test_part_2() {
        let parse_output = parse_test();
        assert_eq!(part_2(&parse_output), 61229);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_main();
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(move || {
            assert_eq!(part_1(black_box(&parse_output)), 476);
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_main();
        b.iter(|| {
            assert_eq!(part_2(black_box(&parse_output)), 1011823);
        });
    }
}
