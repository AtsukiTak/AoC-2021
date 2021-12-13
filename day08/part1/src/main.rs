use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let entries = parse_input(&input);

    let ans = entries
        .iter()
        .flat_map(|entry| &entry.output)
        .filter(|digit| digit.is_one() || digit.is_four() || digit.is_seven() || digit.is_eight())
        .count();

    dbg!(ans);
}

struct Entry<'a> {
    #[allow(dead_code)]
    all: [Digit<'a>; 10],
    output: [Digit<'a>; 4],
}

#[derive(Debug, Clone, Copy)]
struct Digit<'a> {
    signals: &'a str,
}

impl<'a> Digit<'a> {
    fn is_one(&self) -> bool {
        self.signals.len() == 2
    }

    fn is_four(&self) -> bool {
        self.signals.len() == 4
    }

    fn is_seven(&self) -> bool {
        self.signals.len() == 3
    }

    fn is_eight(&self) -> bool {
        self.signals.len() == 7
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

fn parse_input(s: &str) -> Vec<Entry> {
    s.split("\n")
        .filter(|s| !s.is_empty())
        .map(parse_entry)
        .collect()
}

fn parse_entry(s: &str) -> Entry {
    let mut chunks = s.split(" | ");

    let mut all = [Digit { signals: "" }; 10];
    for (i, digit) in chunks
        .next()
        .unwrap()
        .split(" ")
        .map(|s| Digit { signals: s })
        .enumerate()
    {
        all[i] = digit;
    }

    let mut output = [Digit { signals: "" }; 4];
    for (i, digit) in chunks
        .next()
        .unwrap()
        .split(" ")
        .map(|s| Digit { signals: s })
        .enumerate()
    {
        output[i] = digit;
    }

    Entry { all, output }
}
