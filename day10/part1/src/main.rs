use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();

    let sum = input
        .trim_end()
        .split('\n')
        .filter_map(|line| find_corrupted(line))
        .map(error_score)
        .sum::<u32>();

    dbg!(sum);
}

fn find_corrupted(s: &str) -> Option<char> {
    let mut stack = Vec::<char>::new();

    let mut chars = s.chars();

    stack.push(chars.next().unwrap());

    for c in chars {
        if is_open(c) {
            stack.push(c);
        } else {
            let stack_top = *stack.last().unwrap();
            if c == counterpart(stack_top) {
                let _ = stack.pop();
            } else {
                return Some(c);
            }
        }
    }

    None
}

fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{' || c == '<'
}

fn counterpart(c: char) -> char {
    match c {
        '(' => ')',
        ')' => '(',
        '[' => ']',
        ']' => '[',
        '{' => '}',
        '}' => '{',
        '<' => '>',
        '>' => '<',
        _ => unreachable!(),
    }
}

fn error_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
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
