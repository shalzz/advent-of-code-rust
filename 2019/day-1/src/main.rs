use std::fs;
use std::iter;

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let cal_fuel = |mass: u32| mass.div_floor(3) - 2;

    let total_fuel: u32 = contents
        .lines()
        .map(|line| line.trim().parse::<u32>().expect("Input should an integer"))
        .map(cal_fuel) // calculate initial fuel required for the module
        .flat_map(|fuel| {
            // calculate fuel required for the fuel itself
            iter::successors(Some(fuel), |&new_fuel| {
                if new_fuel <= 0.0 {
                    None
                } else {
                    Some(cal_fuel(new_fuel))
                }
            })
            .filter(|&val| val > 0.0)
        })
        .sum();

    println!("Total Fuel required is {}", total_fuel);
    Ok(())
}
