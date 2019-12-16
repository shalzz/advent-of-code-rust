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

    let mut grid: HashMap<Pos, usize> = HashMap::new();
    let mut cross_points: Vec<Pos> = Vec::default();

    for (index, wire) in wires.iter().enumerate() {
        let mut curr_pos = Pos::new(0, 0); // reset for every new wire.
        for mov in wire {
            let dir = match mov.get(..1) {
                Some("L") => Pos::new(0, -1),
                Some("R") => Pos::new(0, 1),
                Some("U") => Pos::new(1, 0),
                Some("D") => Pos::new(-1, 0),
                _ => panic!("Unable to parse input"),
            };

            let dest: i32 = mov.get(1..).unwrap().parse().expect("unable to parse path");
            for _ in 1..=dest {
                curr_pos += dir;
                match grid.insert(curr_pos, index) {
                    Some(wire_index) if wire_index != index => cross_points.push(curr_pos),
                    _ => continue,
                };
            }
        }
    }

    println!("Got cross points {:?}", cross_points);
    Ok(())
}
