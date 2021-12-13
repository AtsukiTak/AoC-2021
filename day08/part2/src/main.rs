use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();
    let entries = parse_input(&input);

    let sum = entries
        .into_iter()
        .map(|mut entry| {
            entry.reveal();
            entry.output_to_num()
        })
        .sum::<u32>();

    dbg!(sum);
}

struct Entry<'a> {
    all: [Digit<'a>; 10],
    output: [Digit<'a>; 4],
}

impl<'a> Entry<'a> {
    fn reveal(&mut self) {
        let one = self.all.into_iter().find(|d| d.is_one()).unwrap();
        let four = self.all.into_iter().find(|d| d.is_four()).unwrap();
        let seven = self.all.into_iter().find(|d| d.is_seven()).unwrap();
        let eight = self.all.into_iter().find(|d| d.is_eight()).unwrap();

        let five_len_digits = self
            .all
            .into_iter()
            .filter(|d| d.signals.len() == 5)
            .collect::<Vec<_>>();

        let six_len_digits = self
            .all
            .into_iter()
            .filter(|d| d.signals.len() == 6)
            .collect::<Vec<_>>();

        let nine = six_len_digits
            .iter()
            .copied()
            .find(|d| d.contains(four))
            .unwrap();

        let zero = six_len_digits
            .iter()
            .copied()
            .find(|d| d.contains(seven) && !d.contains(four))
            .unwrap();

        let six = six_len_digits
            .iter()
            .copied()
            .find(|d| *d != nine && *d != zero)
            .unwrap();

        let five = five_len_digits
            .iter()
            .copied()
            .find(|d| six.contains(*d))
            .unwrap();

        let three = five_len_digits
            .iter()
            .copied()
            .find(|d| nine.contains(*d) && !six.contains(*d))
            .unwrap();

        let two = five_len_digits
            .iter()
            .copied()
            .find(|d| *d != five && *d != three)
            .unwrap();

        self.all = [zero, one, two, three, four, five, six, seven, eight, nine];
    }

    fn output_to_num(&self) -> u32 {
        self.output
            .into_iter()
            .map(|d| self.to_num(d))
            .enumerate()
            .map(|(i, n)| n * 10u32.pow((3 - i) as u32))
            .sum()
    }

    fn to_num(&self, digit: Digit<'a>) -> u32 {
        self.all
            .into_iter()
            .enumerate()
            .find(|(_, d)| *d == digit)
            .map(|(i, _)| i)
            .unwrap() as u32
    }
}

#[derive(Debug, Clone, Copy, Eq)]
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

    fn contains(&self, digit: Digit<'a>) -> bool {
        digit.signals.chars().all(|c| self.signals.contains(c))
    }
}

impl<'a> PartialEq for Digit<'a> {
    fn eq(&self, other: &Digit<'a>) -> bool {
        if self.signals.len() != other.signals.len() {
            return false;
        }
        self.contains(*other)
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
