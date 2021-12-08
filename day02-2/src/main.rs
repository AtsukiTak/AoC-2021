use std::{fs::File, io::Read as _, str::FromStr};

fn main() {
    let input = read_input_file();
    let commands = parse_input(input.as_str());

    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;
    let mut aim = 0;

    for cmd in commands {
        match cmd {
            Command::Forward(n) => {
                horizontal_pos += n;
                vertical_pos += n * aim;
            }
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }

    dbg!(horizontal_pos * vertical_pos);
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug, Clone, Copy)]
struct InvalidCommand {}

impl FromStr for Command {
    type Err = InvalidCommand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("forward") {
            let n = s[8..].parse::<u32>().map_err(|_| InvalidCommand {})?;
            Ok(Command::Forward(n))
        } else if s.starts_with("down") {
            let n = s[5..].parse::<u32>().map_err(|_| InvalidCommand {})?;
            Ok(Command::Down(n))
        } else if s.starts_with("up") {
            let n = s[3..].parse::<u32>().map_err(|_| InvalidCommand {})?;
            Ok(Command::Up(n))
        } else {
            Err(InvalidCommand {})
        }
    }
}

fn read_input_file() -> String {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    content
}

fn parse_input(s: &str) -> Vec<Command> {
    s.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Command>().unwrap())
        .collect()
}
