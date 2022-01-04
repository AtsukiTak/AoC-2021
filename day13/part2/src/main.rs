use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file("../input");
    let (mut paper, folds) = parse_input(&input);

    for fold in folds {
        paper = paper.fold(fold);
    }

    print!("{}", paper);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Paper(Vec<Dot>);

impl Paper {
    fn fold(self, fold: Fold) -> Self {
        let mut new_dots = self
            .0
            .into_iter()
            .filter_map(|dot| dot.fold(fold))
            .collect::<Vec<_>>();
        new_dots.sort_unstable();
        new_dots.dedup();
        Paper(new_dots)
    }

    fn dots_len(&self) -> usize {
        self.0.len()
    }
}

use std::fmt;

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let max_x = self.0.iter().map(|dot| dot.x).max().unwrap();
        let max_y = self.0.iter().map(|dot| dot.y).max().unwrap();

        for row in 0..=max_y {
            for col in 0..max_x {
                let dot = Dot { x: col, y: row };
                if self.0.contains(&dot) {
                    write!(f, "x")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Dot {
    x: u32,
    y: u32,
}

impl Dot {
    fn fold(self, fold: Fold) -> Option<Self> {
        if fold.is_x {
            self.fold_x(fold.along)
        } else {
            self.fold_y(fold.along)
        }
    }

    fn fold_x(self, along: u32) -> Option<Self> {
        if self.x == along {
            None
        } else if self.x < along {
            Some(Dot {
                x: self.x,
                y: self.y,
            })
        } else {
            Some(Dot {
                x: along - (self.x - along),
                y: self.y,
            })
        }
    }

    fn fold_y(self, along: u32) -> Option<Self> {
        if self.y == along {
            None
        } else if self.y < along {
            Some(Dot {
                x: self.x,
                y: self.y,
            })
        } else {
            Some(Dot {
                x: self.x,
                y: along - (self.y - along),
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Fold {
    is_x: bool,
    along: u32,
}

/*
 * ====
 * miscs
 * ====
 */
fn read_input_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

fn parse_input(s: &str) -> (Paper, Vec<Fold>) {
    let dots = s
        .split('\n')
        .take_while(|s| !s.is_empty())
        .map(parse_dot_str)
        .collect::<Vec<_>>();

    let folds = s
        .trim_end()
        .split('\n')
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(parse_fold_str)
        .collect::<Vec<_>>();

    (Paper(dots), folds)
}

fn parse_dot_str(s: &str) -> Dot {
    let (x_s, y_s) = s.split_once(',').unwrap();
    let x = x_s.parse::<u32>().unwrap();
    let y = y_s.parse::<u32>().unwrap();
    Dot { x, y }
}

fn parse_fold_str(s: &str) -> Fold {
    let (axis_s, along_s) = s
        .strip_prefix("fold along ")
        .unwrap()
        .split_once('=')
        .unwrap();

    let is_x = axis_s == "x";
    let along = along_s.parse::<u32>().unwrap();

    Fold { is_x, along }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read_input_file("../input.sample");
        let (mut paper, folds) = parse_input(&input);

        for fold in folds {
            paper = paper.fold(fold);
        }

        assert_eq!(paper.dots_len(), 16);
    }
}
