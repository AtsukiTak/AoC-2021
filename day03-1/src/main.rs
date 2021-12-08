use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();

    let mut bits_counter: [u32; 12] = [0; 12];
    let mut item_counter = 0;

    for bits_str in input.split("\n").filter(|s| !s.is_empty()) {
        for i in 0..12 {
            if &bits_str[i..i+1] == "1" {
                bits_counter[i] += 1;
            }
        }
        item_counter += 1;
    }

    assert!(item_counter == 1000);

    let gamma = {
        let mut gamma = 0;
        for i in 0..12 {
            if bits_counter[i] > item_counter / 2 {
                gamma += 2_u32.pow((11 - i) as u32);
            }
        }
        gamma
    };

    // gammaのbit反転
    let epsilon = (2_u32.pow(12) - 1) - gamma;

    dbg!(gamma * epsilon);
}

fn read_input_file() -> String {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}
