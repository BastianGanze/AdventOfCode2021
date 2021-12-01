use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

type Lines = io::Lines<io::BufReader<File>>;

fn main() {
    match read_lines("./src/01.txt") {
        Ok(measurements) => {
            part_1(measurements);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

    match read_lines("./src/01.txt") {
        Ok(measurements) => {
            part_2(measurements);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn part_1(measurements: Lines) {
    let mut height_increases = 0;
    let mut previous_height: i32 = 99999999;

    for measurement_result in measurements {
        if let Ok(measurement) = measurement_result {
            let new_height = measurement.parse::<i32>().unwrap();

            if  new_height > previous_height {
                height_increases = height_increases + 1;
            }

            previous_height = new_height;
        }
    }
    println!("Height increases in part 1 have been {}", height_increases);
}

fn part_2(measurements: Lines) {
    let mut height_increases = 0;
    let mut measurements_queue = VecDeque::from([99999999, 99999999, 99999999]);
    let mut previous_height: i32 = measurements_queue.iter().sum();

    for measurement_result in measurements {
        if let Ok(measurement) = measurement_result {
            measurements_queue.pop_front();
            measurements_queue.push_back(measurement.parse::<i32>().unwrap());
            let new_height = measurements_queue.iter().sum();

            if  new_height > previous_height {
                height_increases = height_increases + 1;
            }

            previous_height = new_height;
        }
    }
    println!("Height increases in part 2 have been {}", height_increases);
}

fn read_lines<P>(filename: P) -> io::Result<Lines>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
