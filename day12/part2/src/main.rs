use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file("../input");
    let system = parse_input(&input);

    dbg!(solve(system));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cave<'a>(&'a str);

#[derive(Debug, Clone, Copy)]
struct Conn<'a>(Cave<'a>, Cave<'a>);

#[derive(Debug, Clone)]
struct System<'a>(Vec<Conn<'a>>);

#[derive(Debug, Clone)]
struct Path<'a> {
    paths: Vec<Cave<'a>>,
    twice_visited_small: bool,
}

impl<'a> Cave<'a> {
    fn start() -> Self {
        Cave("start")
    }

    fn is_start(&self) -> bool {
        self.0 == "start"
    }

    fn is_end(&self) -> bool {
        self.0 == "end"
    }

    fn is_small(&self) -> bool {
        !self.is_start() && !self.is_end() && self.0.chars().next().unwrap().is_ascii_lowercase()
    }
}

impl<'a> Path<'a> {
    fn new() -> Self {
        Path {
            paths: vec![],
            twice_visited_small: false,
        }
    }

    fn is_pushable(&self, cave: &Cave<'a>) -> bool {
        if cave.is_small() && self.contains(cave) && self.twice_visited_small {
            false
        } else {
            true
        }
    }

    fn push(&mut self, cave: Cave<'a>) {
        if cave.is_small() && self.contains(&cave) {
            assert!(!self.twice_visited_small);
            self.twice_visited_small = true;
        }
        self.paths.push(cave);
    }

    fn last(&self) -> &Cave<'a> {
        self.paths.last().unwrap()
    }

    fn contains(&self, cave: &Cave<'a>) -> bool {
        self.paths.contains(cave)
    }

    fn is_completed(&self) -> bool {
        if self.paths.is_empty() {
            return false;
        }
        self.paths.first().unwrap().is_start() && self.paths.last().unwrap().is_end()
    }
}

impl<'a> System<'a> {
    fn search_next<'b>(&'b self, path: &'b Path<'a>) -> impl Iterator<Item = Path<'a>> + 'b {
        self.find_connected(*path.last())
            .filter(|cave| path.is_pushable(cave))
            .map(|cave| {
                let mut new_path = path.clone();
                new_path.push(cave);
                new_path
            })
    }

    fn find_connected<'b>(&'b self, cave: Cave<'a>) -> impl Iterator<Item = Cave<'a>> + 'b {
        self.0
            .iter()
            .copied()
            .filter_map(move |conn| match conn {
                Conn(c1, c2) if c1 == cave => Some(c2),
                Conn(c1, c2) if c2 == cave => Some(c1),
                _ => None,
            })
            .filter(|cave| !cave.is_start())
    }
}

fn solve<'a>(system: System<'a>) -> usize {
    let mut start_path = Path::new();
    start_path.push(Cave::start());

    let mut searching_paths = vec![start_path];
    let mut n_completed = 0;

    loop {
        let mut next_searching_paths = vec![];

        for searching_path in searching_paths.iter() {
            for next_path in system.search_next(searching_path) {
                if next_path.is_completed() {
                    n_completed += 1;
                } else {
                    next_searching_paths.push(next_path);
                }
            }
        }

        if next_searching_paths.is_empty() {
            break;
        } else {
            searching_paths = next_searching_paths;
        }
    }

    n_completed
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

fn parse_input<'a>(s: &'a str) -> System<'a> {
    let conns = s.trim_end().split('\n').map(parse_line).collect();
    System(conns)
}

fn parse_line<'a>(s: &'a str) -> Conn<'a> {
    let mut caves = s.split('-').map(|s| Cave(s));
    let cave1 = caves.next().unwrap();
    let cave2 = caves.next().unwrap();

    Conn(cave1, cave2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = read_input_file("../input.sample");
        let system = parse_input(&input);

        assert_eq!(solve(system), 36);
    }
}
