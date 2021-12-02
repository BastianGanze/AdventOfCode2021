use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

type Lines = io::Lines<io::BufReader<File>>;

fn main() {
    match read_lines("./src/02.txt") {
        Ok(measurements) => {
            part_1(measurements);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    match read_lines("./src/02.txt") {
        Ok(measurements) => {
            part_2(measurements);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn part_1(commands: Lines) {
    let mut depth = 0;
    let mut horizontal = 0;

    for command_result in commands {
        if let Ok(command) = command_result {
            let mut command_instructions = command.split_whitespace();
            let instruction = command_instructions.next().unwrap();
            let instruction_amount: i32 = command_instructions.next().unwrap().parse::<i32>().unwrap();
            match instruction {
                "up" => {
                    depth = depth - instruction_amount;
                },
                "down" => {
                    depth = depth + instruction_amount;
                },
                "forward" => {
                    horizontal = horizontal + instruction_amount;
                },
                _ => {}
            }
        }
    }
    println!("Solution for one is {}", depth*horizontal);
}

fn part_2(commands: Lines) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim= 0;
    for command_result in commands {
        if let Ok(command) = command_result {
            let mut command_instructions = command.split_whitespace();
            let instruction = command_instructions.next().unwrap();
            let instruction_amount: i32 = command_instructions.next().unwrap().parse::<i32>().unwrap();
            match instruction {
                "up" => {
                    aim = aim - instruction_amount;
                },
                "down" => {
                    aim = aim + instruction_amount;
                },
                "forward" => {
                    horizontal = horizontal + instruction_amount;
                    depth = depth + (aim * instruction_amount);
                },
                _ => {}
            }
        }
    }
    println!("Solution for one is {}", depth*horizontal);
}
fn read_lines<P>(filename: P) -> io::Result<Lines>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
