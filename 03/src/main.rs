enum Binary {
    ONE, ZERO
}

fn main() {
    match std::fs::read_to_string("./src/03.txt") {
        Ok(bits) => {
            let lines: Vec<&str> = bits.split('\n').collect();
            part_1(lines.clone());
            part_2(lines);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn part_1(bit_lines: Vec<&str>) {
    let (sigma, gamma) = get_significant_bit_values(bit_lines);
    println!("Solution to part 1 is {}", sigma * gamma);
}

fn part_2(bit_lines: Vec<&str>) {
    let oxygen_lines = bit_lines.clone();
    let co2_lines = bit_lines;

    let oxygen_binary = filter_lines_by_bit_position(
        oxygen_lines,
        | bin, position, gamma| {
            match bin {
                Binary::ONE => has_1_at_pos(gamma, position),
                Binary::ZERO => !has_1_at_pos(gamma, position),
            }
        });

    assert_eq!(oxygen_binary.len(), 1);

    let co2_binary = filter_lines_by_bit_position(co2_lines, | bin, position, gamma| {
        match bin {
            Binary::ONE => !has_1_at_pos(gamma, position),
            Binary::ZERO => has_1_at_pos(gamma, position),
        }
    });

    assert_eq!(co2_binary.len(), 1);

    let oxygen: u32 = u32::from_str_radix(oxygen_binary.get(0).unwrap(), 2).unwrap();
    let co2: u32 = u32::from_str_radix(co2_binary.get(0).unwrap(), 2).unwrap();

    println!("oxygen {:#012b}", oxygen);
    println!("co2 {:#012b}", co2);
    println!("Solution for second part is {}", oxygen * co2);
}

fn get_significant_bit_values(bit_lines: Vec<&str>) -> (u32, u32) {
    let bit_length = bit_lines.get(0).unwrap().len();
    let line_count = bit_lines.len();

    let mut count_ones_at_position: Vec<usize> = vec![0; bit_length];

    for bit_line in bit_lines {
        for (bit_i, char) in bit_line.chars().enumerate() {
            match char {
                '0' => {},
                '1' => {
                    count_ones_at_position[bit_i] = count_ones_at_position[bit_i] + 1
                },
                _ => {
                    unreachable!();
                }
            }
        }
    }

    let mut most_significant_bit: i32 = 0;
    let mut most_significant_bit_inverse: i32 = 0;
    for (position, one_count) in count_ones_at_position.iter().enumerate() {
        let bit_position = (bit_length - position - 1) as u32;
        if &(line_count - one_count) <= one_count {
            most_significant_bit = most_significant_bit + 2_i32.pow(bit_position);
        } else {
            most_significant_bit_inverse = most_significant_bit_inverse + 2_i32.pow(bit_position);
        }
    }

    (most_significant_bit as u32, most_significant_bit_inverse as u32)
}

fn filter_lines_by_bit_position<F>(bit_lines: Vec<&str>, filter_fun: F) -> Vec<&str> where F: Fn(Binary, usize, u32) -> bool {
    let bit_length = bit_lines.get(0).unwrap().len();

    let mut lines = bit_lines;

    for bit_position in 0..bit_length {
        if lines.len() == 1 {
            break;
        }
        let (gamma, _sigma) = get_significant_bit_values(lines.clone());
        lines = lines.into_iter().filter(|val| {
            match val.chars().nth(bit_position).unwrap() {
                '0' => {
                    filter_fun(Binary::ZERO, bit_length - bit_position - 1, gamma)
                },
                '1' => {
                    filter_fun(Binary::ONE, bit_length - bit_position - 1, gamma)
                },
                _ => {
                    unreachable!();
                }
            }
        }).collect();
    }

    assert_eq!(lines.len(), 1);

    lines
}

fn has_1_at_pos(input: u32, n: usize) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

#[test]
fn it_works() {

    match std::fs::read_to_string("./src/test.txt") {
        Ok(bits) => {
            let lines: Vec<&str> = bits.split('\n').collect();

            let oxygen_binary = filter_lines_by_bit_position(
                lines,
                | bin: Binary, position: usize, gamma| {
                    match bin {
                        Binary::ONE => has_1_at_pos(gamma, position),
                        Binary::ZERO => !has_1_at_pos(gamma, position),
                    }
                });

            assert_eq!(oxygen_binary.get(0).unwrap(), &"10111");
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
