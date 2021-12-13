pub type Point = (i32, i32);
pub type FoldList = Vec<i32>;

pub type ParseOutput = (Vec<Point>, FoldList, FoldList, Point);

pub fn read_main() -> String {
    read_file("src/13.txt")
}

pub fn read_test() -> String {
    read_file("src/test.txt")
}

pub fn read_file(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}

pub fn parse(file: &String) -> ParseOutput {
    let mut x_folds: FoldList = Vec::new();
    let mut y_folds: FoldList = Vec::new();
    let mut first_fold: Option<Point> = None;
    let mut points = Vec::new();

    let (points_to_parse, folds_to_parse) = file.split_once("\n\n").unwrap();

    for point in points_to_parse.split("\n") {
        let (x, y) = point.split_once(",").unwrap();
        points.push((y.parse().unwrap(), x.parse().unwrap()));
    }

    for fold in folds_to_parse.split("\n") {
        if let Some((_, x_fold)) = fold.split_once("x=") {
            x_folds.push(x_fold.parse().unwrap());
            if let None = first_fold {
                first_fold = Some((0, x_folds.last().unwrap().clone()));
            }
        }
        if let Some((_, y_fold)) = fold.split_once("y=") {
            y_folds.push(y_fold.parse().unwrap());
            if let None = first_fold {
                first_fold = Some((y_folds.last().unwrap().clone(), 0));
            }
        }
    }

    (points, y_folds, x_folds, first_fold.unwrap())
}
