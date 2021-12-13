use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let height_map = parse_input(&input);

    let sum = height_map
        .iter()
        .filter(|(height, row, col)| height_map.is_lower_point(*height, *row, *col))
        .map(|(height, _, _)| height as u32 + 1)
        .sum::<u32>();

    dbg!(sum);
}

struct HeightMap {
    rows: Vec<Vec<u8>>,
}

impl HeightMap {
    fn is_lower_point(&self, height: u8, row: usize, col: usize) -> bool {
        self.adjacents(row, col)
            .into_iter()
            .flatten()
            .all(|adj| adj > height)
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.rows[row][col]
    }

    /// # Returns
    /// (up, down, left, right)
    fn adjacents(&self, row: usize, col: usize) -> [Option<u8>; 4] {
        let up = if row == 0 {
            None
        } else {
            Some(self.get(row - 1, col))
        };

        let down = if row == self.num_rows() - 1 {
            None
        } else {
            Some(self.get(row + 1, col))
        };

        let left = if col == 0 {
            None
        } else {
            Some(self.get(row, col - 1))
        };

        let right = if col == self.num_cols() - 1 {
            None
        } else {
            Some(self.get(row, col + 1))
        };

        [up, down, left, right]
    }

    fn num_rows(&self) -> usize {
        self.rows.len()
    }

    fn num_cols(&self) -> usize {
        self.rows[0].len()
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = (u8, usize, usize)> + 'a {
        self.rows.iter().enumerate().flat_map(|(row_idx, col)| {
            col.iter()
                .copied()
                .enumerate()
                .map(move |(col_idx, height)| (height, row_idx, col_idx))
        })
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
