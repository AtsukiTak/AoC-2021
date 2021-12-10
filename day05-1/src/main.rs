use std::{cmp::max, fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let lines = parse_input(input.as_str());

    let vh_lines = lines
        .into_iter()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .collect::<Vec<Line>>();

    let max_x = vh_lines
        .iter()
        .map(|line| max(line.start.x, line.end.x))
        .max()
        .unwrap();
    let max_y = vh_lines
        .iter()
        .map(|line| max(line.start.y, line.end.y))
        .max()
        .unwrap();

    let mut counter = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point { x, y };
            if vh_lines.iter().filter(|line| line.contains(&p)).count() >= 2 {
                counter += 1;
            }
        }
    }

    dbg!(counter);
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn contains(&self, point: &Point) -> bool {
        if self.is_horizontal() {
            if self.start.y != point.y {
                return false;
            }
            (self.start.x <= point.x && point.x <= self.end.x)
                || (self.end.x <= point.x && point.x <= self.start.x)
        } else if self.is_vertical() {
            if self.start.x != point.x {
                return false;
            }
            (self.start.y <= point.y && point.y <= self.end.y)
                || (self.end.y <= point.y && point.y <= self.start.y)
        } else {
            unreachable!("{:?}", self);
        }
    }
}

/*
 * ====
 * miscs
 * ====
 */
fn read_input_file() -> String {
    let mut file = File::open("input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

fn parse_input(s: &str) -> Vec<Line> {
    s.split("\n")
        .filter(|s| !s.is_empty())
        .map(parse_line)
        .collect()
}

/// input example : "609,916 -> 60,367"
fn parse_line(s: &str) -> Line {
    let mut points = s.split(" ").filter(|s| *s != "->").map(parse_point);
    let start = points.next().unwrap();
    let end = points.next().unwrap();
    Line { start, end }
}

/// input example : "609,916"
fn parse_point(s: &str) -> Point {
    let mut iter = s.split(",").map(|s| s.parse::<u32>().unwrap());
    let x = iter.next().unwrap();
    let y = iter.next().unwrap();
    Point { x, y }
}
