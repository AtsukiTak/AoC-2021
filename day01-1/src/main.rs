use std::{fs::File, io::Read as _};

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut measurements = content.split("\n").filter_map(|s| s.parse::<u32>().ok());

    let first_measurement = measurements.next().unwrap();

    let mut inc_count = 0;
    let mut last_measurement = first_measurement;

    for measurement in measurements {
        if measurement > last_measurement {
            inc_count += 1;
        }
        last_measurement = measurement;
    }

    dbg!(inc_count);
}
