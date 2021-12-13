use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let height_map = parse_input(&input);

    let num_height_9 = height_map.iter().filter(|loc| loc.height == 9).count();

    let mut best3 = Best3::new();

    let basins = height_map
        .iter()
        .filter(|loc| height_map.is_lower_point(*loc))
        .map(|lower_point| search_basin(&height_map, lower_point))
        .collect::<Vec<_>>();

    let total_size = basins.iter().map(|basin| basin.size()).sum::<usize>();
    debug_assert_eq!(height_map.iter().count(), num_height_9 + total_size);

    basins
        .iter()
        .for_each(|basin| best3.update_by(basin.size()));

    dbg!(best3.multiplied());
}

/// lower_pointから始めてBasinを検索する
fn search_basin(height_map: &HeightMap, lower_point: Location) -> Basin {
    let mut locs = vec![lower_point];

    loop {
        // 現在のlocsの周囲を検索
        let mut new_locs = locs
            .iter()
            .flat_map(|loc| height_map.adjacents(loc.pos))
            .filter(|adj| adj.height < 9 && !locs.contains(adj))
            .collect::<Vec<Location>>();
        new_locs.sort_unstable();
        new_locs.dedup();

        if new_locs.is_empty() {
            break;
        } else {
            locs.extend_from_slice(&new_locs);
        }
    }

    Basin { locs }
}

struct HeightMap {
    rows: Vec<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Location {
    pos: Position,
    height: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Clone)]
struct Basin {
    locs: Vec<Location>,
}

impl HeightMap {
    fn is_lower_point(&self, loc: Location) -> bool {
        self.adjacents(loc.pos).all(|adj| adj.height > loc.height)
    }

    fn get(&self, pos: Position) -> Location {
        let height = self.rows[pos.row][pos.col];
        Location { pos, height }
    }

    /// # Returns
    /// (up, down, left, right)
    fn adjacents(&self, pos: Position) -> impl Iterator<Item = Location> {
        let up = if pos.row == 0 {
            None
        } else {
            Some(self.get(Position::new(pos.row - 1, pos.col)))
        };

        let down = if pos.row == self.num_rows() - 1 {
            None
        } else {
            Some(self.get(Position::new(pos.row + 1, pos.col)))
        };

        let left = if pos.col == 0 {
            None
        } else {
            Some(self.get(Position::new(pos.row, pos.col - 1)))
        };

        let right = if pos.col == self.num_cols() - 1 {
            None
        } else {
            Some(self.get(Position::new(pos.row, pos.col + 1)))
        };

        [up, down, left, right].into_iter().flatten()
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_cols(&self) -> usize {
        self.rows[0].len()
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = Location> + 'a {
        self.rows.iter().enumerate().flat_map(|(row, cols)| {
            cols.iter()
                .copied()
                .enumerate()
                .map(move |(col, height)| Location {
                    height,
                    pos: Position { row, col },
                })
        })
    }
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

impl Basin {
    fn size(&self) -> usize {
        self.locs.len()
    }
}

/// 小さい方順に並ぶ（e.g. vec![1, 3, 8])
#[derive(Debug)]
struct Best3(Vec<usize>);

impl Best3 {
    fn new() -> Self {
        Best3(vec![0, 0, 0])
    }

    fn update_by(&mut self, n: usize) {
        let pos = match self.0.binary_search(&n) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        if pos == 0 {
            return;
        }
        self.0.insert(pos, n);
        self.0.remove(0);
    }

    fn multiplied(&self) -> usize {
        self.0[0] * self.0[1] * self.0[2]
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

fn parse_input(s: &str) -> HeightMap {
    let rows = s
        .trim_end()
        .split("\n")
        .map(|rows| {
            rows.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    HeightMap { rows }
}
