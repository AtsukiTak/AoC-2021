use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let mut crab_positions = parse_input(&input);
    crab_positions.sort_unstable();

    let largest_pos = *crab_positions.last().unwrap();
    let mut prev_cost = crab_positions.iter().copied().sum();

    for pos in 1..largest_pos {
        let cost = compute_cost(&crab_positions, pos, prev_cost);
        if cost < prev_cost {
            prev_cost = cost;
        } else {
            break;
        }
    }

    let cost = prev_cost;

    dbg!(cost);
}

fn compute_cost(crab_positions: &[u32], pos: u32, prev_pos_cost: u32) -> u32 {
    let n_lower_crabs = num_lower_crabs(crab_positions, pos);
    let n_higher_crabs = crab_positions.len() as u32 - n_lower_crabs;

    // pos を1つずらすと、それより左側にいるcrabの数だけcostは
    // 増え、右側にいるcrabの数だけcostは減る。
    prev_pos_cost + n_lower_crabs - n_higher_crabs
}

fn num_lower_crabs(crab_positions: &[u32], pos: u32) -> u32 {
    crab_positions.iter().take_while(|p| **p < pos).count() as u32
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

fn parse_input(s: &str) -> Vec<u32> {
    s.trim_end()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
