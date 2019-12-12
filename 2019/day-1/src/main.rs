use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let cal_fuel = |mass: f32| (mass / 3.0).floor() - 2.0;

    let total_fuel: f32 = contents
        .lines()
        .map(|line| line.trim().parse::<f32>().expect("Input should an integer"))
        .map(cal_fuel) // calculate initial fuel required for the module
        .map(|fuel| {
            // calculate fuel required for the fuel itself
            let mut total = 0.0; // we account for the initial fuel in the while loop.
            let mut new_fuel = fuel;

            while new_fuel > 0.0 {
                total += new_fuel;
                new_fuel = cal_fuel(new_fuel);
            }
            total
        })
        .sum();

    println!("Total Fuel required is {}", total_fuel);
    Ok(())
}
