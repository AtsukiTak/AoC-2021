use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let mut school = parse_input(input.as_str());

    for _ in 0..256 {
        school.tick();
    }

    dbg!(school.total());
}

#[derive(Debug, Clone)]
struct FishSchool {
    /// 各timer値のfishの数
    n_fishes: [u64; 9],
}

impl FishSchool {
    fn tick(&mut self) {
        let mut next = [0_u64; 9];

        // 単純にtimerが1減ったfish
        for i in 0..8 {
            next[i] = self.n_fishes[i + 1];
        }

        // 新しいfishを産んでtimerが6になったfish
        next[6] += self.n_fishes[0];

        // 新しく生まれたfish
        next[8] = self.n_fishes[0];

        self.n_fishes = next;
    }

    fn total(&self) -> u64 {
        self.n_fishes.into_iter().sum()
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

fn parse_input(s: &str) -> FishSchool {
    let mut n_fishes = [0_u64; 9];

    for timer in s.trim_end().split(",").map(|s| s.parse::<usize>().unwrap()) {
        n_fishes[timer] += 1;
    }

    FishSchool { n_fishes }
}
