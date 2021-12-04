enum Binary {
    ONE,
    ZERO,
}

fn main() {
    match std::fs::read_to_string("./src/04.txt") {
        Ok(text) => {
            let lines: Vec<&str> = text.split('\n').collect();
            part_1(lines.clone());
            part_2(lines);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn part_1(bit_lines: Vec<&str>) {
    todo!()
}

fn part_2(bit_lines: Vec<&str>) {
    todo!()
}

#[test]
fn it_works() {
    match std::fs::read_to_string("./src/test.txt") {
        Ok(text) => {
            let lines: Vec<&str> = text.split('\n').collect();
            assert_eq!(true, true);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
