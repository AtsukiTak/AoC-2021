use std::{
    cmp::{max, min},
    fs::File,
    io::Read as _,
};

fn main() {
    let input = read_input_file();
    let lines = parse_input(input.as_str());

    let max_x = lines
        .iter()
        .map(|line| max(line.start.x, line.end.x))
        .max()
        .unwrap();
    let max_y = lines
        .iter()
        .map(|line| max(line.start.y, line.end.y))
        .max()
        .unwrap();

    let mut counter = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            let p = Point { x, y };
            if lines.iter().filter(|line| line.contains(p)).count() >= 2 {
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

impl Point {
    fn is_horizontally_between(&self, p1: Point, p2: Point) -> bool {
        (p1.x <= self.x && self.x <= p2.x) || (p2.x <= self.x && self.x <= p1.x)
    }

    fn is_vertically_between(&self, p1: Point, p2: Point) -> bool {
        (p1.y <= self.y && self.y <= p2.y) || (p2.y <= self.y && self.y <= p1.y)
    }

    fn is_diagonal(&self, p: Point) -> bool {
        abs(self.x, p.x) == abs(self.y, p.y)
    }
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

    fn is_diagonal(&self) -> bool {
        self.start.is_diagonal(self.end)
    }

    fn contains(&self, point: Point) -> bool {
        if self.is_horizontal() {
            if self.start.y != point.y {
                return false;
            }
            point.is_horizontally_between(self.start, self.end)
        } else if self.is_vertical() {
            if self.start.x != point.x {
                return false;
            }
            point.is_vertically_between(self.start, self.end)
        } else if self.is_diagonal() {
            if !self.start.is_diagonal(point) || !self.end.is_diagonal(point) {
                return false;
            }
            point.is_horizontally_between(self.start, self.end)
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
    let mut file = File::open("../input").unwrap();
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

fn abs(n: u32, m: u32) -> u32 {
    max(n, m) - min(n, m)
}
