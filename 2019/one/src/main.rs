use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let total_fuel: f32 = contents
        .lines()
        .map(|line| line.trim().parse::<f32>().expect("Input should an integer"))
        .map(|mass| (mass / 3.0).floor() - 2.0)
        .sum();

    println!("Total Fuel required is {}", total_fuel);
    Ok(())
}
