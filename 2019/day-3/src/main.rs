use derive_more::*;
use std::collections::HashMap;
use std::fs;

// Simple struct to store row/col
#[derive(Copy, Clone, Eq, PartialEq, Hash, Add, AddAssign, Debug)]
struct Pos {
    row: i32,
    col: i32,
}

impl Pos {
    fn new(row: i32, col: i32) -> Pos {
        Pos { row, col }
    }
}

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let wires: Vec<Vec<_>> = contents
        .lines()
        .map(|line| line.split(',').collect())
        .collect();

    let mut grid: HashMap<Pos, (usize, usize)> = HashMap::new();

    let closest_point = wires
        .iter()
        .enumerate()
        .flat_map(|(index, wire)| {
            wire.iter()
                .flat_map(|mov| {
                    let dir = match mov.get(..1) {
                        Some("L") => Pos::new(0, -1),
                        Some("R") => Pos::new(0, 1),
                        Some("U") => Pos::new(1, 0),
                        Some("D") => Pos::new(-1, 0),
                        _ => panic!("Unable to parse input"),
                    };

                    let dest: i32 = mov.get(1..).unwrap().parse().expect("error parsing path");
                    (1..=dest).map(move |_| dir)
                })
                .scan(Pos::new(0, 0), |curr_pos, dir| {
                    *curr_pos += dir;
                    Some(*curr_pos)
                })
                .zip(std::iter::repeat(index))
                .enumerate() // count steps of a wire
        })
        .flat_map(|(steps, (pos, index))| {
            grid.insert(pos, (index, steps + 1))
                .map(|(wire_idx, wire_steps)| {
                    // don't update when a wire crosses itself
                    if wire_idx == index {
                        grid.insert(pos, (wire_idx, steps + 1));
                    }
                    (wire_idx, wire_steps)
                })
                // don't count a wire crossing itself
                .filter(|&(wire_idx, _)| wire_idx != index)
                .map(|(_, wire_steps)| wire_steps + steps + 1)
        })
        .min();

    println!("Got closest point {:?}", closest_point);
    Ok(())
}
