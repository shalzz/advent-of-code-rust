use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let mut state: Vec<usize> = contents
        .split(',')
        .enumerate()
        .map(|(index, val)| {
            val.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Input {} should an usize at {}", val, index))
        })
        .collect();

    for index in (0..state.len()).step_by(4) {
        let opcode = state[index];
        let first = state[index + 1];
        let second = state[index + 2];
        let output = state[index + 3];

        match opcode {
            1 => state[output] = state[first] + state[second], // add opcode
            2 => state[output] = state[first] * state[second], // multiply opcode
            99 => break,                                       // halt
            _ => panic!("Something went wrong!"),
        }
    }

    println!("final state:\n {:?}", state);
    Ok(())
}
