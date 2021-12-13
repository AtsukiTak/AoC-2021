use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let bitstrs = parse_input(input.as_str());

    let oxygen_rate_str = find_rate(bitstrs.clone(), derive_oxygen_criteria);
    let co2_rate_str = find_rate(bitstrs, derive_co2_criteria);

    let oxygen_rate = oxygen_rate_str.to_num();
    let co2_rate = co2_rate_str.to_num();

    dbg!(oxygen_rate * co2_rate);
}

fn read_input_file() -> String {
    let mut file = File::open("../input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

#[derive(Clone, Copy, Debug)]
struct BitStr<'a>(&'a str);

impl<'a> BitStr<'a> {
    fn bit(&self, idx: usize) -> bool {
        &self.0[idx..idx + 1] == "1"
    }

    fn to_num(&self) -> u32 {
        let mut n = 0;
        for i in 0..12 {
            if self.bit(i) {
                n += 2_u32.pow((11 - i) as u32);
            }
        }
        n
    }
}

fn parse_input<'a>(s: &'a str) -> Vec<BitStr<'a>> {
    s.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| BitStr(s))
        .collect()
}

fn find_rate<'a>(
    bitstrs: Vec<BitStr<'a>>,
    derive_criteria: fn(usize, usize) -> bool,
) -> BitStr<'a> {
    let mut remain_bitstrs = bitstrs;
    let mut depth = 0;

    loop {
        // derive criteria
        let num_1_bits = remain_bitstrs
            .iter()
            .filter(|bitstr| bitstr.bit(depth))
            .count();
        let criteria = derive_criteria(num_1_bits, remain_bitstrs.len());

        // update remain_bitstrs
        remain_bitstrs = remain_bitstrs
            .iter()
            .filter(|bitstr| bitstr.bit(depth) == criteria)
            .copied()
            .collect::<Vec<_>>();
        if remain_bitstrs.len() == 1 {
            return remain_bitstrs[0];
        }

        // update depth
        depth += 1;
    }
}

fn derive_oxygen_criteria(num_1_bits: usize, num_remain_bitstrs: usize) -> bool {
    num_1_bits >= num_remain_bitstrs / 2
}

fn derive_co2_criteria(num_1_bits: usize, num_remain_bitstrs: usize) -> bool {
    num_1_bits < num_remain_bitstrs / 2
}
