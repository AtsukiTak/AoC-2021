use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file();

    let mut scores = input
        .trim_end()
        .split('\n')
        .filter_map(|line| find_incomplete(line))
        .map(error_score)
        .collect::<Vec<_>>();

    scores.sort_unstable();

    let middle = scores[scores.len() / 2];

    dbg!(middle);
}

fn find_incomplete(s: &str) -> Option<Vec<char>> {
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
                return None;
            }
        }
    }

    Some(stack.into_iter().rev().map(counterpart).collect())
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

fn error_score(chars: Vec<char>) -> u64 {
    fn char_error_score(c: char) -> u64 {
        match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => unreachable!(),
        }
    }

    chars
        .into_iter()
        .map(char_error_score)
        .fold(0, |sum, score| sum * 5 + score)
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
