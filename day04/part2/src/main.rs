use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let (numbers, mut boards) = parse_input(input.as_str());

    for number in numbers {
        for board in boards.iter_mut() {
            board.mark(number);
        }
        let new_boards = boards
            .iter()
            .copied()
            .filter(|board| !board.is_win())
            .collect::<Vec<_>>();
        if new_boards.is_empty() {
            let last_win = boards[0];
            let score = last_win.sum_unmarked() * number;
            dbg!(score);
            return;
        } else {
            boards = new_boards;
        }
    }
}

fn read_input_file() -> String {
    let mut file = File::open("../input").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

fn parse_input(s: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = s.split("\n").filter(|s| !s.is_empty());

    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = Vec::new();

    while let Some(row1) = lines.next() {
        let row2 = lines.next().unwrap();
        let row3 = lines.next().unwrap();
        let row4 = lines.next().unwrap();
        let row5 = lines.next().unwrap();
        let board = Board::from_row_strs([row1, row2, row3, row4, row5]);
        boards.push(board);
    }

    (numbers, boards)
}

#[derive(Debug, Clone, Copy)]
struct Board([[(u32, bool); 5]; 5]);

impl Board {
    fn mark(&mut self, n: u32) {
        for row_mut in self.0.as_mut_slice() {
            for cell_mut in row_mut.as_mut_slice() {
                if cell_mut.0 == n {
                    cell_mut.1 = true;
                    return;
                }
            }
        }
    }

    fn is_win(&self) -> bool {
        // rowが揃っているか
        for row in self.0 {
            if row.into_iter().all(|(_, marked)| marked) {
                return true;
            }
        }

        // colが揃っているか
        for col_idx in 0..5 {
            if self
                .0
                .into_iter()
                .map(|row| row[col_idx])
                .all(|(_, marked)| marked)
            {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> u32 {
        self.0
            .into_iter()
            .flatten()
            .filter(|(_, marked)| !*marked)
            .map(|(n, _)| n)
            .sum()
    }

    fn from_row_strs(row_strs: [&str; 5]) -> Self {
        let mut board = [[(0_u32, false); 5]; 5];

        for (row_idx, row_str) in row_strs.into_iter().enumerate() {
            for (col_idx, n_str) in row_str.split(" ").filter(|s| !s.is_empty()).enumerate() {
                let n = n_str.parse::<u32>().unwrap();
                board[row_idx][col_idx].0 = n;
            }
        }

        Board(board)
    }
}
