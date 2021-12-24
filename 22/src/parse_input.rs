use crate::Cube;

pub type ParseOutput = Vec<Cube>;

pub fn read_main() -> String {
    read_file("src/22.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_test_2() -> String {
    read_file("src/test2.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut cuboids = Vec::new();
    let cuboids_as_string = file.trim().split("\n");
    for cuboid_as_string in cuboids_as_string {
        let mut c = (cuboid_as_string.contains("on"), (0, 0), (0, 0), (0, 0));

        let just_coordinates = &cuboid_as_string[3..].trim();
        let coords: Vec<&str> = just_coordinates.split(",").collect();
        let x = coords[0][2..].split_once("..").unwrap();
        c.1 = (x.0.parse().unwrap(), x.1.parse().unwrap());
        let y = coords[1][2..].split_once("..").unwrap();
        c.2 = (y.0.parse().unwrap(), y.1.parse().unwrap());
        let z = coords[2][2..].split_once("..").unwrap();
        c.3 = (z.0.parse().unwrap(), z.1.parse().unwrap());
        cuboids.push(c)
    }

    cuboids
        .iter()
        .enumerate()
        .map(|(i, c)| {
            Cube::from(
                c.1 .0, c.1 .1, c.2 .0, c.2 .1, c.3 .0, c.3 .1, c.0, i as i64,
            )
        })
        .collect()
}
