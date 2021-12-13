use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();

    let mut window_iter = WindowIter::new(input);

    let first_window = window_iter.next().unwrap();

    let mut inc_counter = 0;
    let mut last_window = first_window;

    for window in window_iter {
        if window > last_window {
            inc_counter += 1;
        }
        last_window = window;
    }

    dbg!(inc_counter);
}

fn read_input_file() -> Vec<u32> {
    let mut file = File::open("../input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
        .split("\n")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect()
}

struct WindowIter {
    vec: Vec<u32>,
    idx: usize,
}

impl WindowIter {
    fn new(vec: Vec<u32>) -> Self {
        WindowIter { vec, idx: 0 }
    }
}

impl Iterator for WindowIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.vec.len() - 1 < self.idx + 2 {
            return None;
        }

        let sum = self.vec[self.idx..][..3].iter().sum();

        self.idx += 1;

        Some(sum)
    }
}
