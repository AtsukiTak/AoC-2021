use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let mut crab_positions = parse_input(&input);
    crab_positions.sort_unstable();

    let ans = (0..crab_positions.len() as u32)
        .map(|pos| compute_cost(&crab_positions, pos as u32))
        .min();

    dbg!(ans);
}

fn compute_cost(crab_positions: &[u32], pos: u32) -> u32 {
    crab_positions
        .iter()
        .copied()
        .map(|p| abs(p, pos) * (abs(p, pos) + 1) / 2)
        .sum()
}

fn abs(n: u32, m: u32) -> u32 {
    if n > m {
        n - m
    } else {
        m - n
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

fn parse_input(s: &str) -> Vec<u32> {
    s.trim_end()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
