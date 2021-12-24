use std::{fs::File, io::Read as _};

fn main() {
    let input = read_input_file("../input");
    let mut octs = parse_input(&input);
    let mut step = 0;

    loop {
        step += 1;
        let n_flashed = octs.next_step();
        if n_flashed == 100 {
            break;
        }
    }

    dbg!(step);
}

#[derive(Debug, Clone)]
struct Octopuses(Vec<Vec<Octopus>>);

#[derive(Debug, Copy, Clone)]
struct Octopus {
    energy: u32,
    is_flashed: bool,
}

#[derive(Debug, Copy, Clone)]
struct Loc {
    row: usize,
    col: usize,
}

impl Octopuses {
    fn next_step(&mut self) -> usize {
        let mut add_energy_locs = (0..10)
            .flat_map(|row| (0..10).map(move |col| Loc { row, col }))
            .collect::<Vec<_>>();

        let mut n_flashed = 0;

        loop {
            let flashed_locs = self.add_energy_to(add_energy_locs);
            if flashed_locs.is_empty() {
                break;
            }

            n_flashed += flashed_locs.len();

            add_energy_locs = flashed_locs
                .into_iter()
                .flat_map(|loc| loc.around())
                .collect();
        }

        for row in self.0.iter_mut() {
            for oct in row.iter_mut() {
                oct.reset_flash();
            }
        }

        n_flashed
    }

    /// returns newly flashed
    fn add_energy_to(&mut self, locs: Vec<Loc>) -> Vec<Loc> {
        let mut flashed = vec![];

        for loc in locs {
            let oct = self.get_mut(loc);
            if oct.add_energy() {
                flashed.push(loc);
            }
        }

        flashed
    }

    fn get_mut(&mut self, loc: Loc) -> &mut Octopus {
        let Loc { row, col } = loc;
        self.0.get_mut(row).unwrap().get_mut(col).unwrap()
    }
}

impl Octopus {
    fn add_energy(&mut self) -> bool {
        if self.is_flashed {
            // already flashed
            return false;
        }

        self.energy += 1;
        if self.energy == 10 {
            self.energy = 0;
            self.is_flashed = true;
            // flashed
            return true;
        }

        // unflashed
        return false;
    }

    fn reset_flash(&mut self) {
        self.is_flashed = false;
    }
}

impl Loc {
    fn around(self) -> Vec<Loc> {
        let min_row = if self.row == 0 { 0 } else { self.row - 1 };
        let min_col = if self.col == 0 { 0 } else { self.col - 1 };
        let max_row = if self.row == 9 { 9 } else { self.row + 1 };
        let max_col = if self.col == 9 { 9 } else { self.col + 1 };

        (min_row..=max_row)
            .flat_map(|row| (min_col..=max_col).map(move |col| (row, col)))
            .filter(|(row, col)| *row != self.row || *col != self.col)
            .map(|(row, col)| Loc { row, col })
            .collect()
    }
}

/*
 * ====
 * miscs
 * ====
 */
fn read_input_file(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    buf
}

fn parse_input(s: &str) -> Octopuses {
    let octs = s
        .trim_end()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|c| c.to_digit(10).unwrap())
                .map(|energy| Octopus {
                    energy,
                    is_flashed: false,
                })
                .collect::<Vec<Octopus>>()
        })
        .collect::<Vec<Vec<Octopus>>>();
    Octopuses(octs)
}

impl std::fmt::Display for Octopuses {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in self.0.iter() {
            for oct in row.iter() {
                write!(f, "{}", oct.energy)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let input = read_input_file("../input.sample");
        let mut octs = parse_input(&input);
        let mut step = 0;

        loop {
            step += 1;
            let n_flashed = octs.next_step();
            if n_flashed == 100 {
                break;
            }
        }

        assert_eq!(step, 195);
    }
}
