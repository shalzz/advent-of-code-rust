use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut total_fuel: f32 = 0.0;
    for (index, line) in contents.lines().enumerate() {
        let mass: f32 = line
            .trim()
            .parse()
            .expect(&format!("Input should a integer at {}", index));

        let fuel: f32 = mass / 3.0;
        total_fuel += fuel.floor() - 2.0;
    }

    println!("Total Fuel required is {}", total_fuel);
    Ok(())
}
