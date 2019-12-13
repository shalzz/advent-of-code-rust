use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let init_state: Vec<usize> = contents
        .split(',')
        .enumerate()
        .map(|(index, val)| {
            val.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Input {} should an usize at {}", val, index))
        })
        .collect();

    // Brute force to reach the required state
    for noun in 0..100 {
        for verb in 0..100 {
            let mut state = init_state.clone();
            state[1] = noun;
            state[2] = verb;
            exec_intcode(&mut state);

            if state[0] == 19690720 {
                println!("final state:\n {:?}", state);
                println!("The found alarm code is: {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }

    Ok(())
}

fn exec_intcode(state: &mut Vec<usize>) {
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
}
