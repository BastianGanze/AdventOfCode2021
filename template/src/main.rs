#![feature(test)]

type ParseOutput = String;
type Solution = u32;

fn main() {
    let parse_output = parse_input_file("");
    println!("Solution to part 1 is {}", part_1(parse_output.clone()));
    println!("Solution to part 2 is {}", part_2(parse_output.clone()));
}

fn parse_input_file(input_file: &str) -> ParseOutput {
    todo!()
}

fn part_1(parse_output: ParseOutput) -> Solution {
    todo!()
}

fn part_2(parse_output: ParseOutput) -> Solution {
    todo!()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[test]
    pub fn it_works() {
        assert_eq!(true, true);
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let _ = parse_input_file("");
        });
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let parse_output = parse_input_file("");
        b.iter(|| {
            part_1(parse_output.clone());
        });
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let parse_output = parse_input_file("");
        b.iter(|| {
            part_2(parse_output.clone());
        });
    }
}
